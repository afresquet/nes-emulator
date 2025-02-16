use core::fmt::Display;

use crate::{AddressingMode, Mem, CPU};

pub struct Trace {
    pub program_counter: u16,
    pub opcode: OpCodeTrace,
    pub name: &'static str,
    pub asm: InstructionTrace,
    pub registers: RegistersTrace,
    pub clock_cycles: ClockCyclesTrace,
}

impl Display for Trace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            program_counter,
            opcode,
            name,
            asm,
            registers,
            clock_cycles,
        } = self;

        // D136  E1 80     SBC ($80,X) @ 80 = 0200 = 40    A:40 X:00 Y:6C P:65 SP:FB PPU: 31,250 CYC:3607
        // PC    OpCode    Assembly                        Registers                 Clock Cycles
        write!(
            f,
            "{program_counter:04X} {opcode}  {name} {asm}  {registers} {clock_cycles}",
        )
    }
}

pub struct OpCodeTrace {
    pub code: u8,
    pub address: u16,
    pub len: u16,
}

impl Display for OpCodeTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { code, address, len } = self;

        // E1 80 40
        // OpCode
        match len {
            1 => write!(f, "{code:02X}      "),
            2 => {
                let [lo, _] = address.to_le_bytes();

                write!(f, "{code:02X} {lo:02X}   ")
            }
            3 => {
                let [lo, hi] = address.to_le_bytes();

                write!(f, "{code:02X} {lo:02X} {hi:02X}")
            }
            _ => unreachable!("MAX LENGTH IS 3"),
        }
    }
}

pub enum InstructionTrace {
    Implied,
    Accumulator,
    Immediate(u8),
    Memory {
        address: u16,
        value: u8,
        padding: usize,
    },
    MemoryPlusRegister {
        before: u16,
        after: u16,
        value: u8,
        register: Register,
        padding: usize,
    },
    Indirect {
        address: u16,
        target: u16,
    },
    IndirectPlusRegister {
        before: u16,
        after: u16,
        target: u16,
        value: u8,
        register: Register,
    },
    Relative(u16),
}

impl InstructionTrace {
    pub fn new(cpu: &mut CPU) -> Self {
        let mode = cpu.get_addressing_mode();

        let (addr, value) = match mode {
            AddressingMode::Implied => return Self::Implied,
            AddressingMode::Accumulator => return Self::Accumulator,
            _ => {
                let addr = cpu.get_operand_address().0;
                (addr, cpu.mem_read(addr))
            }
        };

        let address = match mode.bytes() {
            2 => cpu.mem_read(cpu.program_counter) as u16,
            3 => cpu.mem_read_u16(cpu.program_counter),
            _ => unreachable!("already returned from accumulator and implied"),
        };

        match mode {
            AddressingMode::Immediate => Self::Immediate(address as u8),
            AddressingMode::ZeroPage => Self::Memory {
                address: addr,
                value,
                padding: 2,
            },
            AddressingMode::ZeroPageX => Self::MemoryPlusRegister {
                before: address,
                after: addr,
                value,
                register: Register::X,
                padding: 2,
            },
            AddressingMode::ZeroPageY => Self::MemoryPlusRegister {
                before: address,
                after: addr,
                value,
                register: Register::Y,
                padding: 2,
            },
            AddressingMode::Absolute => Self::Memory {
                address: addr,
                value,
                padding: 4,
            },
            AddressingMode::AbsoluteX => Self::MemoryPlusRegister {
                before: address,
                after: addr,
                value,
                register: Register::X,
                padding: 4,
            },
            AddressingMode::AbsoluteY => Self::MemoryPlusRegister {
                before: address,
                after: addr,
                value,
                register: Register::Y,
                padding: 4,
            },
            AddressingMode::Indirect => Self::Indirect {
                address,
                target: addr,
            },
            AddressingMode::IndirectX => Self::IndirectPlusRegister {
                before: address,
                after: address.wrapping_add(cpu.register_x as u16),
                target: addr,
                value,
                register: Register::X,
            },
            AddressingMode::IndirectY => Self::IndirectPlusRegister {
                before: address,
                after: addr.wrapping_sub(cpu.register_y as u16),
                target: addr,
                value,
                register: Register::Y,
            },
            AddressingMode::Relative => Self::Relative(addr),
            _ => unreachable!("already returned from accumulator and implied"),
        }
    }
}

#[derive(Debug)]
pub enum Register {
    X,
    Y,
}

impl Display for InstructionTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InstructionTrace::Implied => write!(f, "                          "),
            InstructionTrace::Accumulator => write!(f, "A                         "),
            InstructionTrace::Immediate(value) => write!(f, "#${value:02X}                      "),
            InstructionTrace::Memory {
                address,
                value,
                padding,
            } => {
                write!(
                    f,
                    "${address:0width$X} = {value:02X}                {empty:<pad$}",
                    width = padding,
                    empty = "",
                    pad = padding % 4,
                )
            }
            InstructionTrace::MemoryPlusRegister {
                before,
                after,
                value,
                register,
                padding,
            } => write!(
                f,
                "${before:0width$X},{register:?} @ {after:0width$X} = {value:02X}         {empty:<pad$}",
                width = padding,
                empty = "",
                pad = padding % 4,
            ),
            InstructionTrace::Indirect { address, target } => {
                write!(f, "(${address:04X}) = {target:04X}                ")
            }
            InstructionTrace::IndirectPlusRegister {
                before,
                after,
                target,
                value,
                register,
            } => match register {
                Register::X => write!(
                    f,
                    "(${before:02X},{register:?}) @ {after:02X} = {target:04X} = {value:02X}    ",
                ),
                Register::Y => write!(
                    f,
                    "(${before:02X}),{register:?} = {after:04X} @ {target:04X} = {value:02X}  ",
                ),
            },
            InstructionTrace::Relative(address) => write!(f, "${address:04X}                     "),
        }
    }
}

#[allow(dead_code)]
enum AddressingModeX {
    Immediate,   // Immediate
    ZeroPage,    // Memory
    ZeroPageX,   // MemoryPlusRegister
    ZeroPageY,   // MemoryPlusRegister
    Absolute,    // Memory
    AbsoluteX,   // MemoryPlusRegister
    AbsoluteY,   // MemoryPlusRegister
    Indirect,    // Indirect
    IndirectX,   // IndirectPlusRegister
    IndirectY,   // IndirectPlusRegister
    Accumulator, // Accumulator
    Relative,    // Relative
    Implied,     // Implied
}

pub struct RegistersTrace {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub stack_pointer: u8,
}

impl Display for RegistersTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // A:40 X:00 Y:6C P:65 SP:FB
        // Registers
        write!(
            f,
            "A:{a:02X} X:{x:02X} Y:{y:02X} P:{s:02X} SP:{sp:02X}",
            a = self.register_a,
            x = self.register_x,
            y = self.register_y,
            s = self.status,
            sp = self.stack_pointer,
        )
    }
}

pub struct ClockCyclesTrace {
    pub scanline: u16,
    pub ppu_cycles: usize,
    pub cycles: usize,
}

impl Display for ClockCyclesTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // PPU: 31,250 CYC:3607
        // Clock Cycles
        write!(
            f,
            "PPU:{scanline:>3},{ppu_cycles:>3} CYC:{cycles}",
            scanline = self.scanline,
            ppu_cycles = self.ppu_cycles,
            cycles = self.cycles
        )
    }
}
