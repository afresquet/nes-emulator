use crate::{Instruction, OpCode, Status, CPU};

pub const SEC: u8 = 0x38;

/// Set the carry flag to one.
#[derive(Debug)]
pub struct InstructionSEC;

impl OpCode for InstructionSEC {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::SEC(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status.insert(Status::CARRY);
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
    fn sec() {
        let mut cpu = CPU::new_test(&[SEC, BRK]);
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }
}
