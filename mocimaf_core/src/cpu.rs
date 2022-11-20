
use crate::NES;
use crate::mappers::Mapper;

pub struct CPU {
    cycle: u64,

    reg_pc: u16,
    reg_s: u8,
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,

    flag_n: bool,
    flag_v: bool,
    flag_d: bool,
    flag_i: bool,
    flag_z: bool,
    flag_c: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cycle: 0,

            reg_pc: 0,
            reg_s: 0,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,

            flag_n: false,
            flag_v: false,
            flag_d: false,
            flag_i: false,
            flag_z: false,
            flag_c: false,
        }
    }
}

impl NES {
    pub fn cpu_reset(&mut self) {
        self.cpu.reg_s -= 3;
        self.cpu.flag_i = true;
    }

    pub fn cpu_step(&mut self) {
        cpu_opcode();
    }

    fn cpu_opcode(&mut self) {
        let opcode = fetch(self);
        let _aaa = opcode >> 5;
        let _bbb = (opcode >> 2) & 7;
        let _cc = opcode & 3;

        match opcode {
            0x09 => addressing_mode_rm(self, _bbb, ora),
            _ => panic!("Unimplemented opcode 0x{:0>2X} at 0x{:0>4X}", opcode, self.cpu.reg_pc - 1)
        }
    }

    fn cpu_read(&mut self, address: u16) -> u8 {
        self.cpu.cycle += 1;

        return match address {
            0x0000..=0x1FFF => self.wram[address as usize & 0x7FF],
            _ => self.mapper.cpu_read(address)
        }
    }

    fn cpu_write(&mut self, address: u16, value: u8) {
        self.cpu.cycle += 1;

        match address {
            0x0000..=0x1FFF => self.wram[address as usize & 0x7FF] = value,
            _ => self.mapper.cpu_write(address, value)
        }
    }
}

fn fetch(emulator: &mut NES) -> u8 {
    let val = emulator.cpu_read(emulator.cpu.reg_pc);
    emulator.cpu.reg_pc += 1;

    val
}

fn addressing_mode_rm(emulator: &mut NES, mode: u8, op: fn(&mut CPU, u8)) {
    op(&mut emulator.cpu, match mode {
        2 => fetch(emulator),
        _ => panic!()
    })
}

fn ora(cpu: &mut CPU, val: u8) {
    cpu.reg_a |= val;
    cpu.flag_n = (cpu.reg_a & 0x80) != 0;
    cpu.flag_z = cpu.reg_a == 0;
}