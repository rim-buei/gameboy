mod cpu;
mod ram;
mod rom;

pub struct Context {
    registers: cpu::register::Registers,
    ram: ram::Ram,
}
