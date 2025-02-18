use crate::{AddressingMode, Instruction, OpCode, CPU};

use super::{InstructionAND, InstructionROL};

pub const RLA_ZEROPAGE: u8 = 0x27;
pub const RLA_ZEROPAGEX: u8 = 0x37;
pub const RLA_ABSOLUTE: u8 = 0x2F;
pub const RLA_ABSOLUTEX: u8 = 0x3F;
pub const RLA_ABSOLUTEY: u8 = 0x3B;
pub const RLA_INDIRECTX: u8 = 0x23;
pub const RLA_INDIRECTY: u8 = 0x33;

/// Perfoms ROL and AND.
#[derive(Debug)]
pub struct InstructionRLA {
    rol: InstructionROL,
    and: InstructionAND,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionRLA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::RLA(Self {
            rol: InstructionROL {
                addr: Some(addr),
                addressing_mode,
            },
            and: InstructionAND {
                addr,
                addressing_mode,
                page_crossed,
            },
            addressing_mode,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.rol.execute(cpu);
        self.and.execute(cpu);
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

    #[test_case(RLA_ZEROPAGE ; "zero_page")]
    #[test_case(RLA_ZEROPAGEX ; "zero_page_x")]
    #[test_case(RLA_ABSOLUTE ; "absolute")]
    #[test_case(RLA_ABSOLUTEX ; "absolute_x")]
    #[test_case(RLA_ABSOLUTEY ; "absolute_y")]
    #[test_case(RLA_INDIRECTX ; "indirect_x")]
    #[test_case(RLA_INDIRECTY ; "indirect_y")]
    fn rla(instruction: u8) {
        // Just test that it runs, ASL and ORA are already tested.
        CPU::new_test(&[instruction]).run();
    }
}
