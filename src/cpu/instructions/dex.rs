use crate::{Instruction, OpCode, CPU};

pub const DEX: u8 = 0xCA;

/// Subtracts one from the X register setting the zero and negative flags as appropriate.
#[derive(Debug)]
pub struct InstructionDEX;

impl OpCode for InstructionDEX {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::DEX(Self)
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        let result = cpu.register_x.wrapping_sub(1);
        cpu.register_x = result;
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
    fn dex() {
        // Setup
        let mut cpu = CPU::new_test(&[DEX, BRK]);

        // Decrement
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
