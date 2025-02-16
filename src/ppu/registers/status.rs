pub const PPUSTATUS: u16 = 0x2002;

bitflags::bitflags! {
    /// 7  bit  0
    /// ---- ----
    /// VSO. ....
    /// |||| ||||
    /// |||+-++++- Least significant bits previously written into a PPU register
    /// |||        (due to register not being updated for this address)
    /// ||+------- Sprite overflow. The intent was for this flag to be set
    /// ||         whenever more than eight sprites appear on a scanline, but a
    /// ||         hardware bug causes the actual behavior to be more complicated
    /// ||         and generate false positives as well as false negatives; see
    /// ||         PPU sprite evaluation. This flag is set during sprite
    /// ||         evaluation and cleared at dot 1 (the second dot) of the
    /// ||         pre-render line.
    /// |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    /// |          a nonzero background pixel; cleared at dot 1 of the pre-render
    /// |          line.  Used for raster timing.
    /// +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
    ///            Set at dot 1 of line 241 (the line *after* the post-render
    ///            line); cleared after reading $2002 and at dot 1 of the
    ///            pre-render line.
    #[derive(Debug, Clone, Copy)]
    pub struct StatusRegister: u8 {
        const SPRITE_OVERFLOW = 0b00100000;
        const SPRITE_ZERO_HIT = 0b01000000;
        const VBLANK_STARTED  = 0b10000000;
    }
}

impl Default for StatusRegister {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl StatusRegister {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_in_vblank(&self) -> bool {
        self.contains(Self::VBLANK_STARTED)
    }

    pub fn set_vblank_status(&mut self, status: bool) {
        self.set(Self::VBLANK_STARTED, status);
    }
}
