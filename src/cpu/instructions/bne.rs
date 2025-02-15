use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const BNE: u8 = 0xD0;

/// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBNE {
    skip: i8,
}

impl OpCode for InstructionBNE {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BNE(Self {
            skip: cpu.get_operand_address() as i8,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.skip, !cpu.status.intersects(Status::ZERO));
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
