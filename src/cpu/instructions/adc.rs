use crate::{OpCode, CPU};

pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZEROPAGE: u8 = 0x65;
pub const ADC_ZEROPAGEX: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTEX: u8 = 0x7D;
pub const ADC_ABSOLUTEY: u8 = 0x79;
pub const ADC_INDIRECTX: u8 = 0x61;
pub const ADC_INDIRECTY: u8 = 0x71;

/// This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
pub fn adc(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let value = cpu.mem_read(addr);
    cpu.sum(value);
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};
    use test_case::test_case;

    use super::*;

    #[test_case(&[ADC_IMMEDIATE, 0x40, BRK] ; "immediate")]
    #[test_case(&[ADC_ZEROPAGE, 0x10, BRK] ; "zero_page")]
    #[test_case(&[ADC_ZEROPAGEX, 0x00, BRK] ; "zero_page_x")]
    #[test_case(&[ADC_ABSOLUTE, 0x1A, BRK] ; "absolute")]
    #[test_case(&[ADC_ABSOLUTEX, 0x00, BRK] ; "absolute_x")]
    #[test_case(&[ADC_ABSOLUTEY, 0x00, BRK] ; "absolute_y")]
    #[test_case(&[ADC_INDIRECTX, 0x0A, BRK] ; "indirect_x")]
    #[test_case(&[ADC_INDIRECTY, 0x4A, BRK] ; "indirect_y")]
    fn adc(program: &[u8]) {
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
        assert_eq!(cpu.register_a, 0x40);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // From existing value
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x01;
        cpu.run();
        assert_eq!(cpu.register_a, 0x41);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // u8::MAX
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0xBF;
        cpu.run();
        assert_eq!(cpu.register_a, 0xFF);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // Carry Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0xC0;
        cpu.run();
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
        assert!(cpu.status.intersects(Status::CARRY));
        assert!(!cpu.status.intersects(Status::OVERFLOW));

        // Overflow Flag
        cpu.reset_status();
        cpu.reset_program_counter();
        cpu.register_a = 0x40;
        cpu.run();
        assert_eq!(cpu.register_a, 0x80);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(cpu.status.intersects(Status::NEGATIVE));
        assert!(!cpu.status.intersects(Status::CARRY));
        assert!(cpu.status.intersects(Status::OVERFLOW));
    }
}
