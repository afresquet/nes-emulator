use crate::{Instruction, OpCode, Status, CPU};

pub const BCC: u8 = 0x90;

/// If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBCC {
    target: u16,
    condition: bool,
    page_crossed: bool,
}

impl OpCode for InstructionBCC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (target, page_crossed) = cpu.get_operand_address();
        Instruction::BCC(Self {
            target,
            condition: !cpu.status.contains(Status::CARRY),
            page_crossed,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.target, self.condition);
    }

    fn cycles(&self) -> u8 {
        2 + (self.condition as u8 * if self.page_crossed { 2 } else { 1 })
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
