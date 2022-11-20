
mod cpu;
mod mappers;
mod ppu;

use cpu::CPU;
use mappers::MapperEnum;
use mappers::default_mapper::DefaultMapper;
use ppu::PPU;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct NES {
    mapper: MapperEnum,
    cpu: CPU,
    ppu: PPU,

    wram: [u8; 0x800],
}

impl NES {
    pub fn new() -> Self {
       Self {
           mapper: MapperEnum::DefaultMapper(DefaultMapper::new().into()),
           cpu: CPU::new(),
           ppu: PPU::new(),

           wram: [0; 0x800],
       }
    }

    fn load_rom(&mut self, rom_path: &Path) -> std::io::Result<()> {
        // Read header
        let mut rom_file = File::open(rom_path)?;
        let mut header: [u8; 16] = [0; 16];
        rom_file.read_exact(&mut header)?;

        // Get format
        assert!((header[0] == 0x4E) && (header[1] == 0x45) && (header[2] == 0x53) && (header[3] == 0x1A), "Rom does not have iNES header");
        let _header_ver2 = (header[7] & 0x0C) == 0x08;

        self.mapper = MapperEnum::DefaultMapper(DefaultMapper::new().into());

        Ok(())
    }
}