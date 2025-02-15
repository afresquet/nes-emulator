use crate::{Instruction, OpCode, Status, CPU};

pub const SEI: u8 = 0x78;

/// Set the interrupt disable flag to one.
#[derive(Debug)]
pub struct InstructionSEI;

impl OpCode for InstructionSEI {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::SEI(Self)
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.status.insert(Status::INTERRUPT_DISABLE);
        self.cycles(false)
    }

    fn cycles(&self, _page_crossed: bool) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sei() {
        let mut cpu = CPU::new_test(&[SEI, BRK]);
        cpu.run();
        assert!(cpu.status.intersects(Status::INTERRUPT_DISABLE));
    }
}
