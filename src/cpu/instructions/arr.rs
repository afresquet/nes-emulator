use crate::{Instruction, OpCode, Status, CPU};

use super::{InstructionAND, InstructionROR};

pub const ARR_IMMEDIATE: u8 = 0x6B;

/// Perfoms AND and ROR, then setting the Carry and Overflow flags checking bits 5 and 6 against:
/// |    Bits   |  |   Flags   |
/// |  5  |  6  |  |  C  |  V  |
/// |-----|-----|  |-----|-----|
/// |  1  |  1  |->|  1  |  0  |
/// |  1  |  0  |->|  0  |  1  |
/// |  0  |  1  |->|  1  |  1  |
/// |  0  |  0  |->|  0  |  0  |
#[derive(Debug)]
pub struct InstructionARR {
    and: InstructionAND,
    ror: InstructionROR,
}

impl OpCode for InstructionARR {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        let addressing_mode = cpu.get_addressing_mode();
        Instruction::ARR(Self {
            and: InstructionAND {
                addr,
                addressing_mode,
                page_crossed,
            },
            ror: InstructionROR {
                addr: None,
                addressing_mode,
            },
        })
    }

    fn execute(self, cpu: &mut CPU) {
        self.and.execute(cpu);
        self.ror.execute(cpu);
        let (carry, overflow) = match (cpu.register_a >> 5 & 1, cpu.register_a >> 6 & 1) {
            (1, 1) => (true, false),
            (1, 0) => (false, true),
            (0, 1) => (true, true),
            (0, 0) => (false, false),
            _ => unreachable!(),
        };
        cpu.status.set(Status::CARRY, carry);
        cpu.status.set(Status::OVERFLOW, overflow);
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
    fn arr() {
        // Only test Carry and Overflow flags, AND and ROR are already tested.
        // Setup
        let mut cpu = CPU::new_test(&[ARR_IMMEDIATE, 0b1100_0000, BRK]);

        // Carry Flag Set and Overflow Flag Clear
        cpu.register_a = 0b1100_1010;
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));

        // Carry Flag Clear and Overflow Flag Set
        cpu.swap_test_rom(&[ARR_IMMEDIATE, 0b0101_0100, BRK]);
        cpu.reset();
        cpu.register_a = 0b0100_1010;
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::OVERFLOW));

        // Carry Flag Set and Overflow Flag Set
        cpu.swap_test_rom(&[ARR_IMMEDIATE, 0b1001_0100, BRK]);
        cpu.reset();
        cpu.register_a = 0b1100_1010;
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::OVERFLOW));

        // Carry Flag Clear and Overflow Flag Clear
        cpu.swap_test_rom(&[ARR_IMMEDIATE, 0b0000_0100, BRK]);
        cpu.reset();
        cpu.register_a = 0b1100_1110;
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::OVERFLOW));
    }
}
