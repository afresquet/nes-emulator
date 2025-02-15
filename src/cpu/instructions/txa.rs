use crate::{Instruction, OpCode, CPU};

pub const TXA: u8 = 0x8A;

/// Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionTXA;

impl OpCode for InstructionTXA {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::TXA(Self)
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.register_a = cpu.register_x;
        cpu.update_zero_and_negative_flags(cpu.register_a);
        self.cycles(false)
    }

    fn cycles(&self, _page_crossed: bool) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn txa() {
        let mut cpu = CPU::new_test(&[TXA, BRK]);

        // Transfer
        cpu.register_x = 0x05;
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.reset();
        cpu.register_x = 0x80;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
