use crate::{
    ppu::{registers::*, *},
    Mem, Rom,
};

#[derive(Debug)]
pub struct Bus {
    pub cpu_vram: [u8; 2048],
    pub prg_rom: Vec<u8>,
    pub ppu: PPU,
    pub cycles: usize,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.screen_mirroring),
            cycles: 0,
        }
    }

    pub fn insert_rom(&mut self, rom: Rom) {
        self.prg_rom = rom.prg_rom;
        self.ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);
        self.cycles = 0;
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        self.ppu.tick(cycles * 3);
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<()> {
        self.ppu.poll_nmi_interrupt()
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
                let mirror_down_addr = addr & 0b0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }

            // PPU
            PPUCTRL | PPUMASK | OAMADDR | PPUSCROLL | PPUADDR | OAMDMA => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }
            PPUSTATUS => self.ppu.read_status(),
            OAMDATA => self.ppu.read_oam_data(),
            PPUDATA => self.ppu.read_data(),
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_read(mirror_down_addr)
            }

            // PROGRAM
            PROGRAM..=PROGRAM_END => {
                addr -= PROGRAM;
                if self.prg_rom.len() == PROGRAM as usize / 2 && addr >= PROGRAM / 2 {
                    // mirror if needed
                    addr %= PROGRAM / 2;
                }
                self.prg_rom[addr as usize]
            }

            _ => 0,
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            // RAM
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }

            // PPU
            PPUCTRL => {
                self.ppu.write_to_ctrl(data);
            }
            PPUMASK => self.ppu.write_to_mask(data),
            PPUSTATUS => panic!("Attempted to write to PPU Status register"),
            OAMADDR => self.ppu.write_to_oam_addr(data),
            OAMDATA => self.ppu.write_to_oam_data(data),
            PPUSCROLL => self.ppu.write_to_scroll(data),
            PPUADDR => self.ppu.write_to_addr(data),
            PPUDATA => self.ppu.write_data(data),
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_write(mirror_down_addr, data);
            }

            // PROGRAM
            PROGRAM..=PROGRAM_END => panic!("Attempted to write to cartridge ROM space"),

            _ => (),
        }
    }
}
