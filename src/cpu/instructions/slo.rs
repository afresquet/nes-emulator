use crate::{AddressingMode, Instruction, OpCode, CPU};

use super::{InstructionASL, InstructionORA};

pub const SLO_ZEROPAGE: u8 = 0x07;
pub const SLO_ZEROPAGEX: u8 = 0x17;
pub const SLO_ABSOLUTE: u8 = 0x0F;
pub const SLO_ABSOLUTEX: u8 = 0x1F;
pub const SLO_ABSOLUTEY: u8 = 0x1B;
pub const SLO_INDIRECTX: u8 = 0x03;
pub const SLO_INDIRECTY: u8 = 0x13;

/// Perfoms ASL and ORA.
#[derive(Debug)]
pub struct InstructionSLO {
    asl: InstructionASL,
    ora: InstructionORA,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionSLO {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::SLO(Self {
            asl: InstructionASL {
                addr: Some(addr),
                addressing_mode,
            },
            ora: InstructionORA {
                addr,
                addressing_mode,
                page_crossed,
            },
            addressing_mode,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.asl.execute(cpu);
        self.ora.execute(cpu);
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

    #[test_case(SLO_ZEROPAGE ; "zero_page")]
    #[test_case(SLO_ZEROPAGEX ; "zero_page_x")]
    #[test_case(SLO_ABSOLUTE ; "absolute")]
    #[test_case(SLO_ABSOLUTEX ; "absolute_x")]
    #[test_case(SLO_ABSOLUTEY ; "absolute_y")]
    #[test_case(SLO_INDIRECTX ; "indirect_x")]
    #[test_case(SLO_INDIRECTY ; "indirect_y")]
    fn slo(instruction: u8) {
        // Just test that it runs, ASL and ORA are already tested.
        CPU::new_test(&[instruction]).run();
    }
}
