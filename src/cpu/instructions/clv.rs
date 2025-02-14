use crate::{Bus, OpCode, Rom, Status, CPU};

pub const CLV: u8 = 0xB8;

/// Clears the overflow flag.
pub fn clv(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status.remove(Status::OVERFLOW);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn clv() {
        let mut cpu = CPU::new().insert_test_rom(&[CLV, BRK]);
        cpu.status.insert(Status::OVERFLOW);
        cpu.run();
        assert!(!cpu.status.intersects(Status::OVERFLOW))
    }
}
