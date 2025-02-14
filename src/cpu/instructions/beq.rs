use crate::{Bus, OpCode, Rom, Status, CPU};

pub const BEQ: u8 = 0xF0;

/// If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
pub fn beq(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.branch(cpu.status.intersects(Status::ZERO));
}

#[cfg(test)]
mod tests {
    use crate::instructions::{BRK, INX};

    use super::*;

    #[test]
    fn beq() {
        let mut cpu = CPU::new().insert_test_rom(&[BEQ, 0x01, INX, INX, BRK]);

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
