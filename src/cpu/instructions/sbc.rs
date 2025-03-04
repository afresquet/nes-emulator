use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const SBC_IMMEDIATE: u8 = 0xE9;
pub const SBC_IMMEDIATE2: u8 = 0xEB;
pub const SBC_ZEROPAGE: u8 = 0xE5;
pub const SBC_ZEROPAGEX: u8 = 0xF5;
pub const SBC_ABSOLUTE: u8 = 0xED;
pub const SBC_ABSOLUTEX: u8 = 0xFD;
pub const SBC_ABSOLUTEY: u8 = 0xF9;
pub const SBC_INDIRECTX: u8 = 0xE1;
pub const SBC_INDIRECTY: u8 = 0xF1;

/// This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit.
/// If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
#[derive(Debug)]
pub struct InstructionSBC {
    pub(crate) addr: u16,
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) page_crossed: bool,
}

impl OpCode for InstructionSBC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::SBC(Self {
            addr,
            page_crossed,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let value = cpu.mem_read(self.addr);
        cpu.sum((value as i8).wrapping_neg().wrapping_sub(1) as u8);
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
    use crate::{instructions::BRK, Status};
    use test_case::test_case;

    use super::*;

    #[test_case(&[SBC_IMMEDIATE, 0x40, BRK] ; "immediate")]
    #[test_case(&[SBC_IMMEDIATE2, 0x40, BRK] ; "immediate_2")]
    #[test_case(&[SBC_ZEROPAGE, 0x10, BRK] ; "zero_page")]
    #[test_case(&[SBC_ZEROPAGEX, 0x00, BRK] ; "zero_page_x")]
    #[test_case(&[SBC_ABSOLUTE, 0x1A, BRK] ; "absolute")]
    #[test_case(&[SBC_ABSOLUTEX, 0x00, BRK] ; "absolute_x")]
    #[test_case(&[SBC_ABSOLUTEY, 0x00, BRK] ; "absolute_y")]
    #[test_case(&[SBC_INDIRECTX, 0x0A, BRK] ; "indirect_x")]
    #[test_case(&[SBC_INDIRECTY, 0x4A, BRK] ; "indirect_y")]
    fn sbc(program: &[u8]) {
        // Setup
        let mut cpu = CPU::new_test(program);
        cpu.register_x = 0x10;
        cpu.register_y = 0x1A;
        cpu.mem_write(0x10, 0x40);
        cpu.mem_write_u16(0x1A, 0x40);
        cpu.mem_write(0x40, 0x40);
        cpu.mem_write_u16(0x4A, 0x26);

        // From 0
        cpu.run();
        assert_eq!(cpu.register_a, 0xBF);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // From existing value
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x42;
        cpu.run();
        assert_eq!(cpu.register_a, 1);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // Carry Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x41;
        cpu.run();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // Overflow Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x20;
        cpu.run();
        assert_eq!(cpu.register_a, 0xDF);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));
    }
}
