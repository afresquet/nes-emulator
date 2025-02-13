use crate::{Mem, OpCode, CPU};

pub const DEC_ZEROPAGE: u8 = 0xC6;
pub const DEC_ZEROPAGEX: u8 = 0xD6;
pub const DEC_ABSOLUTE: u8 = 0xCE;
pub const DEC_ABSOLUTEX: u8 = 0xDE;

/// Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.
pub fn dec(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let result = cpu.mem_read(addr).wrapping_sub(1);
    cpu.mem_write(addr, result);
    cpu.update_zero_and_negative_flags(result);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(DEC_ZEROPAGE, 0x10 ; "zero_page")]
    #[test_case(DEC_ZEROPAGEX, 0x0D ; "zero_page_x")]
    #[test_case(DEC_ABSOLUTE, 0x10 ; "absolute")]
    #[test_case(DEC_ABSOLUTEX, 0x0D ; "absolute_x")]
    fn dec(instruction: u8, addr: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, addr, BRK]);
        cpu.register_x = 0x03;

        // Decrement
        cpu.reset_program_counter();
        cpu.reset_status();
        cpu.mem_write(0x10, 2);
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 1);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.reset_program_counter();
        cpu.reset_status();
        cpu.mem_write(0x10, 1);
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag and Underflow
        cpu.reset_program_counter();
        cpu.reset_status();
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), u8::MAX);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
