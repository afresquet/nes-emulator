use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const CMP_IMMEDIATE: u8 = 0xC9;
pub const CMP_ZEROPAGE: u8 = 0xC5;
pub const CMP_ZEROPAGEX: u8 = 0xD5;
pub const CMP_ABSOLUTE: u8 = 0xCD;
pub const CMP_ABSOLUTEX: u8 = 0xDD;
pub const CMP_ABSOLUTEY: u8 = 0xD9;
pub const CMP_INDIRECTX: u8 = 0xC1;
pub const CMP_INDIRECTY: u8 = 0xD1;

/// This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.
#[derive(Debug)]
pub struct InstructionCMP {
    addr: u16,
    addressing_mode: AddressingMode,
    page_crossed: bool,
}

impl OpCode for InstructionCMP {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::CMP(Self {
            addr,
            page_crossed,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let data = cpu.mem_read(self.addr);
        cpu.compare(data, cpu.register_a);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 4,
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 4 + self.page_crossed as u8,
            AddressingMode::IndirectX => 6,
            AddressingMode::IndirectY => 5 + self.page_crossed as u8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Mem, Status};

    use super::*;

    #[test_case(CMP_IMMEDIATE, 0x00, 0x10, 0x11 ; "immediate")]
    #[test_case(CMP_ZEROPAGE, 0x10, 0x12, 0x14 ; "zero_page")]
    #[test_case(CMP_ZEROPAGEX, 0x0B, 0x0D, 0x0F ; "zero_page_x")]
    #[test_case(CMP_ABSOLUTE, 0x10, 0x12, 0x14 ; "absolute")]
    #[test_case(CMP_ABSOLUTEX, 0x0B, 0x0D, 0x0F ; "absolute_x")]
    #[test_case(CMP_ABSOLUTEY, 0x0A, 0x0C, 0x0E ; "absolute_y")]
    #[test_case(CMP_INDIRECTX, 0x1B, 0x1D, 0x1F ; "indirect_x")]
    #[test_case(CMP_INDIRECTY, 0x30, 0x32, 0x34 ; "indirect_y")]
    fn cmp(instruction: u8, carry: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, carry, BRK]);
        cpu.register_a = 0x10;
        cpu.register_x = 0x05;
        cpu.register_y = 0x06;
        cpu.mem_write(0x10, 0x00);
        cpu.mem_write(0x12, 0x10);
        cpu.mem_write(0x14, 0x11);
        cpu.mem_write_u16(0x20, 0x10);
        cpu.mem_write_u16(0x22, 0x12);
        cpu.mem_write_u16(0x24, 0x14);
        cpu.mem_write_u16(0x30, 0x0A);
        cpu.mem_write_u16(0x32, 0x0C);
        cpu.mem_write_u16(0x34, 0x0E);

        // Carry Flag
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
