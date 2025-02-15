use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const CLD: u8 = 0xD8;

/// Sets the decimal mode flag to zero.
#[derive(Debug)]
pub struct InstructionCLD;

impl OpCode for InstructionCLD {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::CLD(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.status.remove(Status::DECIMAL);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn cld() {
        let mut cpu = CPU::new().insert_test_rom(&[CLD, BRK]);
        cpu.status.insert(Status::DECIMAL);
        cpu.run();
        assert!(!cpu.status.intersects(Status::DECIMAL))
    }
}
