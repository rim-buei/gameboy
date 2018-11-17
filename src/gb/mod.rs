pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod interrupt;
pub mod mmu;
pub mod ram;

pub struct Context {
    registers: cpu::register::Registers,
    ram: ram::Ram,
}
