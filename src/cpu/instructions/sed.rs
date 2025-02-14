use crate::{Bus, OpCode, Rom, Status, CPU};

pub const SED: u8 = 0xF8;

/// Set the decimal mode flag to one.
pub fn sed(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status.insert(Status::DECIMAL);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn sed() {
        let mut cpu = CPU::new().insert_test_rom(&[SED, BRK]);
        cpu.run();
        assert!(cpu.status.intersects(Status::DECIMAL));
    }
}
