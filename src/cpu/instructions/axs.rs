use crate::{Instruction, Mem, OpCode, Status, CPU};

pub const AXS_IMMEDIATE: u8 = 0xCB;

/// A logical AND is performed, bit by bit, on the register X contents using the contents of the accumulator,
/// then sutract the memory byte from the X register.
#[derive(Debug)]
pub struct InstructionAXS {
    addr: u16,
}

impl OpCode for InstructionAXS {
    fn fetch(cpu: &mut CPU) -> Instruction {
        Instruction::SBX(Self {
            addr: cpu.get_operand_address().0,
        })
    }

    fn execute(self, cpu: &mut CPU) {
        let data = cpu.mem_read(self.addr);
        let and = cpu.register_x & cpu.register_a;
        cpu.register_x = and.wrapping_sub(data);
        cpu.update_zero_and_negative_flags(cpu.register_x);
        cpu.status.set(Status::CARRY, data <= and);
    }

    fn cycles(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::BRK, Status};

    use super::*;

    #[test]
    fn axs() {
        // Setup
        let mut cpu = CPU::new_test(&[AXS_IMMEDIATE, 0x03, BRK]);

        // AXS
        cpu.register_a = 0b1001_0011;
        cpu.register_x = 0b1010_1010;
        cpu.run();
        assert_eq!(cpu.register_x, 0b0111_1111);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));

        // Carry Flag
        cpu.reset();
        cpu.register_a = 0b0001_0011;
        cpu.register_x = 0b0010_1001;
        cpu.run();
        assert_eq!(cpu.register_x, 1u8.wrapping_sub(3));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(!cpu.status.contains(Status::CARRY));

        // Zero Flag
        cpu.reset();
        cpu.register_a = 0b1001_0011;
        cpu.register_x = 0b0010_1011;
        cpu.run();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));

        // Negative Flag
        cpu.reset();
        cpu.register_a = 0b1001_0011;
        cpu.register_x = 0b1010_1011;
        cpu.run();
        assert_eq!(cpu.register_x, 0b1000_0000);
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
        assert!(cpu.status.contains(Status::CARRY));
    }
}
