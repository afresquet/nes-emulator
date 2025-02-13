use crate::{OpCode, CPU};

pub const PHA: u8 = 0x48;

/// Pushes a copy of the accumulator on to the stack.
pub fn pha(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.stack_push(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::{BRK, TXA},
        STACK,
    };

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
        assert_eq!(cpu.memory[STACK + cpu.stack_pointer as usize + 5], 0x10);
        assert_eq!(cpu.memory[STACK + cpu.stack_pointer as usize + 4], 0x20);
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
