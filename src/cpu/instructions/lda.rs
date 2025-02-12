use crate::{OpCode, CPU};

pub const LDA_IMMEDIATE: u8 = 0xA9;
pub const LDA_ZEROPAGE: u8 = 0xA5;
pub const LDA_ZEROPAGEX: u8 = 0xB5;
pub const LDA_ABSOLUTE: u8 = 0xAD;
pub const LDA_ABSOLUTEX: u8 = 0xBD;
pub const LDA_ABSOLUTEY: u8 = 0xB9;
pub const LDA_INDIRECTX: u8 = 0xA1;
pub const LDA_INDIRECTY: u8 = 0xB1;

/// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
pub fn lda(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let value = cpu.mem_read(addr);
    cpu.register_a = value;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(LDA_IMMEDIATE, 0x05, 0x00, 0x80 ; "immediate")]
    #[test_case(LDA_ZEROPAGE, 0x10, 0x00, 0x1A ; "zero_page")]
    #[test_case(LDA_ZEROPAGEX, 0x06, 0x00, 0x10 ; "zero_page_x")]
    #[test_case(LDA_ABSOLUTE, 0x20, 0x00, 0x2A ; "absolute")]
    #[test_case(LDA_ABSOLUTEX, 0x06, 0x00, 0x10 ; "absolute_x")]
    #[test_case(LDA_ABSOLUTEY, 0x00, 0x01, 0x0A ; "absolute_y")]
    #[test_case(LDA_INDIRECTX, 0x26, 0x00, 0x30 ; "indirect_x")]
    #[test_case(LDA_INDIRECTY, 0x40, 0x20, 0x4A ; "indirect_y")]
    fn lda(instruction: u8, load: u8, zero: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, load, BRK]);
        cpu.register_x = 0x0A;
        cpu.register_y = 0x10;
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
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Override
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0xFF;
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
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
