pub mod cpu;
pub mod mmu;
pub mod ppu;

mod bus;
mod cartridge;
mod interrupt;
mod ram;

pub struct GameBoy {
    cpu: cpu::Cpu,
    ppu: ppu::Ppu,
    mmu: mmu::Mmu,
}
