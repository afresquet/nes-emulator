use crate::{Instruction, OpCode, CPU};

use super::{InstructionAND, InstructionLSR};

pub const ASR_IMMEDIATE: u8 = 0x4B;

/// Perform AND and LSR.
#[derive(Debug)]
pub struct InstructionASR {
    and: InstructionAND,
    lsr: InstructionLSR,
}

impl OpCode for InstructionASR {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::ASR(Self {
            and: InstructionAND {
                addr,
                addressing_mode,
                page_crossed,
            },
            lsr: InstructionLSR {
                addr: None,
                addressing_mode,
            },
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.and.execute(cpu);
        self.lsr.execute(cpu);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slo() {
        // Just test that it runs, AND and LSR are already tested.
        CPU::new_test(&[ASR_IMMEDIATE]).run();
    }
}
