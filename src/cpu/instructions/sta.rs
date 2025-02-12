use crate::{OpCode, CPU};

pub fn sta(cpu: &mut CPU, opcode: &OpCode) {
    let addr = cpu.get_operand_address(opcode.mode);
    cpu.mem_write(addr, cpu.register_a);
}
