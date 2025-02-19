use crate::{Instruction, Mem, OpCode, CPU};

pub const SYA_ABSOLUTEX: u8 = 0x9C;

/// A logical AND is performed, bit by bit, on the Y register contents
/// using the contents of the high byte of target address + 1,
/// storing the result in memory.
#[derive(Debug)]
pub struct InstructionSYA {
    addr: u16,
}

impl OpCode for InstructionSYA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::SHY(Self {
            addr: cpu.get_operand_address().0,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let [_, hi] = self.addr.to_le_bytes();
        let result = cpu.register_y & hi.wrapping_add(1);
        cpu.mem_write(self.addr, result);
    }

    fn cycles(&self) -> u8 {
        5
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sxa() {
        let mut cpu = CPU::new_test(&[SYA_ABSOLUTEX, 0x02, 0x03, BRK]);
        cpu.register_x = 0x03;

        // SYA
        cpu.register_y = 0b1010_0101;
        cpu.run();
        assert_eq!(cpu.mem_read(0x0305), 0b0100);
    }
}
