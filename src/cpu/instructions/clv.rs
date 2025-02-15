use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const CLV: u8 = 0xB8;

/// Clears the overflow flag.
#[derive(Debug)]
pub struct InstructionCLV;

impl OpCode for InstructionCLV {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::CLV(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status.remove(Status::OVERFLOW);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clv() {
        let mut cpu = CPU::new_test(&[CLV, BRK]);
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert!(!cpu.status.intersects(Status::OVERFLOW))
    }
}
