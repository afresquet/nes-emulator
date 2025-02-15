use crate::{OpCode, CPU};

use super::Instruction;

pub const JMP_ABSOLUTE: u8 = 0x4C;
pub const JMP_INDIRECT: u8 = 0x6C;

/// Sets the program counter to the address specified by the operand.
#[derive(Debug)]
pub struct InstructionJMP {
    addr: u16,
}

impl OpCode for InstructionJMP {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::JMP(Self {
            addr: cpu.get_operand_address(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        cpu.program_counter = self.addr;
    }
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
            let mut cpu = CPU::new_test(&[JMP_ABSOLUTE, lo, hi, INX, INX, BRK, INX, INX, BRK]);

            // Jump
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }

        #[test]
        fn indirect() {
            // Setup
            let mut cpu = CPU::new_test(&[JMP_INDIRECT, 0x10, 0, INX, INX, BRK, INX, INX, BRK]);
            cpu.mem_write_u16(0x10, PROGRAM + 4);

            // Jump
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }
    }
}
