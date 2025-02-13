use crate::{OpCode, Status, CPU};

pub const SEC: u8 = 0x38;

/// Set the carry flag to one.
pub fn sec(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.status.insert(Status::CARRY);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sec() {
        let mut cpu = CPU::new();
        cpu.load(&[SEC, BRK]);
        cpu.reset();
        cpu.run();
        assert!(cpu.status.intersects(Status::CARRY));
    }
}
