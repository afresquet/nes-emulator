use crate::{Instruction, OpCode, CPU};

pub const NOP: u8 = 0xEA;

/// The NOP instruction causes no changes to the processor other than the normal incrementing of the program counter to the next instruction.
#[derive(Debug)]
pub struct InstructionNOP;

impl OpCode for InstructionNOP {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::NOP(Self)
    }

    fn execute(self, _cpu: &mut CPU) {}

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, PROGRAM};

    use super::*;

    #[test]
    fn nop() {
        let mut cpu = CPU::new_test(&[NOP, BRK]);
        cpu.run();
        assert_eq!(cpu.program_counter, PROGRAM + 2);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status, Status::UNUSED | Status::BREAK_COMMAND);
    }
}
