use crate::{OpCode, Status, CPU};

use super::Instruction;

pub const BPL: u8 = 0x10;

/// If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBPL {
    skip: i8,
}

impl OpCode for InstructionBPL {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BPL(Self {
            skip: cpu.get_operand_address() as i8,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.branch(self.skip, !cpu.status.intersects(Status::NEGATIVE));
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bpl() {
        let mut cpu = CPU::new_test(&[BPL, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::NEGATIVE);
        cpu.run();
        assert_eq!(cpu.register_x, 2);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}
