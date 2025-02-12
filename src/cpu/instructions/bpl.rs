use crate::{OpCode, Status, CPU};

pub const BPL: u8 = 0x10;

/// If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bpl(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.branch(!cpu.status.intersects(Status::NEGATIVE));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bpl() {
        let mut cpu = CPU::new();
        cpu.load(&[BPL, 0x01, INX, INX, BRK]);
        cpu.reset();

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
