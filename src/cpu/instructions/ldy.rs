use crate::{OpCode, CPU};

pub const LDY_IMMEDIATE: u8 = 0xA0;
pub const LDY_ZEROPAGE: u8 = 0xA4;
pub const LDY_ZEROPAGEX: u8 = 0xB4;
pub const LDY_ABSOLUTE: u8 = 0xAC;
pub const LDY_ABSOLUTEX: u8 = 0xBC;

/// Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
pub fn ldy(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.register_y = cpu.mem_read(addr);
    cpu.update_zero_and_negative_flags(cpu.register_y);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(LDY_IMMEDIATE, 0x05, 0x00, 0x80 ; "immediate")]
    #[test_case(LDY_ZEROPAGE, 0x10, 0x00, 0x1A ; "zero_page")]
    #[test_case(LDY_ZEROPAGEX, 0x06, 0x00, 0x10 ; "zero_page_x")]
    #[test_case(LDY_ABSOLUTE, 0x20, 0x00, 0x2A ; "absolute")]
    #[test_case(LDY_ABSOLUTEX, 0x06, 0x00, 0x10 ; "absolute_x")]
    fn ldx(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, load, BRK]);
        cpu.register_x = 0x0A;
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
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Override
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_y = 0xFF;
        cpu.run();
        assert_eq!(cpu.register_y, 0x05);
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
