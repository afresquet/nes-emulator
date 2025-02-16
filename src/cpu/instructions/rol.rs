use crate::{AddressingMode, Instruction, Mem, OpCode, Status, CPU};

pub const ROL_ACCUMULATOR: u8 = 0x2A;
pub const ROL_ZEROPAGE: u8 = 0x26;
pub const ROL_ZEROPAGEX: u8 = 0x36;
pub const ROL_ABSOLUTE: u8 = 0x2E;
pub const ROL_ABSOLUTEX: u8 = 0x3E;

/// Move each of the bits in either A or M one place to the left.
/// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
#[derive(Debug)]
pub struct InstructionROL {
    addr: Option<u16>,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionROL {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let addr = (cpu.current_instruction_register != ROL_ACCUMULATOR)
            .then(|| cpu.get_operand_address().0);

        Instruction::ROL(Self {
            addr,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let value = self
            .addr
            .map(|addr| cpu.mem_read(addr))
            .unwrap_or(cpu.register_a);

        let bit_zero = cpu.status.intersects(Status::CARRY);
        let carry = value & 0b1000_0000;

        let shifted = value << 1 | bit_zero as u8;

        cpu.status.set(Status::CARRY, carry > 0);

        match self.addr {
            Some(addr) => {
                cpu.mem_write(addr, shifted);
                cpu.update_zero_and_negative_flags(shifted);
            }
            None => {
                cpu.register_a = shifted;
                cpu.update_negative_flag(shifted);
            }
        }

        cpu.update_zero_and_negative_flags(shifted);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::Accumulator => 2,
            AddressingMode::ZeroPage => 5,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 6,
            AddressingMode::AbsoluteX => 7,
            _ => unreachable!(),
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
            let mut cpu = CPU::new_test(&[ROL_ACCUMULATOR, BRK]);

            // Shift
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
            let mut cpu = CPU::new_test(&[instruction, addr, BRK]);
            cpu.register_x = 0x10;

            // Shift
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
