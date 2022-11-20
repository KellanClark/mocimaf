
pub mod default_mapper;

use enum_dispatch::enum_dispatch;
use default_mapper::DefaultMapper;
use crate::NES;

#[enum_dispatch]
pub enum MapperEnum {
    DefaultMapper
}

#[enum_dispatch(MapperEnum)]
pub(crate) trait Mapper {
    fn cpu_read(&self, /*emulator: &mut NES, */address: u16) -> u8;
    fn cpu_write(&self, /*emulator: &mut NES, */address: u16, value: u8);
    fn ppu_read(&self, /*emulator: &mut NES, */address: u16) -> u8;
    fn ppu_write(&self, /*emulator: &mut NES, */address: u16, value: u8);
}