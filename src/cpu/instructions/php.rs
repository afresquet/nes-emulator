use crate::{Instruction, OpCode, Status, CPU};

pub const PHP: u8 = 0x08;

/// Pushes a copy of the status flags on to the stack with the B flag set.
#[derive(Debug)]
pub struct InstructionPHP;

impl OpCode for InstructionPHP {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::PHP(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        let status = cpu.status.union(Status::BREAK_COMMAND | Status::UNUSED);
        cpu.stack_push(status.bits());
    }

    fn cycles(&self) -> u8 {
        3
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn php() {
        // Setup
        let mut cpu = CPU::new_test(&[PHP, BRK]);
        cpu.status = Status::from_bits_retain(0b1010_1010);

        // Push
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull(), 0b1011_1010);
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new_test(&[PHP, BRK]);
        cpu.stack_pointer = 0;
        cpu.run();
    }
}
