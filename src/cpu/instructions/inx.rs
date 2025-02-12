use crate::{OpCode, CPU};

pub fn inx(cpu: &mut CPU, _opcode: &OpCode) {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    cpu.update_zero_and_negative_flags(cpu.register_x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(&[0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = u8::MAX;
        cpu.run();
        assert_eq!(cpu.register_x, 1)
    }
}
