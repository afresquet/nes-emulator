use crate::{OpCode, CPU};

pub const TSX: u8 = 0xBA;

/// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
pub fn tsx(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.register_x = cpu.stack_pull();
    cpu.update_zero_and_negative_flags(cpu.register_x);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn tsx() {
        let mut cpu = CPU::new();
        cpu.load(&[TSX, BRK]);

        // Transfer
        cpu.reset();
        cpu.stack_push(0x05);
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.stack_push(0);
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.reset();
        cpu.stack_push(0x80);
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
