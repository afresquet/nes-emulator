use crate::{Bus, OpCode, Rom, Status, CPU};

pub const BCS: u8 = 0xB0;

/// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bcs(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.branch(cpu.status.intersects(Status::CARRY));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bcs() {
        let mut cpu = CPU::new().insert_test_rom(&[BCS, 0x01, INX, INX, BRK]);

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
