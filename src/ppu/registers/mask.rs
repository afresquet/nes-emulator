pub const PPUMASK: u16 = 0x2001;

bitflags::bitflags! {
    /// 7  bit  0
    /// ---- ----
    /// BGRs bMmG
    /// |||| ||||
    /// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
    /// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
    /// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    /// |||| +---- 1: Show background
    /// |||+------ 1: Show sprites
    /// ||+------- Emphasize red*
    /// |+-------- Emphasize green*
    /// +--------- Emphasize blue*
    #[derive(Debug, Clone, Copy)]
    pub struct MaskRegister: u8 {
        const GREYSCALE            = 0b00000001;
        const SHOW_BACKGROUND_LEFT = 0b00000010;
        const SHOW_SPRITES_LEFT    = 0b00000100;
        const SHOW_BACKGROUND      = 0b00001000;
        const SHOW_SPRITES         = 0b00010000;
        const EMPHAZISE_RED        = 0b00100000;
        const EMPHAZISE_GREEN      = 0b01000000;
        const EMPHAZISE_BLUE       = 0b10000000;
    }
}

impl Default for MaskRegister {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl MaskRegister {
    pub fn new() -> Self {
        Default::default()
    }
}
