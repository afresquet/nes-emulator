use crate::{Bus, OpCode, Rom, CPU};

use super::Instruction;

pub const PHA: u8 = 0x48;

/// Pushes a copy of the accumulator on to the stack.
#[derive(Debug)]
pub struct InstructionPHA;

impl OpCode for InstructionPHA {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::PHA(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.stack_push(cpu.register_a);
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, TXA};

    use super::*;

    #[test]
    fn pha() {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[PHA, TXA, PHA, BRK]);
        cpu.register_a = 0x10;
        cpu.register_x = 0x20;

        // Push
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull(), 0x20);
        assert_eq!(cpu.stack_pull(), 0x10);
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new().insert_test_rom(&[PHA, BRK]);
        cpu.stack_pointer = 0;
        cpu.run();
    }
}
