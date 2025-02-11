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
    pub status: Status,
    pub program_counter: u16,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: 0,
            status: Status::UNUSED,
            program_counter: 0,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, program: &[u8]) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0xA9 /* LDA */ => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status.insert(Status::ZERO);
                    } else {
                        self.status.remove(Status::ZERO);
                    }

                    if self.register_a & 1 << 7 != 0 {
                        self.status.insert(Status::NEGATIVE);
                    } else {
                        self.status.remove(Status::NEGATIVE);
                    }
                }
                0x00 /* BRK */ => return,
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(&[0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert!(!cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(&[0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status.intersection(Status::ZERO), Status::ZERO);
        assert!(!cpu.status.intersects(Status::NEGATIVE));
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(&[0xa9, 0x80, 0x00]);
        assert!(!cpu.status.intersects(Status::ZERO));
        assert_eq!(cpu.status.intersection(Status::NEGATIVE), Status::NEGATIVE);
    }
}
