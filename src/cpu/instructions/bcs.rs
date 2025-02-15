use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const BCS: u8 = 0xB0;

/// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBCS {
    skip: i8,
}

impl OpCode for InstructionBCS {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BCS(Self {
            skip: cpu.get_operand_address() as i8,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.skip, cpu.status.intersects(Status::CARRY));
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
