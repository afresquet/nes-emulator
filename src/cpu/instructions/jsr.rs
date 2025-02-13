use crate::{OpCode, CPU};

pub const JSR: u8 = 0x20;

/// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
pub fn jsr(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.stack_push_u16(cpu.program_counter.wrapping_add(2));
    cpu.program_counter = addr;
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::{BRK, INX},
        PROGRAM, STACK,
    };

    use super::*;

    #[test]
    fn jsr() {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[JSR, 0x04, 0x80, INX, INX, BRK, INX, INX, INX, INX, BRK]);
        cpu.reset();

        // Jump
        cpu.run();
        let lo = cpu.memory[STACK + cpu.stack_pointer as usize + 5];
        let hi = cpu.memory[STACK + cpu.stack_pointer as usize + 4];
        assert_eq!(u16::from_le_bytes([lo, hi]), PROGRAM + 3);
    }
}
