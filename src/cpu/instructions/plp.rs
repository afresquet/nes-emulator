use crate::{OpCode, Status, CPU};

pub const PLP: u8 = 0x28;

/// Pulls an 8 bit value from the stack and into the processor flags.
/// The flags will take on new states as determined by the value pulled.
pub fn plp(cpu: &mut CPU, _opcode: &OpCode) {
    let status = cpu.stack_pull();
    cpu.status = Status::from_bits_retain(status);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, STACK, STACK_SIZE};

    use super::*;

    #[test]
    fn plp() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[PLP, BRK]);
        cpu.reset();
        cpu.memory[STACK + cpu.stack_pointer as usize] = 0b0101_0101;
        cpu.stack_pointer -= 1;

        // Push
        cpu.run();
        assert_eq!(cpu.status, Status::from_bits_truncate(0b0101_0101));
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new();
        cpu.load(&[PLP, BRK]);
        cpu.reset();
        cpu.stack_pointer = STACK_SIZE;
        cpu.run();
    }
}
