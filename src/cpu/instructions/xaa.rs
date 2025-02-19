use crate::{Instruction, OpCode, CPU};

use super::{InstructionAND, InstructionTXA};

pub const XAA_IMMEDIATE: u8 = 0x8B;

// Unknown operation, higly unstable
#[derive(Debug)]
pub struct InstructionXAA {
    txa: InstructionTXA,
    and: InstructionAND,
}

impl OpCode for InstructionXAA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::ANE(Self {
            txa: InstructionTXA,
            and: InstructionAND {
                addr,
                addressing_mode: cpu.get_addressing_mode(),
                page_crossed,
            },
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.txa.execute(cpu);
        self.and.execute(cpu);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xaa() {
        // Just test that it runs, TXA and AND are already tested.
        CPU::new_test(&[XAA_IMMEDIATE]).run();
    }
}
