use crate::{AddressingMode, Instruction, OpCode, CPU};

use super::{InstructionINC, InstructionSBC};

pub const ISC_ZEROPAGE: u8 = 0xE7;
pub const ISC_ZEROPAGEX: u8 = 0xF7;
pub const ISC_ABSOLUTE: u8 = 0xEF;
pub const ISC_ABSOLUTEX: u8 = 0xFF;
pub const ISC_ABSOLUTEY: u8 = 0xFB;
pub const ISC_INDIRECTX: u8 = 0xE3;
pub const ISC_INDIRECTY: u8 = 0xF3;

/// Adds one to the value held at a specified memory location,
/// then subtracts the result to the accumulator together with the not of the carry bit.
/// If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed,
/// and setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionISC {
    inc: InstructionINC,
    sbc: InstructionSBC,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionISC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::ISB(Self {
            inc: InstructionINC {
                addr,
                addressing_mode,
            },
            sbc: InstructionSBC {
                addr,
                addressing_mode,
                page_crossed,
            },
            addressing_mode,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.inc.execute(cpu);
        self.sbc.execute(cpu);
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

    #[test_case(ISC_ZEROPAGE ; "zero_page")]
    #[test_case(ISC_ZEROPAGEX ; "zero_page_x")]
    #[test_case(ISC_ABSOLUTE ; "absolute")]
    #[test_case(ISC_ABSOLUTEX ; "absolute_x")]
    #[test_case(ISC_ABSOLUTEY ; "absolute_y")]
    #[test_case(ISC_INDIRECTX ; "indirect_x")]
    #[test_case(ISC_INDIRECTY ; "indirect_y")]
    fn isc(instruction: u8) {
        // Just test that it runs, ASL and ORA are already tested.
        CPU::new_test(&[instruction]).run();
    }
}
