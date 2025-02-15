use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const SEI: u8 = 0x78;

/// Set the interrupt disable flag to one.
#[derive(Debug)]
pub struct InstructionSEI;

impl OpCode for InstructionSEI {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::SEI(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.status.insert(Status::INTERRUPT_DISABLE);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sei() {
        let mut cpu = CPU::new().insert_test_rom(&[SEI, BRK]);
        cpu.run();
        assert!(cpu.status.intersects(Status::INTERRUPT_DISABLE));
    }
}
