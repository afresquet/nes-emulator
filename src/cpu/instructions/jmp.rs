use crate::{OpCode, CPU};

pub const JMP_ABSOLUTE: u8 = 0x4C;
pub const JMP_INDIRECT: u8 = 0x6C;

/// Sets the program counter to the address specified by the operand.
pub fn jmp(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.program_counter = addr;
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use crate::instructions::{BRK, INX};

    use super::*;

    #[test_case(JMP_ABSOLUTE, 0x04, 0x80 ; "absolute")]
    #[test_case(JMP_INDIRECT, 0x00, 0x10 ; "indirect")]
    fn jmp(instruction: u8, lo: u8, hi: u8) {
        // Setup
        let mut cpu = CPU::new();
        cpu.load(&[instruction, lo, hi, INX, INX, BRK, INX, INX, INX, INX, BRK]);
        cpu.reset();
        cpu.mem_write_u16(0x1000, 0x8004);

        // Jump
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
}
