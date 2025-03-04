use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const LDA_IMMEDIATE: u8 = 0xA9;
pub const LDA_ZEROPAGE: u8 = 0xA5;
pub const LDA_ZEROPAGEX: u8 = 0xB5;
pub const LDA_ABSOLUTE: u8 = 0xAD;
pub const LDA_ABSOLUTEX: u8 = 0xBD;
pub const LDA_ABSOLUTEY: u8 = 0xB9;
pub const LDA_INDIRECTX: u8 = 0xA1;
pub const LDA_INDIRECTY: u8 = 0xB1;

/// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionLDA {
    pub(crate) addr: u16,
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) page_crossed: bool,
}

impl OpCode for InstructionLDA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::LDA(Self {
            addr,
            page_crossed,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.register_a = cpu.mem_read(self.addr);
        cpu.update_zero_and_negative_flags(cpu.register_a);
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

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(LDA_IMMEDIATE, 0x05, 0x00, 0x80 ; "immediate")]
    #[test_case(LDA_ZEROPAGE, 0x12, 0x10, 0x14 ; "zero_page")]
    #[test_case(LDA_ZEROPAGEX, 0x0F, 0x0C, 0x11 ; "zero_page_x")]
    #[test_case(LDA_ABSOLUTE, 0x12, 0x10, 0x14 ; "absolute")]
    #[test_case(LDA_ABSOLUTEX, 0x0F, 0x0C, 0x11 ; "absolute_x")]
    #[test_case(LDA_ABSOLUTEY, 0x0E, 0x0B, 0x10 ; "absolute_y")]
    #[test_case(LDA_INDIRECTX, 0x13, 0x15, 0x17 ; "indirect_x")]
    #[test_case(LDA_INDIRECTY, 0x1C, 0x1E, 0x20 ; "indirect_y")]
    fn lda(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, load, BRK]);
        cpu.register_x = 0x03;
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x10, 0x00);
        cpu.mem_write(0x12, 0x05);
        cpu.mem_write(0x14, 0x80);
        cpu.mem_write_u16(0x16, 0x12);
        cpu.mem_write_u16(0x18, 0x10);
        cpu.mem_write_u16(0x1A, 0x14);
        cpu.mem_write_u16(0x1C, 0x0E);
        cpu.mem_write_u16(0x1E, 0x0C);
        cpu.mem_write_u16(0x20, 0x10);

        // Load
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Override
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0xFF;
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.run();
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.run();
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
