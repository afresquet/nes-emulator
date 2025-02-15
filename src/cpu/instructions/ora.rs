use crate::{Bus, Mem, OpCode, Rom, CPU};

use super::Instruction;

pub const ORA_IMMEDIATE: u8 = 0x09;
pub const ORA_ZEROPAGE: u8 = 0x05;
pub const ORA_ZEROPAGEX: u8 = 0x15;
pub const ORA_ABSOLUTE: u8 = 0x0D;
pub const ORA_ABSOLUTEX: u8 = 0x1D;
pub const ORA_ABSOLUTEY: u8 = 0x19;
pub const ORA_INDIRECTX: u8 = 0x01;
pub const ORA_INDIRECTY: u8 = 0x11;

/// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
#[derive(Debug)]
pub struct InstructionORA {
    addr: u16,
}

impl OpCode for InstructionORA {
    fn fetch(cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::ORA(Self {
            addr: cpu.get_operand_address(),
        })
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        let data = cpu.mem_read(self.addr);
        cpu.register_a |= data;
        cpu.update_zero_and_negative_flags(cpu.register_a);
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(ORA_IMMEDIATE, 0b0101, 0, 0b1000_0000 ; "immediate")]
    #[test_case(ORA_ZEROPAGE, 0x12, 0x10, 0x14 ; "zero_page")]
    #[test_case(ORA_ZEROPAGEX, 0x0F, 0x0C, 0x11 ; "zero_page_x")]
    #[test_case(ORA_ABSOLUTE, 0x12, 0x10, 0x14 ; "absolute")]
    #[test_case(ORA_ABSOLUTEX, 0x0F, 0x0C, 0x11 ; "absolute_x")]
    #[test_case(ORA_ABSOLUTEY, 0x0E, 0x0B, 0x10 ; "absolute_y")]
    #[test_case(ORA_INDIRECTX, 0x13, 0x15, 0x17 ; "indirect_x")]
    #[test_case(ORA_INDIRECTY, 0x1C, 0x1E, 0x20 ; "indirect_y")]
    fn ora(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[instruction, load, BRK]);
        cpu.register_a = 0b1010;
        cpu.register_x = 0x03;
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x10, 0);
        cpu.mem_write(0x12, 0b0101);
        cpu.mem_write(0x14, 0b1000_0000);
        cpu.mem_write_u16(0x16, 0x12);
        cpu.mem_write_u16(0x18, 0x10);
        cpu.mem_write_u16(0x1A, 0x14);
        cpu.mem_write_u16(0x1C, 0x0E);
        cpu.mem_write_u16(0x1E, 0x0C);
        cpu.mem_write_u16(0x20, 0x10);

        // OR
        cpu.run();
        assert_eq!(cpu.register_a, 0b1111);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.register_a = 0;
        cpu.run();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.register_a = 0b1010;
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1010);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
