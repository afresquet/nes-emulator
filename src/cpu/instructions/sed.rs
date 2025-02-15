use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const SED: u8 = 0xF8;

/// Set the decimal mode flag to one.
#[derive(Debug)]
pub struct InstructionSED;

impl OpCode for InstructionSED {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::SED(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status.insert(Status::DECIMAL);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sed() {
        let mut cpu = CPU::new_test(&[SED, BRK]);
        cpu.run();
        assert!(cpu.status.intersects(Status::DECIMAL));
    }
}
