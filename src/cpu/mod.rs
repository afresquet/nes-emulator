use crate::{AddressingMode, OpCodeType, OPCODES};

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

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: Status,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Status::UNUSED,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(pos, lo);
        self.mem_write(pos.wrapping_add(1), hi);
    }

    pub fn reset_status(&mut self) {
        self.status = Status::UNUSED;
    }

    pub fn reset_program_counter(&mut self) {
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.reset_status();
        self.reset_program_counter();
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.program_counter);
            let opcode = OPCODES.get(&opcode).expect("to be a valid opcode");

            self.program_counter = self.program_counter.wrapping_add(1);

            (opcode.instruction)(self, opcode);

            if let OpCodeType::BRK = opcode.ty {
                return;
            }

            self.program_counter = self.program_counter.wrapping_add(opcode.bytes as u16 - 1);
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(Status::ZERO);
        } else {
            self.status.remove(Status::ZERO);
        }

        if result & 1 << 7 != 0 {
            self.status.insert(Status::NEGATIVE);
        } else {
            self.status.remove(Status::NEGATIVE);
        }
    }

    fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        use AddressingMode as AM;

        match mode {
            AM::Immediate => self.program_counter,
            AM::ZeroPage | AM::Relative => self.mem_read(self.program_counter) as u16,
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
            let skip = self.get_operand_address(AddressingMode::Relative);
            self.program_counter = self.program_counter.wrapping_add(skip);
        }
    }

    pub fn compare(&mut self, value: u8, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        if value >= data {
            self.status.insert(Status::CARRY);
        } else {
            self.status.remove(Status::CARRY);
        }

        self.update_zero_and_negative_flags(value.wrapping_sub(data));
    }
}
