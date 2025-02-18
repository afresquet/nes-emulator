use crate::{Instruction, OpCode, Status, CPU};

pub const BNE: u8 = 0xD0;

/// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBNE {
    target: u16,
    condition: bool,
    page_crossed: bool,
}

impl OpCode for InstructionBNE {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (target, page_crossed) = cpu.get_operand_address();
        Instruction::BNE(Self {
            target,
            condition: !cpu.status.contains(Status::ZERO),
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
    fn bne() {
        let mut cpu = CPU::new_test(&[BNE, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::ZERO);
        cpu.run();
        assert_eq!(cpu.register_x, 2);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}
