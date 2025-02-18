use crate::{Instruction, OpCode, Status, CPU};

pub const CLI: u8 = 0x58;

/// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
#[derive(Debug)]
pub struct InstructionCLI;

impl OpCode for InstructionCLI {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::CLI(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status.remove(Status::INTERRUPT_DISABLE);
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
    fn cli() {
        let mut cpu = CPU::new_test(&[CLI, BRK]);
        cpu.status.insert(Status::INTERRUPT_DISABLE);
        cpu.run();
        assert!(!cpu.status.contains(Status::INTERRUPT_DISABLE))
    }
}
