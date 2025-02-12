use crate::{OpCode, Status, CPU};

pub const BVS: u8 = 0x70;

/// If the overflow flag is set then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bvs(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.branch(cpu.status.intersects(Status::OVERFLOW));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bvs() {
        let mut cpu = CPU::new();
        cpu.load(&[BVS, 0x01, INX, INX, BRK]);
        cpu.reset();

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
