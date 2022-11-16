use crate::mappers::Mapper;

pub struct DefaultMapper {
    prom: Vec<u8>
}

impl DefaultMapper {
    fn new() -> Self {
        todo!()
    }
}

impl Mapper for DefaultMapper {
    fn cpu_read(&self, address: u16) -> u8 {
        todo!()
    }

    fn cpu_write(&self, address: u16, value: u8) {
        println!("Hello from default mapper!");
        todo!()
    }

    fn ppu_read(&self, address: u16) -> u8 {
        todo!()
    }

    fn ppu_write(&self, address: u16, value: u8) {
        todo!()
    }
}