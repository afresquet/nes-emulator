use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const RTI: u8 = 0x40;

/// The RTI instruction is used at the end of an interrupt processing routine.
/// It pulls the processor flags from the stack followed by the program counter.
#[derive(Debug)]
pub struct InstructionRTI;

impl OpCode for InstructionRTI {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::RTI(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status = Status::from_bits_retain(cpu.stack_pull());
        cpu.program_counter = cpu.stack_pull_u16();
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        instructions::{BRK, PHP},
        Status, PROGRAM,
    };

    use super::*;

    #[test]
    fn rti() {
        // Setup
        let mut cpu = CPU::new_test(&[RTI, BRK, PHP, BRK]);
        cpu.stack_push_u16(PROGRAM + 2);
        cpu.stack_push(0b1010_1010);

        // Break
        cpu.run();
        assert_eq!(cpu.stack_pull(), 0b1010_1010);
        assert_eq!(cpu.status, Status::from_bits_retain(0b1011_1010))
    }
}
