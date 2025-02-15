use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const CLI: u8 = 0x58;

/// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
#[derive(Debug)]
pub struct InstructionCLI;

impl OpCode for InstructionCLI {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::CLI(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.status.remove(Status::INTERRUPT_DISABLE);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn cli() {
        let mut cpu = CPU::new().insert_test_rom(&[CLI, BRK]);
        cpu.status.insert(Status::INTERRUPT_DISABLE);
        cpu.run();
        assert!(!cpu.status.intersects(Status::INTERRUPT_DISABLE))
    }
}
