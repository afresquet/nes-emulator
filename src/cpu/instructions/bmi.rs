use crate::{Bus, OpCode, Rom, Status, CPU};

pub const BMI: u8 = 0x30;

/// If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bmi(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.branch(cpu.status.intersects(Status::NEGATIVE));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bmi() {
        let mut cpu = CPU::new().insert_test_rom(&[BMI, 0x01, INX, INX, BRK]);

        // Zero Flag Set
        cpu.status.insert(Status::NEGATIVE);
        cpu.run();
        assert_eq!(cpu.register_x, 1);

        // Zero Flag Clear
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_x, 2);
    }
}
