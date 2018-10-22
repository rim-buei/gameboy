mod cpu;
mod memory;

pub struct Context {
    cpu: cpu::Cpu,
    memory: memory::Memory,
}
