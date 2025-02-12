use crate::{OpCode, CPU};

pub const BRK: u8 = 0x00;

pub fn brk(_cpu: &mut CPU, _opcode: &OpCode) {
    unreachable!()
}
