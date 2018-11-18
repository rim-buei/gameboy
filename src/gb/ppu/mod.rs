mod register;

use self::register::Register::*;
use super::bus::Bus;
use super::interrupt::{request as request_interrupt, Interrupt};
use std::fmt;

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
        self.update_lcd_status(bus);

        {
            self.state.clock += cycle as u16;
            if self.state.clock < ONE_CYCLE {
                return;
            }
            self.state.clock -= ONE_CYCLE;
        }

        {
            let mut next_line = LY.read(bus) + 1;
            if next_line == SCREEN_H {
                request_interrupt(bus, Interrupt::VBlank);
            } else if next_line > SCREEN_H + 9 {
                next_line = 0;
            } else {
                self.render_scanline();
            }
            LY.write(bus, next_line);
        }
    }

    fn update_lcd_status<B: Bus>(&mut self, bus: &mut B) {
        let mut status = LCDStatus(STAT.read(bus));

        let cur_line = LY.read(bus);
        let cur_mode = status.mode();

        let interrupt = if cur_line >= SCREEN_H {
            status.set_mode(Mode::VBlank);
            status.is_vblank_interrupt_enabled()
        } else {
            match self.state.clock {
                0...79 => {
                    status.set_mode(Mode::OAMRead);
                    status.is_oam_interrupt_enabled()
                }
                80...251 => {
                    status.set_mode(Mode::VRAMRead);
                    false
                }
                _ => {
                    status.set_mode(Mode::HBlank);
                    status.is_hblank_interrupt_enabled()
                }
            }
        };

        if interrupt && status.mode() != cur_mode {
            request_interrupt(bus, Interrupt::LCDStat);
        }

        if cur_line == LYC.read(bus) {
            status.set_lyc_coincidence(true);

            if status.is_lyc_coincidence_interrupt_enabled() {
                request_interrupt(bus, Interrupt::LCDStat);
            }
        } else {
            status.set_lyc_coincidence(false);
        }

        self.state.lcd = status;
        STAT.write(bus, status.0);
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
        write!(
            f,
            "[PPU] Mode: {:?}, Clock: {}",
            self.state.lcd.mode(),
            self.state.clock
        )
    }
}

pub struct State {
    clock: u16,

    lcd: LCDStatus, // For debugging
}

impl State {
    fn new() -> Self {
        State {
            clock: 0,

            lcd: LCDStatus(0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    HBlank,   // Mode 0
    VBlank,   // Mode 1
    OAMRead,  // Mode 2
    VRAMRead, // Mode 3
}

#[derive(Copy, Clone)]
struct LCDStatus(u8);

impl LCDStatus {
    fn new(v: u8) -> Self {
        LCDStatus(v)
    }

    fn mode(&self) -> Mode {
        match self.0 & 0b0000_0011 {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OAMRead,
            3 => Mode::VRAMRead,
            _ => panic!("unreachable"),
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.0 &= 0b1111_1100;
        self.0 |= match mode {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAMRead => 2,
            Mode::VRAMRead => 3,
        };
    }

    fn set_lyc_coincidence(&mut self, v: bool) {
        self.0 &= 0b1111_1011;
        if v {
            self.0 |= 0b0100;
        }
    }

    fn is_hblank_interrupt_enabled(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    fn is_vblank_interrupt_enabled(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    fn is_oam_interrupt_enabled(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    fn is_lyc_coincidence_interrupt_enabled(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }
}
