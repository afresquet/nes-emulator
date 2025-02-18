use crate::{Instruction, OpCode, Status, CPU};

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

    fn cycles(&self) -> u8 {
        2
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
        assert!(cpu.status.contains(Status::DECIMAL));
    }
}
