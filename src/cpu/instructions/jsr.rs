use crate::{Bus, OpCode, Rom, CPU};

pub const JSR: u8 = 0x20;

/// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
pub fn jsr(cpu: &mut CPU<Bus<Rom>>, opcode: &OpCode) {
    cpu.stack_push_u16(cpu.program_counter.wrapping_add(1));
    cpu.program_counter = cpu.get_operand_address(opcode.mode);
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::{BRK, INX},
        PROGRAM,
    };

    use super::*;

    #[test]
    fn jsr() {
        // Setup
        let [lo, hi] = (PROGRAM + 4).to_le_bytes();
        let mut cpu = CPU::new().insert_test_rom(&[JSR, lo, hi, INX, INX, BRK, INX, INX, INX, BRK]);

        // Jump
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull_u16(), PROGRAM + 2);
        assert_eq!(cpu.register_x, 1);
    }
}
