
mod cpu;
mod mappers;

use cpu::CPU;
use mappers::MapperEnum;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct NES {
    mapper: MapperEnum,
    cpu: CPU
}

impl NES {
    fn new() -> Self {
       Self {
           mapper: MapperEnum::DefaultMapper,
           cpu: CPU::new()
       }
    }

    fn loadRom(&mut self, rom_path: &Path) {
        // Read header
        let mut rom_file = File::open(rom_path)?;
        let mut header: [u8; 16];
        let test = rom_file.read_exact(&mut header);

        // Get format
        assert!((header[0] == 0x4E) && (header[1] == 0x45) && (header[2] == 0x53) && (header[3] == 0x1A), "Rom does not have iNES header");
        let header_ver2 = (header[7] & 0x0C) == 0x08;

        self.mapper = MapperEnum::DefaultMapper(DefaultMapper::new());
        self.cpu.set_mapper(self.mapper);
        self.cpu.test();
    }
}