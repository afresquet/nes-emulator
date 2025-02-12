use crate::{OpCode, Status, CPU};

pub const BRK: u8 = 0x00;

/// The BRK instruction forces the generation of an interrupt request.
/// The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
pub fn brk(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.status.insert(Status::BREAK_COMMAND);
}

#[cfg(test)]
mod tests {
    use crate::Status;

    use super::*;

    #[test]
    fn brk() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[BRK]);
        assert_eq!(cpu.program_counter, 0x8001);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status, Status::UNUSED | Status::BREAK_COMMAND);
    }
}
