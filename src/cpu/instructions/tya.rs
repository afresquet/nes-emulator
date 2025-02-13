use crate::{OpCode, CPU};

pub const TYA: u8 = 0x98;

/// Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.
pub fn tya(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.register_a = cpu.register_y;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn tya() {
        let mut cpu = CPU::new();
        cpu.load(&[TYA, BRK]);

        // Transfer
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.reset();
        cpu.register_y = 0x80;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
