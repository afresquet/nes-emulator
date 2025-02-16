use crate::{Instruction, OpCode, CPU};

pub const INY: u8 = 0xC8;

/// Adds one to the Y register setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionINY;

impl OpCode for InstructionINY {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::INY(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.register_y = cpu.register_y.wrapping_add(1);
        cpu.update_zero_and_negative_flags(cpu.register_y);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn iny() {
        let mut cpu = CPU::new_test(&[INY, BRK]);

        // Increments
        cpu.run();
        assert_eq!(cpu.register_y, 1);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Overflow
        cpu.swap_test_rom(&[INY, INY, BRK]);
        cpu.reset();
        cpu.register_y = u8::MAX;
        cpu.run();
        assert_eq!(cpu.register_y, 1);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[INY, BRK]);
        cpu.reset();
        cpu.register_y = u8::MAX;
        cpu.run();
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[INY, BRK]);
        cpu.reset();
        cpu.register_y = u8::MAX - 1;
        cpu.run();
        assert_eq!(cpu.register_y, u8::MAX);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
