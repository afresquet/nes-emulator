use crate::{AddressingMode, Instruction, Mem, OpCode, CPU};

pub const DCP_ZEROPAGE: u8 = 0xC7;
pub const DCP_ZEROPAGEX: u8 = 0xD7;
pub const DCP_ABSOLUTE: u8 = 0xCF;
pub const DCP_ABSOLUTEX: u8 = 0xDF;
pub const DCP_ABSOLUTEY: u8 = 0xDB;
pub const DCP_INDIRECTX: u8 = 0xC3;
pub const DCP_INDIRECTY: u8 = 0xD3;

/// Subtracts one to the value held at a specified memory location and compares it with the accumulator.
#[derive(Debug)]
pub struct InstructionDCP {
    pub addr: u16,
    pub addressing_mode: AddressingMode,
}

impl OpCode for InstructionDCP {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::DCP(Self {
            addr: cpu.get_operand_address().0,
            addressing_mode: cpu.get_addressing_mode(),
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let result = cpu.mem_read(self.addr).wrapping_sub(1);
        cpu.mem_write(self.addr, result);
        cpu.compare(result, cpu.register_a);
    }

    fn cycles(&self) -> u8 {
        match self.addressing_mode {
            AddressingMode::ZeroPage => 5,
            AddressingMode::ZeroPageX | AddressingMode::Absolute => 6,
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 7,
            AddressingMode::IndirectX | AddressingMode::IndirectY => 8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{instructions::BRK, Status};

    use super::*;

    #[test_case(DCP_ZEROPAGE, 0x10 ; "zero_page")]
    #[test_case(DCP_ZEROPAGEX, 0x0D ; "zero_page_x")]
    #[test_case(DCP_ABSOLUTE, 0x10 ; "absolute")]
    #[test_case(DCP_ABSOLUTEX, 0x0D ; "absolute_x")]
    #[test_case(DCP_ABSOLUTEY, 0x0C ; "absolute_y")]
    #[test_case(DCP_INDIRECTX, 0x1D ; "indirect_x")]
    #[test_case(DCP_INDIRECTY, 0x22 ; "indirect_y")]
    fn dcp(instruction: u8, addr: u8) {
        // Setup
        let mut cpu = CPU::new_test(&[instruction, addr, BRK]);
        cpu.register_x = 0x03;
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x20, 0x10);
        cpu.mem_write_u16(0x22, 0x0C);

        // DCP
        cpu.register_a = 0x20;
        cpu.mem_write(0x10, 0x10);
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0x0F);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));

        // Zero Flag
        cpu.swap_test_rom(&[instruction, addr, BRK]);
        cpu.reset_status();
        cpu.register_a = 0x01;
        cpu.mem_write(0x10, 0x02);
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0x01);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));

        // Negative Flag
        cpu.swap_test_rom(&[instruction, addr, BRK]);
        cpu.reset_status();
        cpu.register_a = 0x00;
        cpu.mem_write(0x10, 0x02);
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 1);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));
    }
}
