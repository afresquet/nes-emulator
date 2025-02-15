use crate::{Mem, OpCode, CPU};

use super::Instruction;

pub const CPY_IMMEDIATE: u8 = 0xC0;
pub const CPY_ZEROPAGE: u8 = 0xC4;
pub const CPY_ABSOLUTE: u8 = 0xCC;

/// This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.
#[derive(Debug)]
pub struct InstructionCPY {
    addr: u16,
}

impl OpCode for InstructionCPY {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::CPY(Self {
            addr: cpu.get_operand_address(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let data = cpu.mem_read(self.addr);
        cpu.compare(data, cpu.register_y);
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Mem, Status};

    use super::*;

    #[test_case(CPY_IMMEDIATE, 0x00, 0x10, 0x11 ; "immediate")]
    #[test_case(CPY_ZEROPAGE, 0x10, 0x20, 0x30 ; "zero_page")]
    #[test_case(CPY_ABSOLUTE, 0x10, 0x20, 0x30 ; "absolute")]
    fn cpy(instruction: u8, carry: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, carry, BRK]);
        cpu.register_y = 0x10;
        cpu.mem_write(0x10, 0x00);
        cpu.mem_write(0x20, 0x10);
        cpu.mem_write(0x30, 0x11);

        // Carry Flag
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.run();
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
