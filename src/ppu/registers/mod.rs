pub mod address;
pub mod control;
pub mod data;
pub mod mask;
pub mod oam_address;
pub mod oam_data;
pub mod oam_dma;
pub mod scroll;
pub mod status;

pub use address::*;
pub use control::*;
pub use data::*;
pub use mask::*;
pub use oam_address::*;
pub use oam_data::*;
pub use oam_dma::*;
pub use scroll::*;
pub use status::*;

use core::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Copy)]
pub struct ByteRegister(u8);

impl ByteRegister {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for ByteRegister {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteRegister {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
