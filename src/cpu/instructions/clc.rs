use crate::{OpCode, Status, CPU};

pub const CLC: u8 = 0x18;

/// Set the carry flag to zero.
pub fn clc(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.status.remove(Status::CARRY);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clc() {
        let mut cpu = CPU::new();
        cpu.load(&[CLC, BRK]);
        cpu.reset();
        cpu.status.insert(Status::CARRY);
        cpu.run();
        assert!(!cpu.status.intersects(Status::CARRY))
    }
}
