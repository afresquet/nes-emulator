use crate::{AddressingMode, Instruction, OpCode, CPU};

use super::{InstructionADC, InstructionROR};

pub const RRA_ZEROPAGE: u8 = 0x67;
pub const RRA_ZEROPAGEX: u8 = 0x77;
pub const RRA_ABSOLUTE: u8 = 0x6F;
pub const RRA_ABSOLUTEX: u8 = 0x7F;
pub const RRA_ABSOLUTEY: u8 = 0x7B;
pub const RRA_INDIRECTX: u8 = 0x63;
pub const RRA_INDIRECTY: u8 = 0x73;

/// Perfoms ROR and ADC.
#[derive(Debug)]
pub struct InstructionRRA {
    ror: InstructionROR,
    adc: InstructionADC,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionRRA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::RRA(Self {
            ror: InstructionROR {
                addr: Some(addr),
                addressing_mode,
            },
            adc: InstructionADC {
                addr,
                addressing_mode,
                page_crossed,
            },
            addressing_mode,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.ror.execute(cpu);
        self.adc.execute(cpu);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::ZeroPage => 5,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 6,
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 7,
            AddressingMode::IndirectX | AddressingMode::IndirectY => 8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(RRA_ZEROPAGE ; "zero_page")]
    #[test_case(RRA_ZEROPAGEX ; "zero_page_x")]
    #[test_case(RRA_ABSOLUTE ; "absolute")]
    #[test_case(RRA_ABSOLUTEX ; "absolute_x")]
    #[test_case(RRA_ABSOLUTEY ; "absolute_y")]
    #[test_case(RRA_INDIRECTX ; "indirect_x")]
    #[test_case(RRA_INDIRECTY ; "indirect_y")]
    fn rra(instruction: u8) {
        // Just test that it runs, ASL and ORA are already tested.
        CPU::new_test(&[instruction]).run();
    }
}
