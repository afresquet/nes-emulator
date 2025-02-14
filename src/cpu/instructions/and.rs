use crate::{Bus, Mem, OpCode, Rom, CPU};

pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZEROPAGE: u8 = 0x25;
pub const AND_ZEROPAGEX: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2D;
pub const AND_ABSOLUTEX: u8 = 0x3D;
pub const AND_ABSOLUTEY: u8 = 0x39;
pub const AND_INDIRECTX: u8 = 0x21;
pub const AND_INDIRECTY: u8 = 0x31;

/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
pub fn and(cpu: &mut CPU<Bus<Rom>>, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let data = cpu.mem_read(addr);
    cpu.register_a &= data;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(AND_IMMEDIATE, 0b1010, 0b1000_0000 ; "immediate")]
    #[test_case(AND_ZEROPAGE, 0x20, 0x2A ; "zero_page")]
    #[test_case(AND_ZEROPAGEX, 0x10, 0x1A ; "zero_page_x")]
    #[test_case(AND_ABSOLUTE, 0x30, 0x3A ; "absolute")]
    #[test_case(AND_ABSOLUTEX, 0x10, 0x1A ; "absolute_x")]
    #[test_case(AND_ABSOLUTEY, 0x16, 0x20 ; "absolute_y")]
    #[test_case(AND_INDIRECTX, 0x40, 0x4A ; "indirect_x")]
    #[test_case(AND_INDIRECTY, 0x60, 0x6A ; "indirect_y")]
    fn and(instruction: u8, and: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[instruction, and, BRK]);
        cpu.register_a = 0b1000_1010;
        cpu.register_x = 0x10;
        cpu.register_y = 0x0A;
        cpu.mem_write(0x20, 0b1010);
        cpu.mem_write(0x2A, 0b1000_0000);
        cpu.mem_write_u16(0x30, 0b1010);
        cpu.mem_write_u16(0x3A, 0b1000_0000);
        cpu.mem_write_u16(0x40, 0x10);
        cpu.mem_write_u16(0x4A, 0x00);
        cpu.mem_write_u16(0x50, 0x20);
        cpu.mem_write_u16(0x5A, 0x2A);
        cpu.mem_write_u16(0x60, 0x16);
        cpu.mem_write_u16(0x6A, 0x20);

        // AND
        cpu.run();
        assert_eq!(cpu.register_a, 0b1010);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, 0, BRK]);
        cpu.reset_status();
        cpu.register_a = 0;
        cpu.run();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.register_a = 0b1000_1010;
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
