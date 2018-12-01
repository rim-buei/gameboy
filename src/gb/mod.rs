pub mod cpu;
pub mod mmu;
pub mod ppu;
pub mod screen;

mod bus;
mod cartridge;
mod interrupt;
mod ram;

use self::cpu::Cpu;
use self::mmu::Mmu;
use self::ppu::Ppu;
use self::screen::Screen;

pub struct GameBoy {
    cpu: Cpu,
    ppu: Ppu,
    mmu: Mmu,
    screen: Screen,
}

impl GameBoy {
    pub fn new() -> Self {
        let mut mmu = Mmu::new();

        GameBoy {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mmu: mmu,
            screen: Screen::new(),
        }
    }

    pub fn step(&mut self) {
        loop {
            let cycle = self.cpu.step(&mut self.mmu);
            self.ppu.step(&mut self.mmu, cycle);
            if self.ppu.is_screen_prepared() {
                break;
            }
        }
    }

    pub fn screen(&mut self) -> Vec<u8> {
        self.screen.refresh(&self.ppu.transfer_screen());
        self.screen.dump()
    }
}
