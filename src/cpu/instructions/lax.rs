use crate::{AddressingMode, Instruction, OpCode, CPU};

use super::{InstructionLDA, InstructionLDX};

pub const LAX_ZEROPAGE: u8 = 0xA7;
pub const LAX_ZEROPAGEY: u8 = 0xB7;
pub const LAX_ABSOLUTE: u8 = 0xAF;
pub const LAX_ABSOLUTEY: u8 = 0xBF;
pub const LAX_INDIRECTX: u8 = 0xA3;
pub const LAX_INDIRECTY: u8 = 0xB3;

/// Performs LDA and LDX.
#[derive(Debug)]
pub struct InstructionLAX {
    lda: InstructionLDA,
    ldx: InstructionLDX,
    page_crossed: bool,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionLAX {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::LAX(Self {
            lda: InstructionLDA {
                addr,
                addressing_mode,
                page_crossed,
            },
            ldx: InstructionLDX {
                addr,
                addressing_mode,
                page_crossed,
            },
            page_crossed,
            addressing_mode,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.lda.execute(cpu);
        self.ldx.execute(cpu);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageY | AddressingMode::Absolute => 4,
            AddressingMode::AbsoluteY => 4 + self.page_crossed as u8,
            AddressingMode::IndirectX => 6,
            AddressingMode::IndirectY => 5 + self.page_crossed as u8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(LAX_ZEROPAGE ; "zero_page")]
    #[test_case(LAX_ZEROPAGEY ; "zero_page_y")]
    #[test_case(LAX_ABSOLUTE ; "absolute")]
    #[test_case(LAX_ABSOLUTEY ; "absolute_y")]
    #[test_case(LAX_INDIRECTX ; "indirect_x")]
    #[test_case(LAX_INDIRECTY ; "indirect_y")]
    fn lax(instruction: u8) {
        // Just test that it runs, ASL and ORA are already tested.
        CPU::new_test(&[instruction]).run();
    }
}
