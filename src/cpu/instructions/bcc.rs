use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const BCC: u8 = 0x90;

/// If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBCC {
    skip: i8,
}

impl OpCode for InstructionBCC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BCC(Self {
            skip: cpu.get_operand_address() as i8,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.skip, !cpu.status.intersects(Status::CARRY));
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bcc() {
        let mut cpu = CPU::new_test(&[BCC, 0x01, INX, INX, BRK]);

        // Carry Flag Set
        cpu.status.insert(Status::CARRY);
        cpu.run();
        assert_eq!(cpu.register_x, 2);

        // Carry Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}
