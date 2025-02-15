use crate::{OpCode, CPU};

use super::Instruction;

pub const TSX: u8 = 0xBA;

/// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionTSX;

impl OpCode for InstructionTSX {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::TSX(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.register_x = cpu.stack_pull();
        cpu.update_zero_and_negative_flags(cpu.register_x);
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn tsx() {
        let mut cpu = CPU::new_test(&[TSX, BRK]);

        // Transfer
        cpu.stack_push(0x05);
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.stack_push(0);
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.reset();
        cpu.stack_push(0x80);
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
