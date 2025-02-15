use crate::{Bus, Mem, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const LSR_ACCUMULATOR: u8 = 0x4A;
pub const LSR_ZEROPAGE: u8 = 0x46;
pub const LSR_ZEROPAGEX: u8 = 0x56;
pub const LSR_ABSOLUTE: u8 = 0x4E;
pub const LSR_ABSOLUTEX: u8 = 0x5E;

/// Each of the bits in A or M is shift one place to the right.
/// The bit that was in bit 0 is shifted into the carry flag.
/// Bit 7 is set to zero.
#[derive(Debug)]
pub struct InstructionLSR {
    addr: Option<u16>,
}

impl OpCode for InstructionLSR {
    fn fetch(cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        let addr = (cpu.current_instruction_register != LSR_ACCUMULATOR)
            .then(|| cpu.get_operand_address());

        Instruction::LSR(Self { addr })
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        let value = self
            .addr
            .map(|addr| cpu.mem_read(addr))
            .unwrap_or(cpu.register_a);

        cpu.status.set(Status::CARRY, value & 1 != 0);

        let shifted = value >> 1;

        match self.addr {
            Some(addr) => {
                cpu.mem_write(addr, shifted);
            }
            None => {
                cpu.register_a = shifted;
            }
        }

        cpu.update_zero_and_negative_flags(shifted);
    }
}

#[cfg(test)]
mod tests {
    mod lsr {
        use test_case::test_case;

        use crate::instructions::BRK;

        use super::super::*;

        #[test]
        fn accumulator() {
            // Setup
            let mut cpu = CPU::new().insert_test_rom(&[LSR_ACCUMULATOR, BRK]);

            // Shift
            cpu.register_a = 0b1010;
            cpu.run();
            assert_eq!(cpu.register_a, 0b0101);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));

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
        }

        #[test_case(LSR_ZEROPAGE, 0x40 ; "zero_page")]
        #[test_case(LSR_ZEROPAGEX, 0x30 ; "zero_page_x")]
        #[test_case(LSR_ABSOLUTE, 0x40 ; "absolute")]
        #[test_case(LSR_ABSOLUTEX, 0x30 ; "absolute_x")]
        fn memory(instruction: u8, addr: u8) {
            // Setup
            let mut cpu = CPU::new().insert_test_rom(&[instruction, addr, BRK]);
            cpu.register_x = 0x10;

            // Shift
            cpu.mem_write(0x40, 0b1010);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b0101);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(!cpu.status.intersects(Status::CARRY));

            // Carry Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b0101);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0b0010);
            assert!(!cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));

            // Zero Flag
            cpu.reset_status();
            cpu.reset_program_counter();
            cpu.mem_write(0x40, 0b0001);
            cpu.run();
            assert_eq!(cpu.mem_read(0x40), 0);
            assert!(cpu.status.intersects(Status::ZERO));
            assert!(!cpu.status.intersects(Status::NEGATIVE));
            assert!(cpu.status.intersects(Status::CARRY));
        }
    }
}
