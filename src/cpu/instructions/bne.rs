use crate::{Bus, OpCode, Rom, Status, CPU};

pub const BNE: u8 = 0xD0;

/// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bne(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.branch(!cpu.status.intersects(Status::ZERO));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn bne() {
        let mut cpu = CPU::new().insert_test_rom(&[BNE, 0x01, INX, INX, BRK]);

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
