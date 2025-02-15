use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const CLC: u8 = 0x18;

/// Set the carry flag to zero.
#[derive(Debug)]
pub struct InstructionCLC;

impl OpCode for InstructionCLC {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::CLC(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.status.remove(Status::CARRY);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clc() {
        let mut cpu = CPU::new().insert_test_rom(&[CLC, BRK]);
        cpu.status.insert(Status::CARRY);
        cpu.run();
        assert!(!cpu.status.intersects(Status::CARRY))
    }
}
