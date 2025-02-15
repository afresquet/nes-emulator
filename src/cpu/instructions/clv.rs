use crate::{Instruction, OpCode, Status, CPU};

pub const CLV: u8 = 0xB8;

/// Clears the overflow flag.
#[derive(Debug)]
pub struct InstructionCLV;

impl OpCode for InstructionCLV {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::CLV(Self)
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.status.remove(Status::OVERFLOW);
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
    fn clv() {
        let mut cpu = CPU::new_test(&[CLV, BRK]);
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert!(!cpu.status.intersects(Status::OVERFLOW))
    }
}
