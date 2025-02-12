use crate::{OpCode, CPU};

pub const DEX: u8 = 0xCA;

/// Subtracts one from the X register setting the zero and negative flags as appropriate.
pub fn dex(cpu: &mut CPU, _opcode: &OpCode) {
    let result = cpu.register_x.wrapping_sub(1);
    cpu.register_x = result;
    cpu.update_zero_and_negative_flags(result);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn dex() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[DEX, BRK]);

        // Decrement
        cpu.reset();
        cpu.register_x = 2;
        cpu.run();
        assert_eq!(cpu.register_x, 1);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.register_x = 1;
        cpu.run();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag and Underflow
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, u8::MAX);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
