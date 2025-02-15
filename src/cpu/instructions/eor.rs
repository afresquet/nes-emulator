use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const EOR_IMMEDIATE: u8 = 0x49;
pub const EOR_ZEROPAGE: u8 = 0x45;
pub const EOR_ZEROPAGEX: u8 = 0x55;
pub const EOR_ABSOLUTE: u8 = 0x4D;
pub const EOR_ABSOLUTEX: u8 = 0x5D;
pub const EOR_ABSOLUTEY: u8 = 0x59;
pub const EOR_INDIRECTX: u8 = 0x41;
pub const EOR_INDIRECTY: u8 = 0x51;

/// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
#[derive(Debug)]
pub struct InstructionEOR {
    addr: u16,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionEOR {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::EOR(Self {
            addr: cpu.get_operand_address(),
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        let data = cpu.mem_read(self.addr);
        cpu.register_a ^= data;
        cpu.update_zero_and_negative_flags(cpu.register_a);
        self.cycles(false)
    }

    fn cycles(&self, page_crossed: bool) -> u8 {
        match self.addressing_mode {
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 4,
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 4 + page_crossed as u8,
            AddressingMode::IndirectX => 6,
            AddressingMode::IndirectY => 5 + page_crossed as u8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(EOR_IMMEDIATE, 0b1000_0000, 0, 0b1000_1010 ; "immediate")]
    #[test_case(EOR_ZEROPAGE, 0x12, 0x10, 0x14 ; "zero_page")]
    #[test_case(EOR_ZEROPAGEX, 0x0F, 0x0D, 0x11 ; "zero_page_x")]
    #[test_case(EOR_ABSOLUTE, 0x16, 0x10, 0x18 ; "absolute")]
    #[test_case(EOR_ABSOLUTEX, 0x13, 0x0D, 0x15 ; "absolute_x")]
    #[test_case(EOR_ABSOLUTEY, 0x12, 0x0C, 0x14 ; "absolute_y")]
    #[test_case(EOR_INDIRECTX, 0x17, 0x19, 0x1B ; "indirect_x")]
    #[test_case(EOR_INDIRECTY, 0x20, 0x22, 0x24 ; "indirect_y")]
    fn eor(instruction: u8, eor: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, eor, BRK]);
        cpu.register_a = 0b1000_1010;
        cpu.register_x = 0x03;
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x10, 0);
        cpu.mem_write(0x12, 0b1000_0000);
        cpu.mem_write(0x14, 0b1000_1010);
        cpu.mem_write_u16(0x16, 0b1000_0000);
        cpu.mem_write_u16(0x18, 0b1000_1010);
        cpu.mem_write_u16(0x1A, 0x12);
        cpu.mem_write_u16(0x1C, 0x10);
        cpu.mem_write_u16(0x1E, 0x14);
        cpu.mem_write_u16(0x20, 0x0E);
        cpu.mem_write_u16(0x22, 0x0C);
        cpu.mem_write_u16(0x24, 0x10);

        // EOR
        cpu.run();
        assert_eq!(cpu.register_a, 0b1010);
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
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
