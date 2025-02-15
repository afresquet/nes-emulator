use crate::{Bus, OpCode, Rom, Status, CPU};

use super::Instruction;

pub const BRK: u8 = 0x00;

/// The BRK instruction forces the generation of an interrupt request.
/// The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
#[derive(Debug)]
pub struct InstructionBRK;

impl OpCode for InstructionBRK {
    fn fetch(_cpu: &mut CPU<Bus<Rom>>) -> Instruction {
        Instruction::BRK(Self)
    }

    fn execute(self, cpu: &mut CPU<Bus<Rom>>) {
        cpu.stack_push_u16(cpu.program_counter);
        cpu.stack_push(cpu.status.bits());
        cpu.status.insert(Status::BREAK_COMMAND);
    }
}

#[cfg(test)]
mod tests {

    use crate::{Status, PROGRAM};

    use super::*;

    #[test]
    fn brk() {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[BRK]);

        // Break
        cpu.run();
        let status = cpu.stack_pull();
        assert_eq!(Status::from_bits_retain(status), Status::UNUSED);
        let program_counter = cpu.stack_pull_u16();
        assert_eq!(program_counter, PROGRAM + 1);
        assert!(cpu.status.intersects(Status::BREAK_COMMAND));
    }
}
