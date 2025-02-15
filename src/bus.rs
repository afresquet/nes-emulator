use crate::{
    ppu::{registers::*, *},
    Mem, Rom,
};

#[derive(Debug)]
pub struct Bus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.screen_mirroring),
        }
    }

    pub fn insert_rom(&mut self, rom: Rom) {
        *self = Self::new(rom);
    }
}

pub const RAM: u16 = 0;
pub const RAM_MIRRORS_END: u16 = 0x1FFF;

pub const PPU_REGISTERS: u16 = 0x2008;
pub const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub const PROGRAM: u16 = 0x8000;
pub const PROGRAM_START: u16 = 0xFFFC;
pub const PROGRAM_END: u16 = 0xFFFF;

pub const STACK: u16 = 0x0100;
pub const STACK_SIZE: u8 = 0xFF;

impl Mem for Bus {
    fn mem_read(&mut self, mut addr: u16) -> u8 {
        match addr {
            // RAM
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize]
            }

            // PPU
            PPUCTRL | PPUMASK | OAMADDR | PPUSCROLL | PPUADDR | OAMDMA => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }
            PPUDATA => self.ppu.read_data(),
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
            }

            // Program
            PROGRAM..=PROGRAM_END => {
                addr -= 0x8000;
                if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                    // mirror if needed
                    addr %= 0x4000;
                }
                self.prg_rom[addr as usize]
            }

            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            // RAM
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }

            // PPU
            PPUCTRL => {
                self.ppu.write_to_ctrl(data);
            }
            PPUADDR => {
                self.ppu.write_to_addr(data);
            }
            PPUDATA => {
                self.ppu.write_data(data);
            }
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            }

            // PROGRAM
            PROGRAM..=PROGRAM_END => {
                panic!("Attempted to write to cartridge ROM space");
            }

            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}
