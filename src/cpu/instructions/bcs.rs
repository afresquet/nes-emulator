use crate::{Instruction, OpCode, Status, CPU};

pub const BCS: u8 = 0xB0;

/// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBCS {
    target: u16,
    condition: bool,
}

impl OpCode for InstructionBCS {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BCS(Self {
            target: cpu.get_operand_address(),
            condition: cpu.status.intersects(Status::CARRY),
        })
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.branch(self.target, self.condition);
        self.cycles(false)
    }

    fn cycles(&self, page_crossed: bool) -> u8 {
        2 + (self.condition as u8 * if page_crossed { 2 } else { 1 })
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bcs() {
        let mut cpu = CPU::new_test(&[BCS, 0x01, INX, INX, BRK]);

        // Carry Flag Set
        cpu.status.insert(Status::CARRY);
        cpu.run();
        assert_eq!(cpu.register_x, 1);

        // Carry Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 2);
    }
}
