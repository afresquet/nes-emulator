use crate::{Instruction, Mem, OpCode, CPU};

pub const DOP_IMMEDIATE1: u8 = 0x80;
pub const DOP_IMMEDIATE2: u8 = 0x82;
pub const DOP_IMMEDIATE3: u8 = 0x89;
pub const DOP_IMMEDIATE4: u8 = 0xC2;
pub const DOP_IMMEDIATE5: u8 = 0xE2;
pub const DOP_ZEROPAGE1: u8 = 0x04;
pub const DOP_ZEROPAGE2: u8 = 0x44;
pub const DOP_ZEROPAGE3: u8 = 0x64;
pub const DOP_ZEROPAGEX1: u8 = 0x14;
pub const DOP_ZEROPAGEX2: u8 = 0x34;
pub const DOP_ZEROPAGEX3: u8 = 0x54;
pub const DOP_ZEROPAGEX4: u8 = 0x74;
pub const DOP_ZEROPAGEX5: u8 = 0xD4;
pub const DOP_ZEROPAGEX6: u8 = 0xF4;
pub const TOP_ABSOLUTE: u8 = 0x0C;
pub const TOP_ABSOLUTEX1: u8 = 0x1C;
pub const TOP_ABSOLUTEX2: u8 = 0x3C;
pub const TOP_ABSOLUTEX3: u8 = 0x5C;
pub const TOP_ABSOLUTEX4: u8 = 0x7C;
pub const TOP_ABSOLUTEX5: u8 = 0xDC;
pub const TOP_ABSOLUTEX6: u8 = 0xFC;
pub const NOP: u8 = 0xEA;
pub const NOP_IMPLIED1: u8 = 0x1A;
pub const NOP_IMPLIED2: u8 = 0x3A;
pub const NOP_IMPLIED3: u8 = 0x5A;
pub const NOP_IMPLIED4: u8 = 0x7A;
pub const NOP_IMPLIED5: u8 = 0xDA;
pub const NOP_IMPLIED6: u8 = 0xFA;

/// The NOP instruction causes no changes to the processor other than the normal incrementing of the program counter to the next instruction.
#[derive(Debug)]
pub struct InstructionNOP {
    opcode: u8,
    page_cross: bool,
}

impl OpCode for InstructionNOP {
    fn fetch(cpu: &mut CPU) -> Instruction {
        let opcode = cpu.mem_read(cpu.program_counter);
        let page_cross = match opcode {
            TOP_ABSOLUTEX1 | TOP_ABSOLUTEX2 | TOP_ABSOLUTEX3 | TOP_ABSOLUTEX4 | TOP_ABSOLUTEX5
            | TOP_ABSOLUTEX6 => cpu.get_operand_address().1,
            _ => false,
        };
        Instruction::NOP(Self { opcode, page_cross })
    }

    fn execute(self, _cpu: &mut CPU) {}

    fn cycles(&self) -> u8 {
        match self.opcode {
            NOP | DOP_IMMEDIATE1 | DOP_IMMEDIATE2 | DOP_IMMEDIATE3 | DOP_IMMEDIATE4
            | DOP_IMMEDIATE5 | NOP_IMPLIED1 | NOP_IMPLIED2 | NOP_IMPLIED3 | NOP_IMPLIED4
            | NOP_IMPLIED5 | NOP_IMPLIED6 => 2,

            DOP_ZEROPAGE1 | DOP_ZEROPAGE2 | DOP_ZEROPAGE3 => 3,

            DOP_ZEROPAGEX1 | DOP_ZEROPAGEX2 | DOP_ZEROPAGEX3 | DOP_ZEROPAGEX4 | DOP_ZEROPAGEX5
            | DOP_ZEROPAGEX6 | TOP_ABSOLUTE => 4,

            TOP_ABSOLUTEX1 | TOP_ABSOLUTEX2 | TOP_ABSOLUTEX3 | TOP_ABSOLUTEX4 | TOP_ABSOLUTEX5
            | TOP_ABSOLUTEX6 => 4 + self.page_cross as u8,

            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status, PROGRAM};

    use super::*;

    #[test_case(DOP_IMMEDIATE1, 2 ; "immediate_1")]
    #[test_case(DOP_IMMEDIATE2, 2 ; "immediate_2")]
    #[test_case(DOP_IMMEDIATE3, 2 ; "immediate_3")]
    #[test_case(DOP_IMMEDIATE4, 2 ; "immediate_4")]
    #[test_case(DOP_IMMEDIATE5, 2 ; "immediate_5")]
    #[test_case(DOP_ZEROPAGE1, 2 ; "zero_page_1")]
    #[test_case(DOP_ZEROPAGE2, 2 ; "zero_page_2")]
    #[test_case(DOP_ZEROPAGE3, 2 ; "zero_page_3")]
    #[test_case(DOP_ZEROPAGEX1, 2 ; "zero_page_x_1")]
    #[test_case(DOP_ZEROPAGEX2, 2 ; "zero_page_x_2")]
    #[test_case(DOP_ZEROPAGEX3, 2 ; "zero_page_x_3")]
    #[test_case(DOP_ZEROPAGEX4, 2 ; "zero_page_x_4")]
    #[test_case(DOP_ZEROPAGEX5, 2 ; "zero_page_x_5")]
    #[test_case(DOP_ZEROPAGEX6, 2 ; "zero_page_x_6")]
    #[test_case(TOP_ABSOLUTE, 3 ; "absolute")]
    #[test_case(TOP_ABSOLUTEX1, 3 ; "absolute_1")]
    #[test_case(TOP_ABSOLUTEX2, 3 ; "absolute_2")]
    #[test_case(TOP_ABSOLUTEX3, 3 ; "absolute_3")]
    #[test_case(TOP_ABSOLUTEX4, 3 ; "absolute_4")]
    #[test_case(TOP_ABSOLUTEX5, 3 ; "absolute_5")]
    #[test_case(TOP_ABSOLUTEX6, 3 ; "absolute_6")]
    #[test_case(NOP, 1 ; "implied")]
    #[test_case(NOP_IMPLIED1, 1 ; "implied_1")]
    #[test_case(NOP_IMPLIED2, 1 ; "implied_2")]
    #[test_case(NOP_IMPLIED3, 1 ; "implied_3")]
    #[test_case(NOP_IMPLIED4, 1 ; "implied_4")]
    #[test_case(NOP_IMPLIED5, 1 ; "implied_5")]
    #[test_case(NOP_IMPLIED6, 1 ; "implied_6")]
    fn nop(instruction: u8, bytes: u16) {
        let mut cpu = CPU::new_test(&[instruction, BRK]);
        cpu.run();
        assert_eq!(cpu.program_counter, PROGRAM +1 /* from BRK */ + bytes);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(
            cpu.status,
            Status::INTERRUPT_DISABLE | Status::UNUSED | Status::BREAK_COMMAND
        );
    }
}
