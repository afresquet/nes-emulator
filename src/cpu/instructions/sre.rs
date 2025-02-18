use crate::{AddressingMode, Instruction, OpCode, CPU};

use super::{InstructionEOR, InstructionLSR};

pub const SRE_ZEROPAGE: u8 = 0x47;
pub const SRE_ZEROPAGEX: u8 = 0x57;
pub const SRE_ABSOLUTE: u8 = 0x4F;
pub const SRE_ABSOLUTEX: u8 = 0x5F;
pub const SRE_ABSOLUTEY: u8 = 0x5B;
pub const SRE_INDIRECTX: u8 = 0x43;
pub const SRE_INDIRECTY: u8 = 0x53;

/// Perfoms LSR and EOR.
#[derive(Debug)]
pub struct InstructionSRE {
    asl: InstructionLSR,
    ora: InstructionEOR,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionSRE {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::SRE(Self {
            asl: InstructionLSR {
                addr: Some(addr),
                addressing_mode,
            },
            ora: InstructionEOR {
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

    #[test_case(SRE_ZEROPAGE ; "zero_page")]
    #[test_case(SRE_ZEROPAGEX ; "zero_page_x")]
    #[test_case(SRE_ABSOLUTE ; "absolute")]
    #[test_case(SRE_ABSOLUTEX ; "absolute_x")]
    #[test_case(SRE_ABSOLUTEY ; "absolute_y")]
    #[test_case(SRE_INDIRECTX ; "indirect_x")]
    #[test_case(SRE_INDIRECTY ; "indirect_y")]
    fn sre(instruction: u8) {
        // Just test that it runs, LSR and EOR are already tested.
        CPU::new_test(&[instruction]).run();
    }
}
