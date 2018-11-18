pub mod cpu;
pub mod mmu;

mod bus;
mod cartridge;
mod interrupt;
mod ram;

pub struct GameBoy {
    cpu: cpu::Cpu,
    mmu: mmu::Mmu,
}
