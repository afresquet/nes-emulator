use crate::{Bus, OpCode, Rom, Status, CPU};

pub const CLD: u8 = 0xD8;

/// Sets the decimal mode flag to zero.
pub fn cld(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status.remove(Status::DECIMAL);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn cld() {
        let mut cpu = CPU::new().insert_test_rom(&[CLD, BRK]);
        cpu.status.insert(Status::DECIMAL);
        cpu.run();
        assert!(!cpu.status.intersects(Status::DECIMAL))
    }
}
