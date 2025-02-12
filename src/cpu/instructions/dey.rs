use crate::{OpCode, CPU};

pub const DEY: u8 = 0x88;

/// Subtracts one from the Y register setting the zero and negative flags as appropriate.
pub fn dey(cpu: &mut CPU, _opcode: &OpCode) {
    let result = cpu.register_y.wrapping_sub(1);
    cpu.register_y = result;
    cpu.update_zero_and_negative_flags(result);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn dey() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[DEY, BRK]);

        // Decrement
        cpu.reset();
        cpu.register_y = 2;
        cpu.run();
        assert_eq!(cpu.register_y, 1);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.register_y = 1;
        cpu.run();
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag and Underflow
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_y, u8::MAX);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
