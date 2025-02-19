use crate::{Instruction, OpCode, Status, CPU};

use super::InstructionAND;

pub const AAC_IMMEDIATE1: u8 = 0x0B;
pub const AAC_IMMEDIATE2: u8 = 0x2B;

/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
/// If the result is negative then the carry flag is set.
#[derive(Debug)]
pub struct InstructionAAC {
    and: InstructionAND,
}

impl OpCode for InstructionAAC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::ANC(Self {
            and: InstructionAND {
                addr,
                addressing_mode: cpu.get_addressing_mode(),
                page_crossed,
            },
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.and.execute(cpu);
        cpu.status
            .set(Status::CARRY, cpu.status.contains(Status::NEGATIVE));
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};
    use test_case::test_case;

    use super::*;

    #[test_case(AAC_IMMEDIATE1, 0b1010, 0b1000_0000 ; "immediate_1")]
    #[test_case(AAC_IMMEDIATE2, 0b1010, 0b1000_0000 ; "immediate_2")]
    fn aac(instruction: u8, aac: u8, negative: u8) {
        // Only test Carry flag, AND is already tested.
        // Setup
        let mut cpu = CPU::new_test(&[instruction, aac, BRK]);

        // Carry Flag Clear
        cpu.register_a = 0b1000_1010;
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));

        // Carry Flag Set
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset();
        cpu.register_a = 0b1000_1010;
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }
}
