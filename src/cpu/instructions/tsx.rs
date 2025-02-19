use crate::{Instruction, OpCode, CPU};

pub const TSX: u8 = 0xBA;

/// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionTSX;

impl OpCode for InstructionTSX {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::TSX(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.register_x = cpu.stack_pointer;
        cpu.update_zero_and_negative_flags(cpu.register_x);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, STACK_SIZE};

    use super::*;

    #[test]
    fn tsx() {
        let mut cpu = CPU::new_test(&[TSX, BRK]);

        // Transfer
        cpu.run();
        assert_eq!(cpu.register_x, STACK_SIZE - 2);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
