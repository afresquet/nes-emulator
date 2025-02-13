use crate::{OpCode, CPU};

pub const TXS: u8 = 0x9A;

/// Copies the current contents of the X register into the stack register.
pub fn txs(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.stack_push(cpu.register_x);
}

#[cfg(test)]
mod tests {
    use crate::instructions::BRK;

    use super::*;

    #[test]
    fn txs() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[TXS, BRK]);
        cpu.reset();
        cpu.register_x = 0x05;

        // Transfer
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull(), 0x05);
    }
}
