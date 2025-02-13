use crate::{AddressingMode, Bus, Mem, OpCodeType, OPCODES};

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
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const BREAK_COMMAND = 0b0001_0000;
        const UNUSED = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

pub const STACK: u16 = 0x0100;
pub const STACK_SIZE: u8 = 0xFF;

pub const PROGRAM: u16 = 0x0600;
pub const PROGRAM_START: u16 = 0x1FFC;

#[derive(Debug)]
pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    status: Status,
    program_counter: u16,
    stack_pointer: u8,
    bus: Bus,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Status::UNUSED,
            program_counter: 0,
            stack_pointer: STACK_SIZE,
            bus: Bus::default(),
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
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
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.reset_status();
        self.reset_program_counter();
        self.reset_stack_pointer();
    }

    pub fn load(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.mem_write(PROGRAM + i as u16, *byte);
        }
        self.mem_write_u16(PROGRAM_START, PROGRAM);
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Self),
    {
        let opcodes = std::sync::LazyLock::force(&OPCODES);

        loop {
            let opcode = self.mem_read(self.program_counter);
            let opcode = opcodes.get(&opcode).expect("to be a valid opcode");

            self.program_counter = self.program_counter.wrapping_add(1);

            let pc_state = self.program_counter;

            (opcode.instruction)(self, opcode);

            if let OpCodeType::BRK = opcode.ty {
                return;
            }

            if self.program_counter == pc_state {
                self.program_counter = self.program_counter.wrapping_add(opcode.bytes as u16 - 1);
            }

            callback(self);
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

    fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        use AddressingMode as AM;

        match mode {
            AM::Immediate => self.program_counter,
            AM::ZeroPage => self.mem_read(self.program_counter) as u16,
            AM::ZeroPageX => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_x) as u16,
            AM::ZeroPageY => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_y) as u16,
            AM::Absolute => self.mem_read_u16(self.program_counter),
            AM::AbsoluteX => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_x as u16),
            AM::AbsoluteY => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_y as u16),
            AM::Indirect => {
                let pos = self.mem_read_u16(self.program_counter);

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
                if pos & 0xFF == 0xFF {
                    let lo = self.mem_read(pos);
                    let hi = self.mem_read(pos + 0xFF00);
                    u16::from_le_bytes([lo, hi])
                } else {
                    self.mem_read_u16(pos)
                }
            }
            AM::IndirectX => {
                let pos = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_x);
                self.mem_read_u16(pos as u16)
            }
            AM::IndirectY => {
                let pos = self.mem_read(self.program_counter);
                self.mem_read_u16(pos as u16)
                    .wrapping_add(self.register_y as u16)
            }
            mode => panic!("mode {mode:?} is not supported"),
        }
    }

    pub fn branch(&mut self, condition: bool) {
        if condition {
            let skip = self.mem_read(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add_signed(skip as i16 + 1);
        }
    }

    pub fn compare(&mut self, value: u8, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

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
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }
}
