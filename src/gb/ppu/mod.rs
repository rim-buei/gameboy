mod register;

use self::register::{LCDStatus, Register::*};
use super::bus::Bus;
use super::interrupt::{request as request_interrupt, Interrupt};
use super::screen::{FrameBuffer, SCREEN_H};
use std::fmt;

const ONE_CYCLE: u16 = 456;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    HBlank,   // Mode 0
    VBlank,   // Mode 1
    OAMRead,  // Mode 2
    VRAMRead, // Mode 3
}

pub struct Ppu {
    state: State,
    screen: FrameBuffer,
    screen_buffer: FrameBuffer,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            state: State::new(),
            screen: FrameBuffer::new(),
            screen_buffer: FrameBuffer::new(),
        }
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
                self.screen = self.screen_buffer;
                self.state.screen_prepared = true;

                next_line = 0;
            }
            LY.write(bus, next_line);

            self.state.line_drawn = false;
        }
    }

    fn update_lcd_status<B: Bus>(&mut self, bus: &mut B) {
        let mut status = LCDStatus::new(STAT.read(bus));

        let cur_line = LY.read(bus);
        let cur_mode = status.mode();

        let interrupt = if cur_line >= SCREEN_H {
            status.set_mode(Mode::VBlank);
            status.vblank_interrupt_enabled()
        } else {
            match self.state.clock {
                0...79 => {
                    status.set_mode(Mode::OAMRead);
                    status.oam_interrupt_enabled()
                }
                80...251 => {
                    if !self.state.line_drawn {
                        self.render_scanline(bus);
                        self.state.line_drawn = true;
                    }

                    status.set_mode(Mode::VRAMRead);
                    false
                }
                _ => {
                    status.set_mode(Mode::HBlank);
                    status.hblank_interrupt_enabled()
                }
            }
        };

        if interrupt && status.mode() != cur_mode {
            request_interrupt(bus, Interrupt::LCDStat);
        }

        if cur_line == LYC.read(bus) {
            status.set_lyc_coincidence(true);

            if status.lyc_coincidence_interrupt_enabled() {
                request_interrupt(bus, Interrupt::LCDStat);
            }
        } else {
            status.set_lyc_coincidence(false);
        }

        self.state.lcd = status;
        STAT.write(bus, status.raw());
    }

    pub fn is_screen_prepared(&self) -> bool {
        self.state.screen_prepared
    }

    pub fn transfer_screen(&mut self) -> FrameBuffer {
        if !self.state.screen_prepared {
            panic!("screen data is still not yet prepared")
        }

        self.state.screen_prepared = false;
        self.screen
    }

    fn render_scanline<B: Bus>(&mut self, bus: &mut B) {
        self.render_bg(bus);
        self.render_win(bus);
        self.render_obj(bus);
    }

    fn render_bg<B: Bus>(&mut self, bus: &mut B) {}
    fn render_win<B: Bus>(&mut self, bus: &mut B) {}
    fn render_obj<B: Bus>(&mut self, bus: &mut B) {}
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
    line_drawn: bool,
    screen_prepared: bool,

    lcd: LCDStatus, // For debugging
}

impl State {
    fn new() -> Self {
        State {
            clock: 0,
            line_drawn: false,
            screen_prepared: false,

            lcd: LCDStatus::new(0),
        }
    }
}
