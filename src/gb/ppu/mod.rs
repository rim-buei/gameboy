mod register;
mod renderer;

use self::register::{LCDControl, LCDStatus, Register::*};
use self::renderer::Renderer;
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

    debug_info: Vec<String>, // For debugging
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            state: State::new(),
            screen: FrameBuffer::new(),
            screen_buffer: FrameBuffer::new(),

            debug_info: vec![],
        }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B, cycle: u8) {
        self.update_debug_info(bus);
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

    fn update_debug_info<B: Bus>(&mut self, bus: &mut B) {
        self.debug_info.clear();

        let lcdc = LCDControl::new(LCDC.read(bus));
        self.debug_info.push(format!(
            "LCD_ON: {}, BGWIN_ON: {}, WIN_ON: {}, OBJ_ON: {}",
            lcdc.lcd_enabled(),
            lcdc.bgwin_enabled(),
            lcdc.win_enabled(),
            lcdc.obj_enabled(),
        ));
        self.debug_info.push(format!(
            "TILE_LOC: 0x{:04X}, BG_MAP_LOC: 0x{:04X}, WIN_MAP_LOC: 0x{:04X}",
            lcdc.bgwin_tile_loc(),
            lcdc.bg_map_loc(),
            lcdc.win_map_loc(),
        ));

        let ly = LY.read(bus);
        let lyc = LYC.read(bus);
        self.debug_info.push(format!("LY: {}, LYC: {}", ly, lyc));

        let scy = SCY.read(bus);
        let scx = SCX.read(bus);
        let wy = WY.read(bus);
        let wx = WX.read(bus);
        self.debug_info
            .push(format!("SCY: {}, SCX: {}, WY: {}, WX: {}", scy, scx, wy, wx));
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
                        let mut renderer = Renderer::new(&mut self.screen_buffer, bus);
                        renderer.render_scanline();
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
}

impl fmt::Debug for Ppu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ss: Vec<String> = self.debug_info.iter().map(|s| format!("[PPU] {}", s)).collect();
        write!(f, "{}", ss.join("\n"))
    }
}

pub struct State {
    clock: u16,
    line_drawn: bool,
    screen_prepared: bool,
}

impl State {
    fn new() -> Self {
        State {
            clock: 0,
            line_drawn: false,
            screen_prepared: false,
        }
    }
}
