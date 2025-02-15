pub const PPUSCROLL: u16 = 0x2005;

#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollRegister {
    pub x: u8,
    pub y: u8,
    latch: bool,
}

impl ScrollRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, value: u8) {
        if self.latch {
            self.y = value;
        } else {
            self.x = value
        }

        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }
}
