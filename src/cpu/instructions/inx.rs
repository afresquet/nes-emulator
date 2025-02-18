use crate::{Instruction, OpCode, CPU};

pub const INX: u8 = 0xE8;

/// Adds one to the X register setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionINX;

impl OpCode for InstructionINX {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::INX(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.register_x = cpu.register_x.wrapping_add(1);
        cpu.update_zero_and_negative_flags(cpu.register_x);
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
    fn inx() {
        let mut cpu = CPU::new_test(&[INX, BRK]);

        // Increments
        cpu.run();
        assert_eq!(cpu.register_x, 1);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Overflow
        cpu.swap_test_rom(&[INX, INX, BRK]);
        cpu.reset();
        cpu.register_x = u8::MAX;
        cpu.run();
        assert_eq!(cpu.register_x, 1);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[INX, BRK]);
        cpu.reset();
        cpu.register_x = u8::MAX;
        cpu.run();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[INX, BRK]);
        cpu.reset();
        cpu.register_x = u8::MAX - 1;
        cpu.run();
        assert_eq!(cpu.register_x, u8::MAX);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
