use crate::{Bus, OpCode, Rom, Status, CPU};

pub const CLC: u8 = 0x18;

/// Set the carry flag to zero.
pub fn clc(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status.remove(Status::CARRY);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clc() {
        let mut cpu = CPU::new().insert_test_rom(&[CLC, BRK]);
        cpu.status.insert(Status::CARRY);
        cpu.run();
        assert!(!cpu.status.intersects(Status::CARRY))
    }
}
