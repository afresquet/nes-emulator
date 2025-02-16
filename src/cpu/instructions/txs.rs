use crate::{Instruction, OpCode, CPU};

pub const TXS: u8 = 0x9A;

/// Copies the current contents of the X register into the stack register.
#[derive(Debug)]
pub struct InstructionTXS;

impl OpCode for InstructionTXS {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::TXS(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.stack_push(cpu.register_x);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn txs() {
        // Setup
        let mut cpu = CPU::new_test(&[TXS, BRK]);
        cpu.register_x = 0x05;

        // Transfer
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull(), 0x05);
    }
}
