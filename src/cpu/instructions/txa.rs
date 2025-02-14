use crate::{Bus, OpCode, Rom, CPU};

pub const TXA: u8 = 0x8A;

/// Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
pub fn txa(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.register_a = cpu.register_x;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn txa() {
        let mut cpu = CPU::new().insert_test_rom(&[TXA, BRK]);

        // Transfer
        cpu.register_x = 0x05;
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
        cpu.register_x = 0x80;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
