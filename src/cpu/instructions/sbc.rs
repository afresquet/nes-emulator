use crate::{Mem, OpCode, CPU};

pub const SBC_IMMEDIATE: u8 = 0xE9;
pub const SBC_ZEROPAGE: u8 = 0xE5;
pub const SBC_ZEROPAGEX: u8 = 0xF5;
pub const SBC_ABSOLUTE: u8 = 0xED;
pub const SBC_ABSOLUTEX: u8 = 0xFD;
pub const SBC_ABSOLUTEY: u8 = 0xF9;
pub const SBC_INDIRECTX: u8 = 0xE1;
pub const SBC_INDIRECTY: u8 = 0xF1;

/// This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit.
/// If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
pub fn sbc(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let value = cpu.mem_read(addr);
    cpu.sum((value as i8).wrapping_neg().wrapping_sub(1) as u8);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};
    use test_case::test_case;

    use super::*;

    #[test_case(&[SBC_IMMEDIATE, 0x40, BRK] ; "immediate")]
    #[test_case(&[SBC_ZEROPAGE, 0x10, BRK] ; "zero_page")]
    #[test_case(&[SBC_ZEROPAGEX, 0x00, BRK] ; "zero_page_x")]
    #[test_case(&[SBC_ABSOLUTE, 0x1A, BRK] ; "absolute")]
    #[test_case(&[SBC_ABSOLUTEX, 0x00, BRK] ; "absolute_x")]
    #[test_case(&[SBC_ABSOLUTEY, 0x00, BRK] ; "absolute_y")]
    #[test_case(&[SBC_INDIRECTX, 0x0A, BRK] ; "indirect_x")]
    #[test_case(&[SBC_INDIRECTY, 0x4A, BRK] ; "indirect_y")]
    fn sbc(program: &[u8]) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(program);
        cpu.register_x = 0x10;
        cpu.register_y = 0x1A;
        cpu.mem_write(0x10, 0x40);
        cpu.mem_write_u16(0x1A, 0x40);
        cpu.mem_write(0x40, 0x40);
        cpu.mem_write_u16(0x4A, 0x26);

        // From 0
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert_eq!(cpu.register_a, 0xBF);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // From existing value
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x42;
        cpu.run();
        assert_eq!(cpu.register_a, 1);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // Carry Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x41;
        cpu.run();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // Overflow Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x20;
        cpu.run();
        assert_eq!(cpu.register_a, 0xDF);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));
    }
}
