mod cpu;
mod ram;
mod rom;

pub struct Context {
    registers: cpu::registers::Registers,
    ram: ram::Ram,
}
