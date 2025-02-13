use crate::{OpCode, Status, CPU};

pub const SEI: u8 = 0x78;

/// Set the interrupt disable flag to one.
pub fn sei(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.status.insert(Status::INTERRUPT_DISABLE);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sei() {
        let mut cpu = CPU::new();
        cpu.load(&[SEI, BRK]);
        cpu.reset();
        cpu.run();
        assert!(cpu.status.intersects(Status::INTERRUPT_DISABLE));
    }
}
