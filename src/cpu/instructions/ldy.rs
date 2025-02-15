use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const LDY_IMMEDIATE: u8 = 0xA0;
pub const LDY_ZEROPAGE: u8 = 0xA4;
pub const LDY_ZEROPAGEX: u8 = 0xB4;
pub const LDY_ABSOLUTE: u8 = 0xAC;
pub const LDY_ABSOLUTEX: u8 = 0xBC;

/// Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionLDY {
    addr: u16,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionLDY {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::LDY(Self {
            addr: cpu.get_operand_address(),
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.register_y = cpu.mem_read(self.addr);
        cpu.update_zero_and_negative_flags(cpu.register_y);
        self.cycles(false)
    }

    fn cycles(&self, page_crossed: bool) -> u8 {
        match self.addressing_mode {
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 4,
            AddressingMode::AbsoluteX => 4 + page_crossed as u8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(LDY_IMMEDIATE, 0x05, 0x00, 0x80 ; "immediate")]
    #[test_case(LDY_ZEROPAGE, 0x12, 0x10, 0x14 ; "zero_page")]
    #[test_case(LDY_ZEROPAGEX, 0x0F, 0x0C, 0x11 ; "zero_page_x")]
    #[test_case(LDY_ABSOLUTE, 0x12, 0x10, 0x14 ; "absolute")]
    #[test_case(LDY_ABSOLUTEX, 0x0F, 0x0C, 0x11 ; "absolute_x")]
    fn ldy(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, load, BRK]);
        cpu.register_x = 0x03;
        cpu.mem_write_u16(0x10, 0x00);
        cpu.mem_write(0x12, 0x05);
        cpu.mem_write(0x14, 0x80);
        cpu.mem_write_u16(0x16, 0x12);
        cpu.mem_write_u16(0x18, 0x10);
        cpu.mem_write_u16(0x1A, 0x14);

        // Load
        cpu.run();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Override
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_y = 0xFF;
        cpu.run();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
