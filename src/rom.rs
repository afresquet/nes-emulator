use std::path::Path;

pub const HEADER_SIZE: usize = 16;
pub const TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const PRG_ROM_PAGE_SIZE: usize = 16384;
pub const CHR_ROM_PAGE_SIZE: usize = 8192;
pub const PRG_RAM_PAGE_SIZE: usize = 8192;
pub const TRAINER_SIZE: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RawROM<'a> {
    /// Constant $4E $45 $53 $1A (ASCII "NES" followed by MS-DOS end-of-file)
    tag: &'a [u8],
    /// Size of PRG ROM in 16 KB units
    prg_rom_size: u8,
    /// Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
    chr_rom_size: u8,
    /// Flags 6 – Mapper, mirroring, battery, trainer
    flags_6: Flags6,
    /// Flags 7 – Mapper, VS/Playchoice, NES 2.0
    flags_7: Flags7,
    /// Flags 8 – PRG-RAM size (rarely used extension)
    prg_ram_len: u8,
    /// Flags 9 – TV system (rarely used extension)
    tv_system: u8,
    /// Flags 10 – TV system, PRG-RAM presence (unofficial, rarely used extension)
    tv_system_prg_ram: u8,
    /// Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
    zeroes: &'a [u8],
    program: &'a [u8],
}

impl<'a> RawROM<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, RomError> {
        if bytes.len() < HEADER_SIZE {
            return Err(RomError::TooShort);
        }

        Ok(Self {
            tag: &bytes[0..4],
            prg_rom_size: bytes[4],
            chr_rom_size: bytes[5],
            flags_6: Flags6::new(bytes[6]),
            flags_7: Flags7::new(bytes[7])?,
            prg_ram_len: bytes[8],
            tv_system: bytes[9],
            tv_system_prg_ram: bytes[10],
            zeroes: &bytes[11..16],
            program: &bytes[16..],
        })
    }
}

#[derive(Debug)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn from_file<P>(path: P) -> Result<Self, RomError>
    where
        P: AsRef<Path>,
    {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RomError> {
        let raw = RawROM::from_bytes(bytes)?;
        Self::from_raw(raw)
    }

    fn from_raw(raw: RawROM<'_>) -> Result<Self, RomError> {
        if raw.tag != TAG {
            return Err(RomError::WrongTag);
        }

        if raw.flags_7.format != FormatOption::One {
            return Err(RomError::UnsupportedFormat);
        }

        let mapper = raw.flags_6.lower_mapper | raw.flags_7.upper_mapper << 4;

        let screen_mirroring = if raw.flags_6.four_screen {
            Mirroring::FourScreen
        } else {
            raw.flags_6.mirroring.into()
        };

        let prg_rom_size = raw.prg_rom_size as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw.chr_rom_size as usize * CHR_ROM_PAGE_SIZE;

        let prg_rom_start = if raw.flags_6.has_trainer {
            TRAINER_SIZE
        } else {
            0
        };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let prg_rom = raw
            .program
            .get(prg_rom_start..(prg_rom_start + prg_rom_size))
            .ok_or_else(|| RomError::PrgRomOutOfBounds {
                len: raw.program.len(),
                start: prg_rom_start,
                end: prg_rom_start + prg_rom_size,
            })?
            .to_vec();
        let chr_rom = raw
            .program
            .get(chr_rom_start..(chr_rom_start + chr_rom_size))
            .ok_or_else(|| RomError::ChrRomOutOfBounds {
                len: raw.program.len(),
                start: chr_rom_start,
                end: chr_rom_start + chr_rom_size,
            })?
            .to_vec();

        Ok(Self {
            prg_rom,
            chr_rom,
            mapper,
            screen_mirroring,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RomError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("The ROM length is shorter than {} bytes", HEADER_SIZE)]
    TooShort,
    #[error("The ROM is not in iNES file format")]
    WrongTag,
    #[error("Unknown format")]
    UnknownFormat,
    #[error("NES2.0 format is not supported")]
    UnsupportedFormat,
    #[error("prg_rom {start}..{end} on program of len {len}")]
    PrgRomOutOfBounds {
        len: usize,
        start: usize,
        end: usize,
    },
    #[error("chr_rom {start}..{end} on program of len {len}")]
    ChrRomOutOfBounds {
        len: usize,
        start: usize,
        end: usize,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flags6 {
    lower_mapper: u8,
    four_screen: bool,
    has_trainer: bool,
    has_battery_ram: bool,
    mirroring: MirroringOption,
}

impl Flags6 {
    fn new(byte: u8) -> Self {
        Self {
            lower_mapper: byte >> 4,
            four_screen: byte & 0b1000 != 0,
            has_trainer: byte & 0b100 != 0,
            has_battery_ram: byte & 0b10 != 0,
            mirroring: if byte & 1 != 0 {
                MirroringOption::Horizontal
            } else {
                MirroringOption::Vertical
            },
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flags7 {
    upper_mapper: u8,
    format: FormatOption,
    play_choice_10: bool,
    vs_unisystem: bool,
}

impl Flags7 {
    fn new(byte: u8) -> Result<Self, RomError> {
        let format = match (byte >> 2) & 0b11 {
            0 => FormatOption::One,
            2 => FormatOption::Two,
            _ => return Err(RomError::UnknownFormat),
        };

        Ok(Self {
            upper_mapper: (byte & 0b1111_0000) >> 4,
            format,
            play_choice_10: byte & 0b10 != 0,
            vs_unisystem: byte & 1 != 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirroringOption {
    Vertical,
    Horizontal,
}

impl From<MirroringOption> for Mirroring {
    fn from(value: MirroringOption) -> Self {
        match value {
            MirroringOption::Vertical => Self::Vertical,
            MirroringOption::Horizontal => Self::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatOption {
    One,
    Two,
}

#[cfg(test)]
pub mod tests {
    use crate::{PROGRAM, PROGRAM_START};

    use super::*;

    const F6: u8 = 0b1011_1010;
    const F7: u8 = 0b0110_0001;
    const HEADER: [u8; HEADER_SIZE] = [
        0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, F6, F7, 0xE4, 0xC0, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    pub fn test_rom(program: &[u8]) -> Rom {
        let mut prg_rom = program.to_vec();
        prg_rom.resize(2 * PRG_ROM_PAGE_SIZE, 0);
        let [lo, hi] = PROGRAM.to_le_bytes();
        prg_rom[(PROGRAM_START - PROGRAM) as usize] = lo;
        prg_rom[(PROGRAM_START - PROGRAM + 1) as usize] = hi;

        let rom = MockROM {
            header: HEADER.to_vec(),
            trainer: None,
            prg_rom,
            chr_rom: vec![0; CHR_ROM_PAGE_SIZE],
        };

        Rom::from_bytes(&rom.into_bytes()).unwrap()
    }

    #[test]
    fn header() {
        let bytes = HEADER
            .into_iter()
            .chain(std::iter::repeat_n(0xFF, 0x0001))
            .collect::<Vec<_>>();

        let raw = RawROM::from_bytes(&bytes).unwrap();

        assert_eq!(raw.tag, TAG);
        assert_eq!(raw.prg_rom_size, 0x02);
        assert_eq!(raw.chr_rom_size, 0x01);
        assert_eq!(raw.flags_6.lower_mapper, 0b1011);
        assert!(raw.flags_6.four_screen);
        assert!(!raw.flags_6.has_trainer);
        assert!(raw.flags_6.has_battery_ram);
        assert_eq!(raw.flags_6.mirroring, MirroringOption::Vertical);
        assert_eq!(raw.flags_7.upper_mapper, 0b0110);
        assert_eq!(raw.flags_7.format, FormatOption::One);
        assert!(!raw.flags_7.play_choice_10);
        assert!(raw.flags_7.vs_unisystem);
        assert_eq!(raw.prg_ram_len, 0xE4);
        assert_eq!(raw.tv_system, 0xC0);
        assert_eq!(raw.tv_system_prg_ram, 0x22);
        assert_eq!(&raw.zeroes, &[0; 5]);
        assert_eq!(&raw.program, &[0xFF; 1]);
    }

    struct MockROM {
        header: Vec<u8>,
        trainer: Option<Vec<u8>>,
        prg_rom: Vec<u8>,
        chr_rom: Vec<u8>,
    }

    impl MockROM {
        pub fn into_bytes(self) -> Vec<u8> {
            let mut bytes = Vec::with_capacity(
                self.header.len()
                    + self
                        .trainer
                        .as_ref()
                        .map(|trainer| trainer.len())
                        .unwrap_or(0)
                    + self.prg_rom.len()
                    + self.chr_rom.len(),
            );
            bytes.extend(self.header);
            if let Some(trainer) = self.trainer {
                bytes.extend(trainer);
            }
            bytes.extend(self.prg_rom);
            bytes.extend(self.chr_rom);
            bytes
        }
    }

    #[test]
    fn rom() {
        let prg_rom = vec![0x80; 0x02 * PRG_ROM_PAGE_SIZE];
        let chr_rom = vec![0xFF; CHR_ROM_PAGE_SIZE];
        let mock = MockROM {
            header: HEADER.to_vec(),
            trainer: None,
            prg_rom: prg_rom.clone(),
            chr_rom: chr_rom.clone(),
        };
        let rom = Rom::from_bytes(&mock.into_bytes()).unwrap();
        assert_eq!(rom.prg_rom, prg_rom);
        assert_eq!(rom.chr_rom, chr_rom);
        assert_eq!(rom.mapper, 0b0110_1011);
        assert_eq!(rom.screen_mirroring, Mirroring::FourScreen);
    }
}
