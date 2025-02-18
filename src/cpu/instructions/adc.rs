use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZEROPAGE: u8 = 0x65;
pub const ADC_ZEROPAGEX: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTEX: u8 = 0x7D;
pub const ADC_ABSOLUTEY: u8 = 0x79;
pub const ADC_INDIRECTX: u8 = 0x61;
pub const ADC_INDIRECTY: u8 = 0x71;

/// This instruction adds the contents of a memory location to the accumulator together with the carry bit.
/// If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
#[derive(Debug)]
pub struct InstructionADC {
    pub(crate) addr: u16,
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) page_crossed: bool,
}

impl OpCode for InstructionADC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::ADC(Self {
            addr,
            page_crossed,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let value = cpu.mem_read(self.addr);
        cpu.sum(value);
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

    #[test_case(ADC_IMMEDIATE, 0x40 ; "immediate")]
    #[test_case(ADC_ZEROPAGE, 0x10 ; "zero_page")]
    #[test_case(ADC_ZEROPAGEX, 0x00 ; "zero_page_x")]
    #[test_case(ADC_ABSOLUTE, 0x1A ; "absolute")]
    #[test_case(ADC_ABSOLUTEX, 0x00 ; "absolute_x")]
    #[test_case(ADC_ABSOLUTEY, 0x00 ; "absolute_y")]
    #[test_case(ADC_INDIRECTX, 0x0A ; "indirect_x")]
    #[test_case(ADC_INDIRECTY, 0x4A ; "indirect_y")]
    fn adc(instruction: u8, addr: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, addr, BRK]);
        cpu.register_x = 0x10;
        cpu.register_y = 0x1A;
        cpu.mem_write(0x10, 0x40);
        cpu.mem_write_u16(0x1A, 0x40);
        cpu.mem_write(0x40, 0x40);
        cpu.mem_write_u16(0x4A, 0x26);

        // From 0
        cpu.run();
        assert_eq!(cpu.register_a, 0x40);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // From existing value
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x01;
        cpu.run();
        assert_eq!(cpu.register_a, 0x41);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // u8::MAX
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0xBF;
        cpu.run();
        assert_eq!(cpu.register_a, 0xFF);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // Carry Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0xC0;
        cpu.run();
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // Overflow Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x40;
        cpu.run();
        assert_eq!(cpu.register_a, 0x80);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::OVERFLOW));
    }
}
