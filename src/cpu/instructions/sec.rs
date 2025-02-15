use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const SEC: u8 = 0x38;

/// Set the carry flag to one.
#[derive(Debug)]
pub struct InstructionSEC;

impl OpCode for InstructionSEC {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::SEC(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.status.insert(Status::CARRY);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sec() {
        let mut cpu = CPU::new().insert_test_rom(&[SEC, BRK]);
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
    }
}
