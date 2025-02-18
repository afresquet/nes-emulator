use crate::{Instruction, OpCode, Status, CPU};

pub const SEI: u8 = 0x78;

/// Set the interrupt disable flag to one.
#[derive(Debug)]
pub struct InstructionSEI;

impl OpCode for InstructionSEI {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::SEI(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status.insert(Status::INTERRUPT_DISABLE);
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
    fn sei() {
        let mut cpu = CPU::new_test(&[SEI, BRK]);
        cpu.run();
        assert!(cpu.status.contains(Status::INTERRUPT_DISABLE));
    }
}
