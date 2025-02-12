use crate::{OpCode, CPU};

pub fn tax(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.register_x = cpu.register_a;
    cpu.update_zero_and_negative_flags(cpu.register_x);
}

#[cfg(test)]
mod tests {
    use crate::Status;

    use super::*;

    #[test]
    fn test_0xaa_tax_transfer_accumulator_to_x() {
        let mut cpu = CPU::new();
        cpu.load(&[0xaa, 0x00]);
        cpu.reset();
        cpu.register_a = 0x05;
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xaa, 0x00]);
        assert_eq!(cpu.status.intersection(Status::ZERO), Status::ZERO);
        assert!(!cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(&[0xaa, 0x00]);
        cpu.reset();
        cpu.register_a = 0x80;
        cpu.run();
        assert!(!cpu.status.intersects(Status::ZERO));
        assert_eq!(cpu.status.intersection(Status::NEGATIVE), Status::NEGATIVE);
    }
}
