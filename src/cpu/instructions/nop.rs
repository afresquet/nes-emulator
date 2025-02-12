use crate::{OpCode, CPU};

/// The NOP instruction causes no changes to the processor other than the normal incrementing of the program counter to the next instruction.
pub const NOP: u8 = 0xEA;

pub fn nop(_cpu: &mut CPU, _opcode: &OpCode) {}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn nop() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[NOP, BRK]);
        assert_eq!(cpu.program_counter, 0x8002);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status, Status::UNUSED);
    }
}
