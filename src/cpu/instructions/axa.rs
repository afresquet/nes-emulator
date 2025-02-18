use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const AXA_ABSOLUTEY: u8 = 0x9F;
pub const AXA_INDIRECTY: u8 = 0x93;

/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of the register X,
/// then AND the result with 7 and store it in memory.
#[derive(Debug)]
pub struct InstructionAXA {
    addr: u16,
    addressing_mode: AddressingMode,
}

impl OpCode for InstructionAXA {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::AXA(Self {
            addr: cpu.get_operand_address().0,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let result = cpu.register_a & cpu.register_x;
        cpu.mem_write(self.addr, result & 0b0111);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::AbsoluteY => 5,
            AddressingMode::IndirectY => 6,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::instructions::BRK;

    use super::*;

    #[test_case(AXA_ABSOLUTEY, 0x0C ; "absolute_y")]
    #[test_case(AXA_INDIRECTY, 0x20 ; "indirect_y")]
    fn axa(instruction: u8, addr: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, addr, BRK]);
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x20, 0x0C);

        // AXA
        cpu.register_a = 0b1001_0011;
        cpu.register_x = 0b1010_1010;
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0b0010);
    }
}
