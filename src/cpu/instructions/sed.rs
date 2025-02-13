use crate::{OpCode, Status, CPU};

pub const SED: u8 = 0xF8;

/// Set the decimal mode flag to one.
pub fn sed(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.status.insert(Status::DECIMAL);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sed() {
        let mut cpu = CPU::new();
        cpu.load(&[SED, BRK]);
        cpu.reset();
        cpu.run();
        assert!(cpu.status.intersects(Status::DECIMAL));
    }
}
