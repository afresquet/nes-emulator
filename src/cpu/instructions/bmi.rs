use crate::{Instruction, OpCode, Status, CPU};

pub const BMI: u8 = 0x30;

/// If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBMI {
    target: u16,
    condition: bool,
    page_crossed: bool,
}

impl OpCode for InstructionBMI {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (target, page_crossed) = cpu.get_operand_address();
        Instruction::BMI(Self {
            target,
            condition: cpu.status.contains(Status::NEGATIVE),
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
    fn bmi() {
        let mut cpu = CPU::new_test(&[BMI, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::NEGATIVE);
        cpu.run();
        assert_eq!(cpu.register_x, 1);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 2);
    }
}
