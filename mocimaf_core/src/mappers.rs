
pub mod default_mapper;

use enum_dispatch::enum_dispatch;
use default_mapper::DefaultMapper;

#[enum_dispatch]
pub enum MapperEnum {
    DefaultMapper
}

#[enum_dispatch(MapperEnum)]
trait Mapper {
    //fn new() -> Self;
    fn cpu_read(&self, address: u16) -> u8;
    fn cpu_write(&self, address: u16, value: u8);
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&self, address: u16, value: u8);
}