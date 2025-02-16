use crate::{Instruction, OpCode, CPU};

pub const RTS: u8 = 0x60;

/// The RTS instruction is used at the end of a subroutine to return to the calling routine.
/// It pulls the program counter (minus one) from the stack.
#[derive(Debug)]
pub struct InstructionRTS;

impl OpCode for InstructionRTS {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::RTS(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.program_counter = cpu.stack_pull_u16().wrapping_add(1);
    }

    fn cycles(&self) -> u8 {
        6
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::{BRK, INX},
        PROGRAM,
    };

    use super::*;

    #[test]
    fn rts() {
        // Setup
        let mut cpu = CPU::new_test(&[RTS, INX, INX, BRK, INX, INX, INX, INX, BRK]);
        cpu.stack_push_u16(PROGRAM + 1);

        // Jump
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}
