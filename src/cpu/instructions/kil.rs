use crate::{Instruction, OpCode, CPU};

pub const KIL_IMPLIED1: u8 = 0x02;
pub const KIL_IMPLIED2: u8 = 0x12;
pub const KIL_IMPLIED3: u8 = 0x22;
pub const KIL_IMPLIED4: u8 = 0x32;
pub const KIL_IMPLIED5: u8 = 0x42;
pub const KIL_IMPLIED6: u8 = 0x52;
pub const KIL_IMPLIED7: u8 = 0x62;
pub const KIL_IMPLIED8: u8 = 0x72;
pub const KIL_IMPLIED9: u8 = 0x92;
pub const KIL_IMPLIED10: u8 = 0xB2;
pub const KIL_IMPLIED11: u8 = 0xD2;
pub const KIL_IMPLIED12: u8 = 0xF2;

/// Stop program counter (processor lock up).
#[derive(Debug)]
pub struct InstructionKIL;

impl OpCode for InstructionKIL {
    fn fetch(_cpu: &mut CPU) -> Instruction {
        Instruction::JAM(Self)
    }

    fn execute(self, _cpu: &mut CPU) {
        // TODO: handle halt
    }

    fn cycles(&self) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status, PROGRAM};

    use super::*;

    #[test_case(KIL_IMPLIED1, 1 ; "implied_1")]
    #[test_case(KIL_IMPLIED2, 1 ; "implied_2")]
    #[test_case(KIL_IMPLIED3, 1 ; "implied_3")]
    #[test_case(KIL_IMPLIED4, 1 ; "implied_4")]
    #[test_case(KIL_IMPLIED5, 1 ; "implied_5")]
    #[test_case(KIL_IMPLIED6, 1 ; "implied_6")]
    #[test_case(KIL_IMPLIED7, 1 ; "implied_7")]
    #[test_case(KIL_IMPLIED8, 1 ; "implied_8")]
    #[test_case(KIL_IMPLIED9, 1 ; "implied_9")]
    #[test_case(KIL_IMPLIED10, 1 ; "implied_10")]
    #[test_case(KIL_IMPLIED11, 1 ; "implied_11")]
    #[test_case(KIL_IMPLIED12, 1 ; "implied_12")]
    fn nop(instruction: u8, bytes: u16) {
        let mut cpu = CPU::new_test(&[instruction, BRK]);
        cpu.run();
        assert_eq!(cpu.program_counter, PROGRAM + bytes);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.status, Status::INTERRUPT_DISABLE | Status::UNUSED);
    }
}
