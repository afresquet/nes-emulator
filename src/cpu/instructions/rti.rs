use crate::{Bus, OpCode, Rom, Status, CPU};

pub const RTI: u8 = 0x40;

/// The RTI instruction is used at the end of an interrupt processing routine.
/// It pulls the processor flags from the stack followed by the program counter.
pub fn rti(cpu: &mut CPU<Bus<Rom>>, _opcode: &OpCode) {
    cpu.status = Status::from_bits_retain(cpu.stack_pull());
    cpu.program_counter = cpu.stack_pull_u16();
}

#[cfg(test)]
mod tests {

    use crate::{
        instructions::{BRK, INX},
        Status, PROGRAM,
    };

    use super::*;

    #[test]
    fn rti() {
        // Setup
        let mut cpu = CPU::new().insert_test_rom(&[RTI, BRK, INX, BRK]);
        cpu.stack_push_u16(PROGRAM + 2);
        cpu.stack_push(0b0101_0101);

        // Break
        cpu.run();
        assert_eq!(cpu.register_x, 1);
        assert_eq!(cpu.status, Status::from_bits_retain(0b0101_0101))
    }
}
