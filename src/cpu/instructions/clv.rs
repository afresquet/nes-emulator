use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const CLV: u8 = 0xB8;

/// Clears the overflow flag.
#[derive(Debug)]
pub struct InstructionCLV;

impl OpCode for InstructionCLV {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::CLV(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.status.remove(Status::OVERFLOW);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clv() {
        let mut cpu = CPU::new().insert_test_rom(&[CLV, BRK]);
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert!(!cpu.status.intersects(Status::OVERFLOW))
    }
}
