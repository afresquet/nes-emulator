use crate::{AddressingMode, Bus, Mem, OpCode, Rom, Status, CPU};

use super::Address;

pub const ROR_ACCUMULATOR: u8 = 0x6A;
pub const ROR_ZEROPAGE: u8 = 0x66;
pub const ROR_ZEROPAGEX: u8 = 0x76;
pub const ROR_ABSOLUTE: u8 = 0x6E;
pub const ROR_ABSOLUTEX: u8 = 0x7E;

/// Move each of the bits in either A or M one place to the right.
/// Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
pub fn ror(cpu: &mut CPU<Bus<Rom>>, opcode: &OpCode) {
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

    let bit_seven = cpu.status.intersects(Status::CARRY);
    let carry = addr.value() & 1;

    let shifted = addr.value() >> 1 | (bit_seven as u8) << 7;

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
    mod ror {
        use test_case::test_case;

        use crate::instructions::BRK;

        use super::super::*;

        #[test]
        fn accumulator() {
            // Setup
            let mut cpu = CPU::new().insert_test_rom(&[ROR_ACCUMULATOR, BRK]);

            // Shift
            cpu.register_a = 0b1000_0101;
            cpu.run();
            assert_eq!(cpu.register_a, 0b0100_0010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Shift with Carry
            cpu.reset();
            cpu.register_a = 0b1000_0101;
            cpu.status.insert(Status::CARRY);
            cpu.run();
            assert_eq!(cpu.register_a, 0b1100_0010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Carry Flag
            cpu.reset();
            cpu.register_a = 0b0101;
            cpu.run();
            assert_eq!(cpu.register_a, 0b0010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Zero Flag
            cpu.reset();
            cpu.register_a = 0b0001;
            cpu.run();
            assert_eq!(cpu.register_a, 0);
            assert!(cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Negative Flag
            cpu.reset();
            cpu.status.insert(Status::CARRY);
            cpu.run();
            assert_eq!(cpu.register_a, 0b1000_0000);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));
        }

        #[test_case(ROR_ZEROPAGE, 0x40 ; "zero_page")]
        #[test_case(ROR_ZEROPAGEX, 0x30 ; "zero_page_x")]
        #[test_case(ROR_ABSOLUTE, 0x40 ; "absolute")]
        #[test_case(ROR_ABSOLUTEX, 0x30 ; "absolute_x")]
        fn memory(instruction: u8, addr: u8) {
            // Setup
            let mut cpu = CPU::new().insert_test_rom(&[instruction, addr, BRK]);
            cpu.register_x = 0x10;

            // Shift
            cpu.mem_write(0x40, 0b1000_0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b0100_0010);
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Shift with Carry
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b1000_0101);
            cpu.status.insert(Status::CARRY);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1100_0010);
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Carry Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b0010);
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Negative Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0);
            cpu.status.insert(Status::CARRY);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b1000_0000);
            assert!(cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));
        }
    }
}
