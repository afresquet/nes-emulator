use crate::{Instruction, OpCode, CPU};

pub const PLA: u8 = 0x68;

/// Pulls an 8 bit value from the stack and into the accumulator.
/// The zero and negative flags are set as appropriate.
#[derive(Debug)]
pub struct InstructionPLA;

impl OpCode for InstructionPLA {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::PLA(Self)
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.register_a = cpu.stack_pull();
        cpu.update_zero_and_negative_flags(cpu.register_a);
        self.cycles(false)
    }

    fn cycles(&self, _page_crossed: bool) -> u8 {
        4
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, STACK_SIZE};

    use super::*;

    #[test]
    fn pla() {
        // Setup
        let mut cpu = CPU::new_test(&[PLA, BRK]);
        cpu.stack_push(0x20);

        // Push
        cpu.run();
        assert_eq!(cpu.register_a, 0x20);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset();
        cpu.register_a = 0x20;
        cpu.stack_push(0);
        cpu.run();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.reset();
        cpu.stack_push(0b1000_0000);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new_test(&[PLA, BRK]);
        cpu.stack_pointer = STACK_SIZE;
        cpu.run();
    }
}
