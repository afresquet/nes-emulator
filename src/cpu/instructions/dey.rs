use crate::{Instruction, OpCode, CPU};

pub const DEY: u8 = 0x88;

/// Subtracts one from the Y register setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionDEY;

impl OpCode for InstructionDEY {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::DEY(Self)
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        let result = cpu.register_y.wrapping_sub(1);
        cpu.register_y = result;
        cpu.update_zero_and_negative_flags(result);
        self.cycles(false)
    }

    fn cycles(&self, _page_crossed: bool) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn dey() {
        // Setup
        let mut cpu = CPU::new_test(&[DEY, BRK]);

        // Decrement
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
