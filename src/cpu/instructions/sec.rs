use crate::{Bus, OpCode, Rom, Status, CPU};

pub const SEC: u8 = 0x38;

/// Set the carry flag to one.
pub fn sec(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status.insert(Status::CARRY);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sec() {
        let mut cpu = CPU::new().insert_test_rom(&[SEC, BRK]);
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
    }
}
