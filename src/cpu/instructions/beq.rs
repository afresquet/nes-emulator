use crate::{Instruction, OpCode, Status, CPU};

pub const BEQ: u8 = 0xF0;

/// If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBEQ {
    target: u16,
    condition: bool,
}

impl OpCode for InstructionBEQ {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BEQ(Self {
            target: cpu.get_operand_address(),
            condition: cpu.status.intersects(Status::ZERO),
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
    fn beq() {
        let mut cpu = CPU::new_test(&[BEQ, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::ZERO);
        cpu.run();
        assert_eq!(cpu.register_x, 1);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 2);
    }
}
