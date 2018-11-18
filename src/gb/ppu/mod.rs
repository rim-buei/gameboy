use super::bus::Bus;
use super::interrupt::{request as irq, Interrupt};
use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Mode {
    HBlank,   // Mode 0
    VBlank,   // Mode 1
    OAMRead,  // Mode 2
    VRAMRead, // Mode 3
}

const SCREEN_W: u8 = 160;
const SCREEN_H: u8 = 144;

const ONE_CYCLE: u16 = 456;

pub struct Ppu {
    state: State,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { state: State::new() }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B, cycle: u8) {
        match self.state.mode {
            Mode::HBlank => {}
            Mode::VBlank => {}
            Mode::OAMRead => {}
            Mode::VRAMRead => {}
        }

        self.state.clock += cycle as u16;
        if self.state.clock < ONE_CYCLE {
            return;
        }

        self.state.clock -= ONE_CYCLE;
        self.state.ly += 1;

        if self.state.ly < SCREEN_H {
            self.render_scanline();
        } else if self.state.ly == SCREEN_H {
            irq(bus, Interrupt::VBlank);
        } else if self.state.ly > SCREEN_H + 9 {
            self.state.ly = 0;
        }
    }

    fn render_scanline(&mut self) {
        self.render_background_scanline();
        self.render_window_scanline();
        self.render_sprites_scanline();
    }

    fn render_background_scanline(&mut self) {}

    fn render_window_scanline(&mut self) {}

    fn render_sprites_scanline(&mut self) {}
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[PPU] Mode: {:?}, Clock: {}", self.state.mode, self.state.clock)
    }
}

pub struct State {
    mode: Mode,

    clock: u16,
    ly: u8,
}

impl State {
    fn new() -> Self {
        State {
            mode: Mode::OAMRead,

            clock: 0,
            ly: 0,
        }
    }
}
