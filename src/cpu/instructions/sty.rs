use crate::{Bus, Mem, OpCode, Rom, CPU};

use super::Instruction;

pub const STY_ZEROPAGE: u8 = 0x84;
pub const STY_ZEROPAGEX: u8 = 0x94;
pub const STY_ABSOLUTE: u8 = 0x8C;

/// Stores the contents of the Y register into memory.
#[derive(Debug)]
pub struct InstructionSTY {
    addr: u16,
}

impl OpCode for InstructionSTY {
    fn fetch(cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::STY(Self {
            addr: cpu.get_operand_address(),
        })
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.mem_write(self.addr, cpu.register_y);
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::instructions::BRK;

    use super::*;

    #[test_case(STY_ZEROPAGE, 0x00, 0x00 ; "zero_page")]
    #[test_case(STY_ZEROPAGEX, 0x00, 0x10 ; "zero_page_x")]
    #[test_case(STY_ABSOLUTE, 0x00, 0x00 ; "absolute")]
    fn sty(instruction: u8, arg: u8, addr: u16) {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[instruction, arg, BRK]);
        cpu.register_x = 0x10;
        cpu.mem_write_u16(0x10, 0x00);

        // Store
        cpu.register_y = 0xFF;
        cpu.run();
        assert_eq!(cpu.mem_read(addr), 0xFF);
    }
}
