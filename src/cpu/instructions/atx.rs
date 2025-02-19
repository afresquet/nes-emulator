use crate::{Instruction, OpCode, CPU};

use super::{InstructionAND, InstructionTAX};

pub const ATX_IMMEDIATE: u8 = 0xAB;

/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory,
/// then copy the value of the accumulator to the X register.
#[derive(Debug)]
pub struct InstructionATX {
    and: InstructionAND,
    tax: InstructionTAX,
}

impl OpCode for InstructionATX {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::LXA(Self {
            and: InstructionAND {
                addr,
                addressing_mode: cpu.get_addressing_mode(),
                page_crossed,
            },
            tax: InstructionTAX,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.and.execute(cpu);
        self.tax.execute(cpu);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atx() {
        // Just test that it runs, AND and LSR are already tested.
        CPU::new_test(&[ATX_IMMEDIATE]).run();
    }
}
