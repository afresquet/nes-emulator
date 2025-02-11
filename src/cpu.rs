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

    pub fn reset(&mut self) {
        *self = Self {
            program_counter: self.mem_read_u16(0xFFFC),
            memory: self.memory,
            ..Default::default()
        }
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

            self.program_counter += 1;

            match opcode {
                0xA9 => {
                    self.lda(AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA5 => {
                    self.lda(AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => unimplemented!(),
            }
        }
    }

    fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
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
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPageX => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_x) as u16,
            AddressingMode::ZeroPageY => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_y) as u16,
            AddressingMode::AbsoluteX => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_x as u16),
            AddressingMode::AbsoluteY => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_y as u16),
            AddressingMode::IndirectX => {
                let pos = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_x);
                let lo = self.mem_read(pos as u16);
                let hi = self.mem_read(pos.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::IndirectY => {
                let pos = self.mem_read(self.program_counter);
                let lo = self.mem_read(pos as u16);
                let hi = self.mem_read(pos.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi]).wrapping_add(self.register_y as u16)
            }
            AddressingMode::NoneAddressing => panic!("mode {mode:?} is not supported"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lda {
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

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }

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
