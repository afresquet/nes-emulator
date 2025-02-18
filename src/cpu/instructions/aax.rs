use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const AAX_ZEROPAGE: u8 = 0x87;
pub const AAX_ZEROPAGEY: u8 = 0x97;
pub const AAX_ABSOLUTE: u8 = 0x83;
pub const AAX_INDIRECTX: u8 = 0x8F;

/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of the X register.
/// The result is stored in memory.
#[derive(Debug)]
pub struct InstructionAAX {
    pub addr: u16,
    pub addressing_mode: AddressingMode,
}

impl OpCode for InstructionAAX {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::AAX(Self {
            addr: cpu.get_operand_address().0,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let result = cpu.register_a & cpu.register_x;
        cpu.mem_write(self.addr, result);
        cpu.update_zero_and_negative_flags(result);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageY | AddressingMode::Absolute => 4,
            AddressingMode::IndirectX => 6,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(AAX_ZEROPAGE, 0x10, 0x10 ; "zero_page")]
    #[test_case(AAX_ZEROPAGEY, 0x0C, 0x0C ; "zero_page_y")]
    #[test_case(AAX_ABSOLUTE, 0x10, 0x10 ; "absolute")]
    #[test_case(AAX_INDIRECTX, 0x16, 0xA0 ; "indirect_x")]
    fn aax(instruction: u8, addr: u8, negative: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, addr, BRK]);
        cpu.register_y = 0x04;
        cpu.mem_write(0x20, 0x10);

        // AAX
        cpu.register_a = 0b1000_1010;
        cpu.register_x = 0b0000_1010;
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0b1010);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, addr, BRK]);
        cpu.reset_status();
        cpu.register_a = 0;
        cpu.register_x = 0b1010;
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, negative, BRK]);
        cpu.reset_status();
        cpu.register_a = 0b1000_1010;
        cpu.register_x = 0b1000_0000;
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0b1000_0000);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
