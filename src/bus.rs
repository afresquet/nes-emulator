use crate::{Mem, Rom};

pub struct Unloaded;

#[derive(Debug)]
pub struct Bus<R> {
    cpu_vram: [u8; 2048],
    rom: R,
}

impl Default for Bus<Unloaded> {
    fn default() -> Self {
        Self {
            cpu_vram: [0; 2048],
            rom: Unloaded,
        }
    }
}

impl Bus<Unloaded> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_rom(self, rom: Rom) -> Bus<Rom> {
        Bus {
            cpu_vram: self.cpu_vram,
            rom,
        }
    }
}

impl Bus<Rom> {
    pub fn swap_rom(&mut self, rom: Rom) -> Rom {
        core::mem::replace(&mut self.rom, rom)
    }

    pub fn remove_rom(self) -> (Bus<Unloaded>, Rom) {
        (
            Bus {
                cpu_vram: self.cpu_vram,
                rom: Unloaded,
            },
            self.rom,
        )
    }
}

pub const RAM: u16 = 0;
pub const RAM_MIRRORS_END: u16 = 0x1FFF;

pub const PPU_REGISTERS: u16 = 0x2000;
pub const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub const PROGRAM: u16 = 0x8000;
pub const PROGRAM_START: u16 = 0xFFFC;
pub const PROGRAM_END: u16 = 0xFFFF;

pub const STACK: u16 = 0x0100;
pub const STACK_SIZE: u8 = 0xFF;

impl Mem for Bus<Rom> {
    fn mem_read(&self, mut addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            PROGRAM..=PROGRAM_END => {
                addr -= 0x8000;
                if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                    // mirror if needed
                    addr %= 0x4000;
                }
                self.rom.prg_rom[addr as usize]
            }
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet");
            }
            PROGRAM..=PROGRAM_END => {
                panic!("Attempted to write to cartridge ROM space");
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}
