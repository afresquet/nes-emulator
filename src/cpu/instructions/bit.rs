use crate::{Mem, OpCode, Status, CPU};

use super::Instruction;

pub const BIT_ZEROPAGE: u8 = 0x24;
pub const BIT_ABSOLUTE: u8 = 0x2C;

/// This instructions is used to test if one or more bits are set in a target memory location.
/// The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept.
/// Bits 7 and 6 of the value from memory are copied into the N and V flags.
#[derive(Debug)]
pub struct InstructionBIT {
    addr: u16,
}

impl OpCode for InstructionBIT {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BIT(Self {
            addr: cpu.get_operand_address(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let data = cpu.mem_read(self.addr);

        let result = cpu.register_a & data;

        cpu.update_zero_flag(result);

        cpu.status.set(Status::OVERFLOW, data & 1 << 6 != 0);
        cpu.update_negative_flag(data);
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::instructions::BRK;

    use super::*;

    #[test_case(BIT_ZEROPAGE ; "zero_page")]
    #[test_case(BIT_ABSOLUTE ; "absolute")]
    fn bit(instruction: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, 0x10, BRK]);
        cpu.mem_write(0x10, 0);
        cpu.mem_write(0x20, 0b0101_0101);
        cpu.mem_write(0x30, 0b1001_0101);

        // Zero Flag
        cpu.register_a = 0b0100_1000;
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::OVERFLOW));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Overflow Flag
        cpu.swap_test_rom(&[instruction, 0x20, BRK]);
        cpu.reset_status();
        cpu.register_a = 0b0110_0101;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::OVERFLOW));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, 0x30, BRK]);
        cpu.reset_status();
        cpu.register_a = 0b1100_0011;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::OVERFLOW));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
