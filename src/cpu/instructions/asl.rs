use crate::{AddressingMode, OpCode, Status, CPU};

pub const ASL_ACCUMULATOR: u8 = 0x0A;
pub const ASL_ZEROPAGE: u8 = 0x06;
pub const ASL_ZEROPAGEX: u8 = 0x16;
pub const ASL_ABSOLUTE: u8 = 0x0E;
pub const ASL_ABSOLUTEX: u8 = 0x1E;

/// This operation shifts all the bits of the accumulator or memory contents one bit left.
/// Bit 0 is set to 0 and bit 7 is placed in the carry flag.
/// The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
pub fn asl(cpu: &mut CPU, opcode: &OpCode) {
    let ptr = match opcode.mode {
        AddressingMode::Accumulator => &mut cpu.register_a,
        mode => {
            let addr = cpu.get_operand_address(mode);
            &mut cpu.memory[addr as usize]
        }
    };

    let shifted = (*ptr as u16) << 1;

    cpu.status.set(Status::CARRY, shifted > u8::MAX as u16);

    *ptr = shifted as u8;

    cpu.update_zero_and_negative_flags(shifted as u8);
}

#[cfg(test)]
mod tests {
    mod asl {
        use test_case::test_case;

        use crate::instructions::BRK;

        use super::super::*;

        #[test]
        fn accumulator() {
            // Setup
            let mut cpu = CPU::new();
            cpu.load(&[ASL_ACCUMULATOR, BRK]);

            // Shift
            cpu.reset();
            cpu.register_a = 0b0101;
            cpu.run();
            assert_eq!(cpu.register_a, 0b1010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));

            // Carry Flag
            cpu.reset();
            cpu.register_a = 0b1000_0101;
            cpu.run();
            assert_eq!(cpu.register_a, 0b1010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Zero Flag
            cpu.reset();
            cpu.register_a = 0b1000_0000;
            cpu.run();
            assert_eq!(cpu.register_a, 0);
            assert!(cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Negative Flag
            cpu.reset();
            cpu.register_a = 0b0100_0000;
            cpu.run();
            assert_eq!(cpu.register_a, 0b1000_0000);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));
        }

        #[test_case(ASL_ZEROPAGE, 0x40 ; "zero_page")]
        #[test_case(ASL_ZEROPAGEX, 0x30 ; "zero_page_x")]
        #[test_case(ASL_ABSOLUTE, 0x40 ; "absolute")]
        #[test_case(ASL_ABSOLUTEX, 0x30 ; "absolute_x")]
        fn memory(instruction: u8, addr: u8) {
            // Setup
            let mut cpu = CPU::new();
            cpu.load(&[instruction, addr, BRK]);
            cpu.register_x = 0x10;

            // Shift
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));

            // Carry Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b1000_0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Zero Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b1000_0000);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0);
            assert!(cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Negative Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b0100_0000);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1000_0000);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));
        }
    }
}
