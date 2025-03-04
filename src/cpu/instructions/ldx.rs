use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const LDX_IMMEDIATE: u8 = 0xA2;
pub const LDX_ZEROPAGE: u8 = 0xA6;
pub const LDX_ZEROPAGEY: u8 = 0xB6;
pub const LDX_ABSOLUTE: u8 = 0xAE;
pub const LDX_ABSOLUTEY: u8 = 0xBE;

/// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionLDX {
    pub(crate) addr: u16,
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) page_crossed: bool,
}

impl OpCode for InstructionLDX {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::LDX(Self {
            addr,
            page_crossed,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.register_x = cpu.mem_read(self.addr);
        cpu.update_zero_and_negative_flags(cpu.register_x);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageY | AddressingMode::Absolute => 4,
            AddressingMode::AbsoluteY => 4 + self.page_crossed as u8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(LDX_IMMEDIATE, 0x05, 0x00, 0x80 ; "immediate")]
    #[test_case(LDX_ZEROPAGE, 0x12, 0x10, 0x14 ; "zero_page")]
    #[test_case(LDX_ZEROPAGEY, 0x0E, 0x0B, 0x10 ; "zero_page_y")]
    #[test_case(LDX_ABSOLUTE, 0x12, 0x10, 0x14 ; "absolute")]
    #[test_case(LDX_ABSOLUTEY, 0x0E, 0x0B, 0x10 ; "absolute_y")]
    fn ldx(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, load, BRK]);
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x10, 0x00);
        cpu.mem_write(0x12, 0x05);
        cpu.mem_write(0x14, 0x80);
        cpu.mem_write_u16(0x16, 0x0E);
        cpu.mem_write_u16(0x18, 0x0C);
        cpu.mem_write_u16(0x2A, 0x10);

        // Load
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Override
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_x = 0xFF;
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
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
