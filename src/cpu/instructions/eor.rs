use crate::{OpCode, CPU};

pub const EOR_IMMEDIATE: u8 = 0x49;
pub const EOR_ZEROPAGE: u8 = 0x45;
pub const EOR_ZEROPAGEX: u8 = 0x55;
pub const EOR_ABSOLUTE: u8 = 0x4D;
pub const EOR_ABSOLUTEX: u8 = 0x5D;
pub const EOR_ABSOLUTEY: u8 = 0x59;
pub const EOR_INDIRECTX: u8 = 0x41;
pub const EOR_INDIRECTY: u8 = 0x51;

/// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
pub fn eor(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let data = cpu.mem_read(addr);
    cpu.register_a ^= data;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(EOR_IMMEDIATE, 0b1000_0000, 0b1000_1010 ; "immediate")]
    #[test_case(EOR_ZEROPAGE, 0x20, 0x2A ; "zero_page")]
    #[test_case(EOR_ZEROPAGEX, 0x10, 0x1A ; "zero_page_x")]
    #[test_case(EOR_ABSOLUTE, 0x30, 0x3A ; "absolute")]
    #[test_case(EOR_ABSOLUTEX, 0x10, 0x1A ; "absolute_x")]
    #[test_case(EOR_ABSOLUTEY, 0x16, 0x20 ; "absolute_y")]
    #[test_case(EOR_INDIRECTX, 0x40, 0x4A ; "indirect_x")]
    #[test_case(EOR_INDIRECTY, 0x60, 0x6A ; "indirect_y")]
    fn eor(instruction: u8, eor: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, eor, BRK]);
        cpu.reset();
        cpu.register_a = 0b1000_1010;
        cpu.register_x = 0x10;
        cpu.register_y = 0x0A;
        cpu.mem_write(0x20, 0b1000_0000);
        cpu.mem_write(0x2A, 0b1000_1010);
        cpu.mem_write_u16(0x30, 0b1000_0000);
        cpu.mem_write_u16(0x3A, 0b1000_1010);
        cpu.mem_write_u16(0x40, 0x10);
        cpu.mem_write_u16(0x4A, 0x00);
        cpu.mem_write_u16(0x50, 0x20);
        cpu.mem_write_u16(0x5A, 0x2A);
        cpu.mem_write_u16(0x60, 0x16);
        cpu.mem_write_u16(0x6A, 0x20);

        // EOR
        cpu.run();
        assert_eq!(cpu.register_a, 0b1010);
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
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
    }
}
