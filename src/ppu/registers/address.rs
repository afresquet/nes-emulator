pub const PPUADDR: u16 = 0x2006;

const LAST_VALID_ADDRESS: u16 = 0x3FFF;

#[derive(Debug, Clone, Copy)]
pub struct AddressRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl Default for AddressRegister {
    fn default() -> Self {
        Self {
            value: (0, 0),
            hi_ptr: true,
        }
    }
}

impl AddressRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self) -> u16 {
        u16::from_be_bytes(self.value.into())
    }

    pub fn set(&mut self, value: u16) {
        self.value = value.to_be_bytes().into();
    }

    /// Mirror down addr above `LAST_VALID_ADDRESS`
    fn mirror(&mut self) {
        if self.get() > LAST_VALID_ADDRESS {
            self.set(self.get() & 0b11_1111_1111_1111);
        }
    }

    pub fn update(&mut self, value: u8) {
        if self.hi_ptr {
            self.value.0 = value;
        } else {
            self.value.1 = value;
        }

        self.mirror();

        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        self.set(self.get().wrapping_add(inc as u16));

        self.mirror();
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_hi() {
        let mut register = AddressRegister::new();

        register.update(0x01);

        assert_eq!(register.value.0, 0x01);
        assert_eq!(register.value.1, 0);
    }

    #[test]
    fn sets_lo() {
        let mut register = AddressRegister::new();

        register.update(0);
        register.update(0x01);

        assert_eq!(register.value.0, 0);
        assert_eq!(register.value.1, 0x01);
    }

    #[test]
    fn gets_value() {
        let mut register = AddressRegister::new();

        register.update(0x03);
        register.update(0x21);

        assert_eq!(register.get(), 0x0321);
    }

    #[test]
    fn mirrors() {
        let mut register = AddressRegister::new();

        register.update(0x40);

        assert_eq!(register.get(), 0);
    }

    #[test]
    fn increment() {
        let mut register = AddressRegister::new();

        register.update(0x01);
        register.update(0x22);
        register.increment(1);

        assert_eq!(register.get(), 0x0123);
    }

    #[test]
    fn increment_mirror() {
        let mut register = AddressRegister::new();

        register.update(0x3F);
        register.update(0xFF);
        register.increment(1);

        assert_eq!(register.get(), 0);
    }
}
