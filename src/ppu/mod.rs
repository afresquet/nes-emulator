pub mod registers;

use crate::Mirroring;
use registers::*;

#[derive(Debug)]
pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,
    pub oam_addr: OAMAddressRegister,
    pub oam_data_r: OAMDataRegister,
    pub scroll: ScrollRegister,
    pub addr: AddressRegister,
    pub data: DataRegister,
    pub oamdma: OAMDMARegister,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Self {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
            mirroring,
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            oam_addr: OAMAddressRegister::new(),
            oam_data_r: OAMDataRegister::new(),
            scroll: ScrollRegister::new(),
            addr: AddressRegister::new(),
            data: DataRegister::new(),
            oamdma: OAMDMARegister::new(),
            internal_data_buf: 0,
        }
    }

    pub fn write_to_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            PPUCTRL..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn write_data(&mut self, value: u8) {
        let addr = self.addr.get();

        match addr {
            0..=0x1FFF => {
                panic!("Attempted to write to chr_rom space")
            }
            PPUCTRL..=0x2FFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3EFF => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),
            // Addresses 0x3F10/0x3F14/0x3F18/0x3F1C are mirrors of 0x3F00/0x3F04/0x3F08/0x3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                self.palette_table[(addr - 0x3F10) as usize] = value;
            }
            0x3F00..=0x3FFF => {
                self.palette_table[(addr - 0x3F00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]
    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        // Mirror down 0x3000-0x3EFF to 0x2000 - 0x2EFF
        let mirrored_vram = addr & 0b11111111111111;
        // To VRAM vector
        let vram_index = mirrored_vram - PPUCTRL;
        // To the name table index
        let name_table = vram_index / 0x400;
        match (self.mirroring, name_table) {
            (Mirroring::Horizontal, 1 | 2) => vram_index - 0x400,
            (Mirroring::Vertical, 2 | 3) | (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
}
