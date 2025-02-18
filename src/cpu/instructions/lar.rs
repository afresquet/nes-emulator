use crate::{Instruction, Mem, OpCode, CPU};

pub const LAR_ABSOLUTEY: u8 = 0xBB;

/// A logical AND is performed, bit by bit, on the stack pointer contents using the contents of a byte of memory,
/// transfer the result to the accumulator, X register and stack pointer,
/// setting the zero and negative flags as appropriate
#[derive(Debug)]
pub struct InstructionLAR {
    addr: u16,
    page_crossed: bool,
}

impl OpCode for InstructionLAR {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let (addr, page_crossed) = cpu.get_operand_address();
        Instruction::LAR(Self { addr, page_crossed })
    }

    fn execute(self, cpu: &mut CPU) {
        let data = cpu.mem_read(self.addr);
        cpu.stack_pointer &= data;
        cpu.register_a = cpu.stack_pointer;
        cpu.register_x = cpu.stack_pointer;
        cpu.update_zero_and_negative_flags(cpu.stack_pointer);
    }

    fn cycles(&self) -> u8 {
        4 + self.page_crossed as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn lar() {
        // Setup
        let mut cpu = CPU::new_test(&[LAR_ABSOLUTEY, 0x0C, BRK]);
        cpu.register_y = 0x04;

        // LAR
        cpu.mem_write(0x10, 0b0100);
        cpu.run();
        let expected = 0b0100;
        assert_eq!(cpu.stack_pointer, expected - 3 /* from BRK */);
        assert_eq!(cpu.register_a, expected);
        assert_eq!(cpu.register_x, expected);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // Negative Flag
        cpu.reset_program_counter();
        cpu.reset_stack_pointer();
        cpu.reset_status();
        cpu.mem_write(0x10, 0b1000_0000);
        cpu.run();
        let expected = 0b1000_0000;
        assert_eq!(cpu.stack_pointer, expected - 3 /* from BRK */);
        assert_eq!(cpu.register_a, expected);
        assert_eq!(cpu.register_x, expected);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}
