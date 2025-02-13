use crate::{AddressingMode, Mem, OpCode, Status, CPU};

use super::Address;

pub const ROL_ACCUMULATOR: u8 = 0x2A;
pub const ROL_ZEROPAGE: u8 = 0x26;
pub const ROL_ZEROPAGEX: u8 = 0x36;
pub const ROL_ABSOLUTE: u8 = 0x2E;
pub const ROL_ABSOLUTEX: u8 = 0x3E;

/// Move each of the bits in either A or M one place to the left.
/// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
pub fn rol(cpu: &mut CPU, opcode: &OpCode) {
    let addr = match opcode.mode {
        AddressingMode::Accumulator => Address::Accumulator(cpu.register_a),
        mode => {
            let addr = cpu.get_operand_address(mode);
            Address::Memory {
                addr,
                value: cpu.mem_read(addr),
            }
        }
    };

    let bit_zero = cpu.status.intersects(Status::CARRY);
    let carry = addr.value() & 0b1000_0000;

    let shifted = addr.value() << 1 | bit_zero as u8;

    cpu.status.set(Status::CARRY, carry > 0);

    match addr {
        Address::Accumulator(_) => {
            cpu.register_a = shifted;
            cpu.update_zero_and_negative_flags(shifted);
        }
        Address::Memory { addr, .. } => {
            cpu.mem_write(addr, shifted);
            cpu.update_negative_flag(shifted);
        }
    }
}

#[cfg(test)]
mod tests {
    mod rol {
        use test_case::test_case;

        use crate::instructions::BRK;

        use super::super::*;

        #[test]
        fn accumulator() {
            // Setup
            let mut cpu = CPU::new();
            cpu.load(&[ROL_ACCUMULATOR, BRK]);

            // Shift
            cpu.reset();
            cpu.register_a = 0b1010_0101;
            cpu.run();
            assert_eq!(cpu.register_a, 0b0100_1010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Shift with Carry
            cpu.reset();
            cpu.register_a = 0b1010_0101;
            cpu.status.insert(Status::CARRY);
            cpu.run();
            assert_eq!(cpu.register_a, 0b0100_1011);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

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

        #[test_case(ROL_ZEROPAGE, 0x40 ; "zero_page")]
        #[test_case(ROL_ZEROPAGEX, 0x30 ; "zero_page_x")]
        #[test_case(ROL_ABSOLUTE, 0x40 ; "absolute")]
        #[test_case(ROL_ABSOLUTEX, 0x30 ; "absolute_x")]
        fn memory(instruction: u8, addr: u8) {
            // Setup
            let mut cpu = CPU::new();
            cpu.load(&[instruction, addr, BRK]);
            cpu.register_x = 0x10;

            // Shift
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b1010_0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b0100_1010);
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Shift with Carry
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b1010_0101);
            cpu.status.insert(Status::CARRY);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b0100_1011);
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Carry Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b1000_0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1010);
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Negative Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b0100_0000);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1000_0000);
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));
        }
    }
}
