mod cartridge;
mod cpu;
mod memory;
mod ram;

pub struct Context {
    registers: cpu::register::Registers,
    ram: ram::Ram,
}
