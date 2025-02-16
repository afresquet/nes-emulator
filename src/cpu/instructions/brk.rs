use crate::{Instruction, OpCode, Status, CPU};

pub const BRK: u8 = 0x00;

/// The BRK instruction forces the generation of an interrupt request.
/// The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
#[derive(Debug)]
pub struct InstructionBRK;

impl OpCode for InstructionBRK {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::BRK(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.stack_push_u16(cpu.program_counter);
        cpu.stack_push(cpu.status.bits());
        cpu.status.insert(Status::BREAK_COMMAND);
    }

    fn cycles(&self) -> u8 {
        7
    }
}

#[cfg(test)]
mod tests {

    use crate::{Status, PROGRAM};

    use super::*;

    #[test]
    fn brk() {
        // Setup
        let mut cpu = CPU::new_test(&[BRK]);

        // Break
        cpu.run();
        let status = cpu.stack_pull();
        assert_eq!(Status::from_bits_retain(status), Status::UNUSED);
        let program_counter = cpu.stack_pull_u16();
        assert_eq!(program_counter, PROGRAM + 1);
        assert!(cpu.status.intersects(Status::BREAK_COMMAND));
    }
}
