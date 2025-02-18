use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const INC_ZEROPAGE: u8 = 0xE6;
pub const INC_ZEROPAGEX: u8 = 0xF6;
pub const INC_ABSOLUTE: u8 = 0xEE;
pub const INC_ABSOLUTEX: u8 = 0xFE;

/// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionINC {
    pub(crate) addr: u16,
    pub(crate) addressing_mode: AddressingMode,
}

impl OpCode for InstructionINC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::INC(Self {
            addr: cpu.get_operand_address().0,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let result = cpu.mem_read(self.addr).wrapping_add(1);
        cpu.mem_write(self.addr, result);
        cpu.update_zero_and_negative_flags(result);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::ZeroPage => 5,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 6,
            AddressingMode::AbsoluteX => 7,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(INC_ZEROPAGE, 0x10, 0x10 ; "zero_page")]
    #[test_case(INC_ZEROPAGEX, 0x00, 0x10 ; "zero_page_x")]
    #[test_case(INC_ABSOLUTE, 0x10, 0x10 ; "absolute")]
    #[test_case(INC_ABSOLUTEX, 0x00, 0x10 ; "absolute_x")]
    fn inc(instruction: u8, addr: u8, target: u16) {
        let mut cpu = CPU::new_test(&[instruction, addr, BRK]);
        cpu.register_x = 0x10;

        // Increments
        cpu.run();
        assert_eq!(cpu.mem_read(target), 1);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Overflow
        cpu.swap_test_rom(&[instruction, addr, BRK]);
        cpu.reset_status();
        cpu.mem_write(target, u8::MAX);
        cpu.run();
        assert_eq!(cpu.mem_read(target), 0);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, addr, BRK]);
        cpu.reset_status();
        cpu.mem_write(target, u8::MAX);
        cpu.run();
        assert_eq!(cpu.mem_read(target), 0);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, addr, BRK]);
        cpu.reset_status();
        cpu.mem_write(target, u8::MAX - 1);
        cpu.run();
        assert_eq!(cpu.mem_read(target), u8::MAX);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
