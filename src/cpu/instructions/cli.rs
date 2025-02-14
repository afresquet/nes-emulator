use crate::{Bus, OpCode, Rom, Status, CPU};

pub const CLI: u8 = 0x58;

/// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
pub fn cli(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status.remove(Status::INTERRUPT_DISABLE);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn cli() {
        let mut cpu = CPU::new().insert_test_rom(&[CLI, BRK]);
        cpu.status.insert(Status::INTERRUPT_DISABLE);
        cpu.run();
        assert!(!cpu.status.intersects(Status::INTERRUPT_DISABLE))
    }
}
