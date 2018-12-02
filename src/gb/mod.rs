pub mod cartridge;
pub mod cpu;
pub mod mmu;
pub mod ppu;
pub mod screen;

mod bus;
mod interrupt;
mod ram;

use self::cartridge::Cartridge;
use self::cpu::Cpu;
use self::mmu::Mmu;
use self::ppu::Ppu;
use self::screen::Screen;

pub struct GameBoy {
    cpu: Cpu,
    ppu: Ppu,
    mmu: Mmu,
    screen: Screen,

    paused: bool,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mmu: Mmu::new(),
            screen: Screen::new(),

            paused: true,
        }
    }

    pub fn load(&mut self, cart: Cartridge) {
        self.cpu.simulate_bootloader();
        self.ppu = Ppu::new();
        self.mmu.simulate_bootloader();
        self.mmu.load(0, cart.read());
    }

    pub fn step(&mut self) -> Vec<u8> {
        if self.paused {
            return self.screen.dump();
        }

        loop {
            let cycle = self.cpu.step(&mut self.mmu);
            self.ppu.step(&mut self.mmu, cycle);
            if self.ppu.is_screen_prepared() {
                break;
            }
        }

        self.screen.refresh(&self.ppu.transfer_screen());
        self.screen.dump()
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }
}
