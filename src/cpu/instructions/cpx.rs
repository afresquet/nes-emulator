use crate::{OpCode, CPU};

pub const CPX_IMMEDIATE: u8 = 0xE0;
pub const CPX_ZEROPAGE: u8 = 0xE4;
pub const CPX_ABSOLUTE: u8 = 0xEC;

/// This instruction compares the contents of the X register with another memory held value and sets the zero and carry flags as appropriate.
pub fn cpx(cpu: &mut CPU, opcode: &OpCode) {
    cpu.compare(cpu.register_x, opcode.mode);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(CPX_IMMEDIATE, 0x00, 0x10, 0x11 ; "immediate")]
    #[test_case(CPX_ZEROPAGE, 0x10, 0x20, 0x30 ; "zero_page")]
    #[test_case(CPX_ABSOLUTE, 0x10, 0x20, 0x30 ; "absolute")]
    fn cpx(instruction: u8, carry: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.register_x = 0x10;
        cpu.mem_write(0x10, 0x00);
        cpu.mem_write(0x20, 0x10);
        cpu.mem_write(0x30, 0x11);

        // Carry Flag
        cpu.load(&[instruction, carry, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.load(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.load(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
