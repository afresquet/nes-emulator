use crate::{Bus, OpCode, Rom, CPU};

/// The NOP instruction causes no changes to the processor other than the normal incrementing of the program counter to the next instruction.
pub const NOP: u8 = 0xEA;

pub fn nop(_cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status, PROGRAM};

    use super::*;

    #[test]
    fn nop() {
        let mut cpu = CPU::new().insert_test_rom(&[NOP, BRK]);
        cpu.run();
        assert_eq!(cpu.program_counter, PROGRAM + 2);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status, Status::UNUSED | Status::BREAK_COMMAND);
    }
}
