use crate::{Instruction, Mem, OpCode, CPU};

pub const XAS_ABSOLUTEY: u8 = 0x9B;

/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of the X register,
/// storing the result in the stack pointer. Then AND the stack pointer with the high byte of the target address + 1,
/// storing the result in memory.
#[derive(Debug)]
pub struct InstructionXAS {
    addr: u16,
}

impl OpCode for InstructionXAS {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::XAS(Self {
            addr: cpu.get_operand_address().0,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let [_, hi] = self.addr.to_le_bytes();
        cpu.stack_pointer = cpu.register_a & cpu.register_x;
        let result = cpu.stack_pointer & hi.wrapping_add(1);
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
    fn lar() {
        // Setup
        let mut cpu = CPU::new_test(&[XAS_ABSOLUTEY, 0x01, 0x03, BRK]);
        cpu.register_y = 0x04;

        // XAS
        cpu.register_a = 0b0110_1101;
        cpu.register_x = 0b1010_0100;
        cpu.run();
        assert_eq!(cpu.stack_pointer, 0b0010_0100 - 3 /* from BRK */);
        assert_eq!(cpu.mem_read(0x0305), 0b0100);
    }
}
