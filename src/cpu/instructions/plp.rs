use crate::{Bus, OpCode, Rom, Status, CPU};

pub const PLP: u8 = 0x28;

/// Pulls an 8 bit value from the stack and into the processor flags.
/// The flags will take on new states as determined by the value pulled.
pub fn plp(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    let status = cpu.stack_pull();
    cpu.status = Status::from_bits_retain(status);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, STACK_SIZE};

    use super::*;

    #[test]
    fn plp() {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[PLP, BRK]);
        cpu.stack_push(0b0101_0101);

        // Push
        cpu.run();
        assert_eq!(cpu.status, Status::from_bits_truncate(0b0101_0101));
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new().insert_test_rom(&[PLP, BRK]);
        cpu.stack_pointer = STACK_SIZE;
        cpu.run();
    }
}
