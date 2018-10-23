mod cpu;
mod ram;
mod rom;

pub struct Context {
    cpu: cpu::Cpu,
    ram: ram::Ram,
}
