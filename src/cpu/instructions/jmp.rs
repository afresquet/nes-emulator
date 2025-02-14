use crate::{Bus, OpCode, Rom, CPU};

pub const JMP_ABSOLUTE: u8 = 0x4C;
pub const JMP_INDIRECT: u8 = 0x6C;

/// Sets the program counter to the address specified by the operand.
pub fn jmp(cpu: &mut CPU<Bus<Rom>>, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.program_counter = addr;
}

#[cfg(test)]
mod tests {
    mod jmp {
        use crate::{
            instructions::{BRK, INX},
            Mem, PROGRAM,
        };

        use super::super::*;

        #[test]
        fn absolute() {
            // Setup
            let [lo, hi] = (PROGRAM + 4).to_le_bytes();
            let mut cpu =
                CPU::new().insert_test_rom(&[JMP_ABSOLUTE, lo, hi, INX, INX, BRK, INX, INX, BRK]);

            // Jump
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }

        #[test]
        fn indirect() {
            // Setup
            let mut cpu =
                CPU::new().insert_test_rom(&[JMP_INDIRECT, 0x10, 0, INX, INX, BRK, INX, INX, BRK]);
            cpu.mem_write_u16(0x10, PROGRAM + 4);

            // Jump
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }
    }
}
