use crate::{Instruction, OpCode, Status, CPU};

pub const PLP: u8 = 0x28;

/// Pulls an 8 bit value from the stack and into the processor flags.
/// The flags will take on new states as determined by the value pulled, removing the B flag.
#[derive(Debug)]
pub struct InstructionPLP;

impl OpCode for InstructionPLP {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::PLP(Self)
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.status = Status::from_bits_truncate(cpu.stack_pull());
        cpu.status.remove(Status::BREAK_COMMAND);
        cpu.status.insert(Status::UNUSED);
    }

    fn cycles(&self) -> u8 {
        4
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, STACK_SIZE};

    use super::*;

    #[test]
    fn plp() {
        // Setup
        let mut cpu = CPU::new_test(&[PLP, BRK]);
        cpu.stack_push(0b0101_0101);

        // Push
        cpu.run();
        assert_eq!(cpu.status, Status::from_bits_truncate(0b0111_0101));
    }

    #[test]
    #[should_panic = "STACK OVERFLOW"]
    fn stack_overflow() {
        let mut cpu = CPU::new_test(&[PLP, BRK]);
        cpu.stack_pointer = STACK_SIZE;
        cpu.run();
    }
}
