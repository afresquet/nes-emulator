pub use instructions::*;

use crate::trace::Trace;
use crate::{AddressingMode, Bus, Mem, OpCode, Rom};
use crate::{PROGRAM_START, STACK, STACK_SIZE};

pub mod instructions;

bitflags::bitflags! {
    /// 7  bit  0
    /// ---- ----
    /// NV1B DIZC
    /// |||| ||||
    /// |||| |||+- Carry
    /// |||| ||+-- Zero
    /// |||| |+--- Interrupt Disable
    /// |||| +---- Decimal
    /// |||+------ (No CPU effect; see: the B flag)
    /// ||+------- (No CPU effect; always pushed as 1)
    /// |+-------- Overflow
    /// +--------- Negative
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Status: u8 {
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL           = 0b0000_1000;
        const BREAK_COMMAND     = 0b0001_0000;
        const UNUSED            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIVE          = 0b1000_0000;
    }
}

#[derive(Debug)]
pub struct CPU {
    pub(crate) register_a: u8,
    pub(crate) register_x: u8,
    pub(crate) register_y: u8,
    pub(crate) status: Status,
    pub(crate) program_counter: u16,
    pub(crate) stack_pointer: u8,
    pub(crate) bus: Bus,
    pub(crate) current_instruction_register: u8,
}

impl CPU {
    fn new_inner(rom: Rom) -> Self {
        let mut bus = Bus::new(rom);

        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Status::UNUSED,
            program_counter: bus.mem_read_u16(PROGRAM_START),
            stack_pointer: STACK_SIZE,
            bus,
            current_instruction_register: 0,
        }
    }

    pub fn new(rom: Rom) -> Self {
        Self::new_inner(rom)
    }

    #[cfg(test)]
    pub fn new_test(program: &[u8]) -> Self {
        use crate::tests::test_rom;

        Self::new_inner(test_rom(program))
    }

    fn swap_rom_inner(&mut self, rom: Rom) {
        self.bus.insert_rom(rom);
        self.reset_program_counter();
    }

    pub fn swap_rom(&mut self, rom: Rom) {
        self.swap_rom_inner(rom);
    }

    #[cfg(test)]
    pub fn swap_test_rom(&mut self, program: &[u8]) {
        use crate::rom::tests::test_rom;

        self.swap_rom_inner(test_rom(program));
    }

    pub fn stack_pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.checked_add(1).expect("STACK OVERFLOW");
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.checked_sub(1).expect("STACK OVERFLOW");
    }

    pub fn stack_pull_u16(&mut self) -> u16 {
        let hi = self.stack_pull();
        let lo = self.stack_pull();
        u16::from_le_bytes([lo, hi])
    }

    pub fn stack_push_u16(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.stack_push(lo);
        self.stack_push(hi);
    }

    pub fn reset_registers(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
    }

    pub fn reset_status(&mut self) {
        self.status = Status::UNUSED;
    }

    pub fn reset_program_counter(&mut self) {
        self.program_counter = self.mem_read_u16(PROGRAM_START);
    }

    pub fn reset_stack_pointer(&mut self) {
        self.stack_pointer = STACK_SIZE;
    }

    pub fn reset(&mut self) {
        self.reset_registers();
        self.reset_status();
        self.reset_program_counter();
        self.reset_stack_pointer();
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_, _| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Self, &Instruction),
    {
        loop {
            if self.status.intersects(Status::BREAK_COMMAND) {
                return;
            }

            if self.bus.poll_nmi_interrupt().is_some() {
                self.nmi_interrupt();
            }

            let instruction = Instruction::fetch(self);

            callback(self, &instruction);

            self.program_counter = self
                .program_counter
                .wrapping_add(self.get_addressing_mode().bytes());

            let cycles = instruction.cycles();

            instruction.execute(self);

            self.bus.tick(cycles);
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);

        self.update_negative_flag(result);
    }

    fn update_zero_flag(&mut self, result: u8) {
        self.status.set(Status::ZERO, result == 0);
    }

    fn update_negative_flag(&mut self, result: u8) {
        self.status.set(Status::NEGATIVE, result & 1 << 7 != 0);
    }

    pub fn get_addressing_mode(&self) -> AddressingMode {
        AddressingMode::new(self.current_instruction_register).expect("valid instruction")
    }

    /// (address, page_crossed)
    pub fn get_operand_address(&mut self) -> (u16, bool) {
        use AddressingMode as AM;

        /// A page is crossed if it crossed a 256 bytes boundary
        fn page_cross(a: u16, b: u16) -> bool {
            a & 0xFF00 != b & 0xFF00
        }

        let mode = self.get_addressing_mode();

        // Skip OpCode
        let program_counter = self.program_counter.wrapping_add(1);

        match mode {
            AM::Immediate => (program_counter, false),
            AM::ZeroPage => (self.mem_read(program_counter) as u16, false),
            AM::ZeroPageX => (
                self.mem_read(program_counter).wrapping_add(self.register_x) as u16,
                false,
            ),
            AM::ZeroPageY => (
                self.mem_read(program_counter).wrapping_add(self.register_y) as u16,
                false,
            ),
            AM::Absolute => (self.mem_read_u16(program_counter), false),
            AM::AbsoluteX => {
                let base = self.mem_read_u16(program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                (addr, page_cross(base, addr))
            }
            AM::AbsoluteY => {
                let base = self.mem_read_u16(program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                (addr, page_cross(base, addr))
            }
            AM::Indirect => {
                let base = self.mem_read_u16(program_counter);

                // The 6502 microprocessor has a known bug
                // related to indirect addressing modes that involve page boundaries.
                //
                // Specifically, the bug occurs when using the indirect JMP instruction across a page boundary.
                //
                // For example, if the instruction is JMP ($10FF)
                // and the memory location $10FF holds $34, and $1100 holds $12,
                // the destination address should normally be $1234.
                //
                // However, due to the bug, the 6502 reads the addresses $10FF and $1000 instead of $10FF and $1100,
                // leading to an incorrect destination address of $3400.
                let addr = if base & 0xFF == 0xFF {
                    let lo = self.mem_read(base);
                    let hi = self.mem_read(base + 0xFF00);
                    u16::from_le_bytes([lo, hi])
                } else {
                    self.mem_read_u16(base)
                };

                (addr, false)
            }
            AM::IndirectX => {
                let pos = self.mem_read(program_counter).wrapping_add(self.register_x);
                (self.mem_read_u16(pos as u16), false)
            }
            AM::IndirectY => {
                let base = self.mem_read(program_counter);
                let deref_base = self.mem_read_u16(base as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                (deref, page_cross(deref, deref_base))
            }
            AM::Relative => {
                let skip = self.mem_read(program_counter);
                let base = self.program_counter.wrapping_add(mode.bytes());
                let addr = base.wrapping_add_signed(skip as i16);
                (addr, page_cross(base, addr))
            }
            mode => panic!("mode {mode:?} is not supported"),
        }
    }

    pub fn branch(&mut self, target: u16, condition: bool) {
        if condition {
            self.program_counter = target;
        }
    }

    pub fn compare(&mut self, data: u8, value: u8) {
        self.status.set(Status::CARRY, value >= data);

        self.update_zero_and_negative_flags(value.wrapping_sub(data));
    }

    pub fn sum(&mut self, value: u8) {
        let sum =
            self.register_a as u16 + value as u16 + self.status.contains(Status::CARRY) as u16;

        self.status.set(Status::CARRY, sum > 0xFF);

        let result = sum as u8;

        let value_mask = value ^ result;
        let acc_mask = self.register_a ^ result;
        let sign_bit = value_mask & acc_mask & 0x80;

        self.status.set(Status::OVERFLOW, sign_bit != 0);

        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn nmi_interrupt(&mut self) {
        self.stack_push_u16(self.program_counter);

        let mut flag = self.status;
        flag.set(Status::BREAK_COMMAND, false);
        self.stack_push(flag.bits());

        self.status.insert(Status::INTERRUPT_DISABLE);

        self.bus.tick(2);

        self.program_counter = self.mem_read_u16(0xFFFA);
    }

    pub fn trace(&mut self) -> Trace {
        use crate::trace::*;

        let addressing_mode = self.get_addressing_mode();

        Trace {
            program_counter: self.program_counter,
            opcode: OpCodeTrace {
                code: self.current_instruction_register,
                address: self.mem_read_u16(self.program_counter + 1),
                len: addressing_mode.bytes(),
            },
            name: Instruction::name(self.current_instruction_register),
            asm: InstructionTrace::new(self),
            registers: RegistersTrace {
                register_a: self.register_a,
                register_x: self.register_x,
                register_y: self.register_y,
                status: self.status.bits(),
                stack_pointer: self.stack_pointer,
            },
            clock_cycles: ClockCyclesTrace {
                scanline: self.bus.ppu.scanline,
                ppu_cycles: self.bus.ppu.cycles,
                cycles: self.bus.cycles,
            },
        }
    }
}

impl Mem for CPU {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }
}
