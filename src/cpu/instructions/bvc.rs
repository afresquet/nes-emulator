use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const BVC: u8 = 0x50;

/// If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBVC {
    skip: i8,
}

impl OpCode for InstructionBVC {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BVC(Self {
            skip: cpu.get_operand_address() as i8,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.skip, !cpu.status.intersects(Status::OVERFLOW));
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bvc() {
        let mut cpu = CPU::new_test(&[BVC, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert_eq!(cpu.register_x, 2);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}
