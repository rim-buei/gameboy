use super::bus::Bus;

use super::interrupt::{self, Interrupt};

const DIV_REG_ADDR: u16 = 0xFF04;
const TIMA_REG_ADDR: u16 = 0xFF05;
const TMA_REG_ADDR: u16 = 0xFF06;
const TAC_REG_ADDR: u16 = 0xFF07;

pub struct Timer {
    divider: Counter,
    timer: Counter,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            divider: Counter { freq: 256, cycles: 0 },
            timer: Counter { freq: 1024, cycles: 0 },
        }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B, cycle: u8) {
        {
            let overflowed = self.divider.inc(cycle as u16);
            if overflowed {
                self.inc_divider_reg(bus);
            }
        }

        if self.is_timer_enabled(bus) {
            self.update_timer_freq(bus);

            let overflowed = self.timer.inc(cycle as u16);
            if overflowed {
                self.inc_timer_reg(bus);
            }
        }
    }

    fn is_timer_enabled<B: Bus>(&self, bus: &mut B) -> bool {
        bus.read8(TAC_REG_ADDR) & 0b0100 == 0b0100
    }

    fn update_timer_freq<B: Bus>(&mut self, bus: &mut B) {
        match bus.read8(TAC_REG_ADDR) & 0b0011 {
            0b0000 => self.timer.freq = 1024,
            0b0001 => self.timer.freq = 16,
            0b0010 => self.timer.freq = 64,
            0b0011 => self.timer.freq = 256,
            _ => unreachable!(),
        };
    }

    fn inc_divider_reg<B: Bus>(&mut self, bus: &mut B) {
        let v = bus.read8(DIV_REG_ADDR).wrapping_add(1);
        bus.write8(DIV_REG_ADDR, v);
    }

    fn inc_timer_reg<B: Bus>(&mut self, bus: &mut B) {
        let v = bus.read8(TIMA_REG_ADDR);
        if v == 0xFF {
            interrupt::request(bus, Interrupt::Timer);
            bus.write8(TIMA_REG_ADDR, bus.read8(TMA_REG_ADDR));
        } else {
            bus.write8(TIMA_REG_ADDR, v + 1);
        }
    }
}

struct Counter {
    freq: u16,
    cycles: u16,
}

impl Counter {
    // inc returns true when overflowed
    fn inc(&mut self, n: u16) -> bool {
        self.cycles += n;

        if self.cycles >= self.freq {
            self.cycles -= self.freq;
            return true;
        }

        false
    }
}
