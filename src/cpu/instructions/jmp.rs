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
    mod jmp {
        use crate::{
            instructions::{BRK, INX},
            PROGRAM,
        };

        use super::super::*;

        #[test]
        fn absolute() {
            // Setup
            let mut cpu = CPU::new();
            let [lo, hi] = (PROGRAM + 4).to_le_bytes();
            cpu.load(&[JMP_ABSOLUTE, lo, hi, INX, INX, BRK, INX, INX, INX, INX, BRK]);
            cpu.reset();
            cpu.mem_write_u16(0x1000, PROGRAM + 4);

            // Jump
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }

        #[test]
        fn indirect() {
            // Setup
            let mut cpu = CPU::new();
            cpu.load(&[JMP_INDIRECT, 0, 0, INX, INX, BRK, INX, INX, INX, INX, BRK]);
            cpu.reset();
            cpu.mem_write_u16(0, PROGRAM + 4);

            // Jump
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }
    }
}
