use crate::{OpCode, CPU};

pub const TAY: u8 = 0xA8;

/// Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.
pub fn tay(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.register_y = cpu.register_a;
    cpu.update_zero_and_negative_flags(cpu.register_y);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn tay() {
        let mut cpu = CPU::new();
        cpu.load(&[TAY, BRK]);

        // Transfer
        cpu.reset();
        cpu.register_a = 0x05;
        cpu.run();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.reset();
        cpu.register_a = 0x80;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
