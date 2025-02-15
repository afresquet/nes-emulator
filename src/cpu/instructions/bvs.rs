use crate::{Instruction, OpCode, Status, CPU};

pub const BVS: u8 = 0x70;

/// If the overflow flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[derive(Debug)]
pub struct InstructionBVS {
    skip: i8,
    condition: bool,
}

impl OpCode for InstructionBVS {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::BVS(Self {
            skip: cpu.get_operand_address() as i8,
            condition: cpu.status.intersects(Status::OVERFLOW),
        })
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.branch(self.skip, self.condition);
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
    fn bvs() {
        let mut cpu = CPU::new_test(&[BVS, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert_eq!(cpu.register_x, 1);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 2);
    }
}
