use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const BEQ: u8 = 0xF0;

/// If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBEQ {
    skip: i8,
}

impl OpCode for InstructionBEQ {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BEQ(Self {
            skip: cpu.get_operand_address() as i8,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.skip, cpu.status.intersects(Status::ZERO));
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
