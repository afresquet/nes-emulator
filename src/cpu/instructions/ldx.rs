use crate::{OpCode, CPU};

pub const LDX_IMMEDIATE: u8 = 0xA2;
pub const LDX_ZEROPAGE: u8 = 0xA6;
pub const LDX_ZEROPAGEY: u8 = 0xB6;
pub const LDX_ABSOLUTE: u8 = 0xAE;
pub const LDX_ABSOLUTEY: u8 = 0xBE;

/// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
pub fn ldx(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.register_x = cpu.mem_read(addr);
    cpu.update_zero_and_negative_flags(cpu.register_x);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(LDX_IMMEDIATE, 0x05, 0x00, 0x80 ; "immediate")]
    #[test_case(LDX_ZEROPAGE, 0x10, 0x00, 0x1A ; "zero_page")]
    #[test_case(LDX_ZEROPAGEY, 0x06, 0x00, 0x10 ; "zero_page_y")]
    #[test_case(LDX_ABSOLUTE, 0x20, 0x00, 0x2A ; "absolute")]
    #[test_case(LDX_ABSOLUTEY, 0x06, 0x00, 0x10 ; "absolute_y")]
    fn ldx(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, load, BRK]);
        cpu.register_y = 0x0A;
        cpu.mem_write(0x10, 0x05);
        cpu.mem_write(0x1A, 0x80);
        cpu.mem_write_u16(0x20, 0x05);
        cpu.mem_write_u16(0x2A, 0x80);
        cpu.mem_write_u16(0x30, 0x10);
        cpu.mem_write_u16(0x3A, 0x1A);
        cpu.mem_write_u16(0x4A, 0x0A);

        // Load
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Override
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_x = 0xFF;
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.load(&[instruction, zero, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.load(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
