use crate::{OpCode, CPU};

pub fn lda(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    let value = cpu.mem_read(addr);
    cpu.register_a = value;
    cpu.update_zero_and_negative_flags(cpu.register_a);
}

#[cfg(test)]
mod tests {
    use crate::Status;

    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status.intersection(Status::ZERO), Status::ZERO);
        assert!(!cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0x80, 0x00]);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert_eq!(cpu.status.intersection(Status::NEGATIVE), Status::NEGATIVE);
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
}
