use crate::{OpCode, Status, CPU};

pub const CLV: u8 = 0xB8;

/// Clears the overflow flag.
pub fn clv(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.status.remove(Status::OVERFLOW);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clv() {
        let mut cpu = CPU::new();
        cpu.load(&[CLV, BRK]);
        cpu.reset();
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert!(!cpu.status.intersects(Status::OVERFLOW))
    }
}
