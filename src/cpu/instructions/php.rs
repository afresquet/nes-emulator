use crate::{OpCode, CPU};

pub const PHP: u8 = 0x08;

/// Pushes a copy of the status flags on to the stack.
pub fn php(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.stack_push(cpu.status.bits());
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn php() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[PHP, BRK]);
        cpu.reset();
        cpu.status = Status::from_bits_retain(0b1010_1010);

        // Push
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull(), 0b1010_1010);
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new();
        cpu.load(&[PHP, BRK]);
        cpu.reset();
        cpu.stack_pointer = 0;
        cpu.run();
    }
}
