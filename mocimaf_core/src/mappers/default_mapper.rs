
use crate::mappers::Mapper;
use crate::NES;

pub struct DefaultMapper {
    prom: Vec<u8>
}

impl DefaultMapper {
    pub fn new() -> Self {
        todo!()
    }
}

impl Mapper for DefaultMapper {
    fn cpu_read(&self, /*emulator: &mut NES, */address: u16) -> u8 {
        todo!()
    }

    fn cpu_write(&self, /*emulator: &mut NES, */address: u16, value: u8) {
        println!("Hello from default mapper CPU! {} {}", address, value);
        todo!()
    }

    fn ppu_read(&self, address: u16) -> u8 {
        todo!()
    }

    fn ppu_write(&self, address: u16, value: u8) {
        println!("Hello from default mapper PPU! {} {}", address, value);
        todo!()
    }
}