use crate::{Instruction, OpCode, Status, CPU};

pub const CLD: u8 = 0xD8;

/// Sets the decimal mode flag to zero.
#[derive(Debug)]
pub struct InstructionCLD;

impl OpCode for InstructionCLD {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::CLD(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status.remove(Status::DECIMAL);
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
    fn cld() {
        let mut cpu = CPU::new_test(&[CLD, BRK]);
        cpu.status.insert(Status::DECIMAL);
        cpu.run();
        assert!(!cpu.status.contains(Status::DECIMAL))
    }
}
