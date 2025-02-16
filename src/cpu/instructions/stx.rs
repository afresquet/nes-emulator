use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const STX_ZEROPAGE: u8 = 0x86;
pub const STX_ZEROPAGEY: u8 = 0x96;
pub const STX_ABSOLUTE: u8 = 0x8E;

/// Stores the contents of the X register into memory.
#[derive(Debug)]
pub struct InstructionSTX {
    addr: u16,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionSTX {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::STX(Self {
            addr: cpu.get_operand_address().0,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.mem_write(self.addr, cpu.register_x);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::ZeroPage => 3,
            AddressingMode::ZeroPageY | AddressingMode::Absolute => 4,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::instructions::BRK;

    use super::*;

    #[test_case(STX_ZEROPAGE, 0x00, 0x00 ; "zero_page")]
    #[test_case(STX_ZEROPAGEY, 0x00, 0x10 ; "zero_page_y")]
    #[test_case(STX_ABSOLUTE, 0x00, 0x00 ; "absolute")]
    fn stx(instruction: u8, arg: u8, addr: u16) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, arg, BRK]);
        cpu.register_y = 0x10;
        cpu.mem_write_u16(0x10, 0x00);

        // Store
        cpu.register_x = 0xFF;
        cpu.run();
        assert_eq!(cpu.mem_read(addr), 0xFF);
    }
}
