use crate::{Instruction, Mem, OpCode, CPU};

pub const SXA_ABSOLUTEY: u8 = 0x9E;

/// A logical AND is performed, bit by bit, on the X register contents
/// using the contents of the high byte of target address + 1,
/// storing the result in memory.
#[derive(Debug)]
pub struct InstructionSXA {
    addr: u16,
}

impl OpCode for InstructionSXA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::SXA(Self {
            addr: cpu.get_operand_address().0,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let [_, hi] = self.addr.to_le_bytes();
        let result = cpu.register_x & hi.wrapping_add(1);
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
        let mut cpu = CPU::new_test(&[SXA_ABSOLUTEY, 0x01, 0x03, BRK]);
        cpu.register_y = 0x04;

        // SXA
        cpu.register_x = 0b1010_0101;
        cpu.run();
        assert_eq!(cpu.mem_read(0x0305), 0b0100);
    }
}
