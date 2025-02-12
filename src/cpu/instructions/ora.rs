use crate::{OpCode, CPU};

pub const ORA_IMMEDIATE: u8 = 0x09;
pub const ORA_ZEROPAGE: u8 = 0x05;
pub const ORA_ZEROPAGEX: u8 = 0x15;
pub const ORA_ABSOLUTE: u8 = 0x0D;
pub const ORA_ABSOLUTEX: u8 = 0x1D;
pub const ORA_ABSOLUTEY: u8 = 0x19;
pub const ORA_INDIRECTX: u8 = 0x01;
pub const ORA_INDIRECTY: u8 = 0x11;

/// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
pub fn ora(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let data = cpu.mem_read(addr);
    cpu.register_a |= data;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(ORA_IMMEDIATE, 0b0101, 0b1000_0000 ; "immediate")]
    #[test_case(ORA_ZEROPAGE, 0x20, 0x2A ; "zero_page")]
    #[test_case(ORA_ZEROPAGEX, 0x10, 0x1A ; "zero_page_x")]
    #[test_case(ORA_ABSOLUTE, 0x30, 0x3A ; "absolute")]
    #[test_case(ORA_ABSOLUTEX, 0x10, 0x1A ; "absolute_x")]
    #[test_case(ORA_ABSOLUTEY, 0x16, 0x20 ; "absolute_y")]
    #[test_case(ORA_INDIRECTX, 0x40, 0x4A ; "indirect_x")]
    #[test_case(ORA_INDIRECTY, 0x60, 0x6A ; "indirect_y")]
    fn ora(instruction: u8, load: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, load, BRK]);
        cpu.reset();
        cpu.register_a = 0b1010;
        cpu.register_x = 0x10;
        cpu.register_y = 0x0A;
        cpu.mem_write(0x20, 0b0101);
        cpu.mem_write(0x2A, 0b1000_0000);
        cpu.mem_write_u16(0x30, 0b0101);
        cpu.mem_write_u16(0x3A, 0b1000_0000);
        cpu.mem_write_u16(0x40, 0x10);
        cpu.mem_write_u16(0x4A, 0x00);
        cpu.mem_write_u16(0x50, 0x20);
        cpu.mem_write_u16(0x5A, 0x2A);
        cpu.mem_write_u16(0x60, 0x16);
        cpu.mem_write_u16(0x6A, 0x20);

        // OR
        cpu.run();
        assert_eq!(cpu.register_a, 0b1111);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.load(&[instruction, 0, BRK]);
        cpu.reset_program_counter();
        cpu.reset_status();
        cpu.register_a = 0;
        cpu.run();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.load(&[instruction, negative, BRK]);
        cpu.reset_program_counter();
        cpu.reset_status();
        cpu.register_a = 0b1010;
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1010);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
