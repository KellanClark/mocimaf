
use crate::NES;
use crate::mappers::Mapper;

pub struct PPU {
    value: i32
}

impl PPU {
    pub fn new() -> Self {
        Self {
            value: 0
        }
    }
}

impl NES {
    fn ppu_read(&mut self, address: u16) -> u8 {
        return self.mapper.ppu_read(address)
    }

    fn ppu_write(&mut self, address: u16, value: u8) {
        self.mapper.ppu_write(address, value);
    }
}