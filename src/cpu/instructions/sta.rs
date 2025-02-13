use crate::{OpCode, CPU};

pub const STA_ZEROPAGE: u8 = 0x85;
pub const STA_ZEROPAGEX: u8 = 0x95;
pub const STA_ABSOLUTE: u8 = 0x8D;
pub const STA_ABSOLUTEX: u8 = 0x9D;
pub const STA_ABSOLUTEY: u8 = 0x99;
pub const STA_INDIRECTX: u8 = 0x81;
pub const STA_INDIRECTY: u8 = 0x91;

/// Stores the contents of the accumulator into memory.
pub fn sta(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.mem_write(addr, cpu.register_a);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::instructions::BRK;

    use super::*;

    #[test_case(STA_ZEROPAGE, 0x00, 0x00 ; "zero_page")]
    #[test_case(STA_ZEROPAGEX, 0x00, 0x10 ; "zero_page_x")]
    #[test_case(STA_ABSOLUTE, 0x00, 0x00 ; "absolute")]
    #[test_case(STA_ABSOLUTEX, 0x00, 0x10 ; "absolute_x")]
    #[test_case(STA_ABSOLUTEY, 0x00, 0x1A ; "absolute_y")]
    #[test_case(STA_INDIRECTX, 0x00, 0x00 ; "indirect_x")]
    #[test_case(STA_INDIRECTY, 0x00, 0x1A ; "indirect_y")]
    fn sta(instruction: u8, arg: u8, addr: usize) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, arg, BRK]);
        cpu.reset();
        cpu.register_x = 0x10;
        cpu.register_y = 0x1A;
        cpu.mem_write_u16(0x10, 0x00);

        // Store
        cpu.register_a = 0xFF;
        cpu.run();
        assert_eq!(cpu.memory[addr], 0xFF);
    }
}
