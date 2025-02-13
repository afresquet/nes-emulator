use crate::{OpCode, CPU};

pub const PHA: u8 = 0x48;

/// Pushes a copy of the accumulator on to the stack.
pub fn pha(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.stack_push(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, TXA};

    use super::*;

    #[test]
    fn pha() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[PHA, TXA, PHA, BRK]);
        cpu.reset();
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
        let mut cpu = CPU::new();
        cpu.load(&[PHA, BRK]);
        cpu.reset();
        cpu.stack_pointer = 0;
        cpu.run();
    }
}
