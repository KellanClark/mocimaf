
pub struct CPU {
    bus: crate::mappers::MapperEnum,
    value: i32
}

impl CPU {
    pub fn set_mapper(&mut self, new_mapper: crate::mappers::MapperEnum) {
        self.bus = new_mapper;
    }

    pub fn test(&self) {
        self.bus.cpu_write(0, 0);
    }
}