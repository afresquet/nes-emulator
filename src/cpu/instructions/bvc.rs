use crate::{Bus, OpCode, Rom, Status, CPU};

pub const BVC: u8 = 0x50;

/// If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bvc(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.branch(!cpu.status.intersects(Status::OVERFLOW));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bvc() {
        let mut cpu = CPU::new().insert_test_rom(&[BVC, 0x01, INX, INX, BRK]);

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
