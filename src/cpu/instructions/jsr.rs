use crate::{Instruction, OpCode, CPU};

pub const JSR: u8 = 0x20;

/// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
#[derive(Debug)]
pub struct InstructionJSR {
    addr: u16,
}

impl OpCode for InstructionJSR {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::JSR(Self {
            addr: cpu.get_operand_address(),
        })
    }

    fn execute(self, cpu: &mut CPU) -> u8 {
        cpu.stack_push_u16(cpu.program_counter - 1);
        cpu.program_counter = self.addr;
        self.cycles(false)
    }

    fn cycles(&self, _page_crossed: bool) -> u8 {
        6
    }
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
        let mut cpu = CPU::new_test(&[JSR, lo, hi, INX, INX, BRK, INX, INX, INX, BRK]);

        // Jump
        cpu.run();
        cpu.stack_pull(); // BRK Status
        cpu.stack_pull_u16(); // BRK Program Counter
        assert_eq!(cpu.stack_pull_u16(), PROGRAM + 2);
        assert_eq!(cpu.register_x, 1);
    }
}
