use super::super::bus::Bus;
use super::super::screen::{FrameBuffer, Pixel, SCREEN_W};
use super::register::{LCDControl, Register::*};

pub struct Renderer<'a, B: Bus + 'a> {
    frame_buffer: &'a mut FrameBuffer,
    bus: &'a mut B,
}

impl<'a, B: Bus + 'a> Renderer<'a, B> {
    pub fn new(frame_buffer: &'a mut FrameBuffer, bus: &'a mut B) -> Self {
        Renderer { frame_buffer, bus }
    }

    pub fn render_scanline(&mut self) {
        let control = LCDControl::new(LCDC.read(self.bus));

        if control.bgwin_enabled() {
            self.render_bg_scanline();
            self.render_win_scanline();
        }

        if control.obj_enabled() {
            self.render_obj_scanline();
        }
    }

    fn render_bg_scanline(&mut self) {
        let tiles_loc = LCDControl::new(LCDC.read(self.bus)).bgwin_tile_loc();
        let map_loc = LCDControl::new(LCDC.read(self.bus)).bg_map_loc();
        let scroll_y = SCY.read(self.bus);
        let scroll_x = SCX.read(self.bus);

        let y = LY.read(self.bus);
        let y_adjusted = (y as u16 + scroll_y as u16) as u8;
        let tile_offset = (y_adjusted / 8) as u16 * 32;

        for tile_n in 0..32 {
            let tile_id = if tiles_loc == 0x8800 {
                (self.bus.read8(map_loc + tile_offset + tile_n) as i16 + 128) as u8
            } else {
                self.bus.read8(map_loc + tile_offset + tile_n)
            };

            let tile_addr = tiles_loc + (tile_id as u16) * 16 + (y_adjusted % 8 * 2) as u16;

            let byte1 = self.bus.read8(tile_addr);
            let byte2 = self.bus.read8(tile_addr + 1);

            let x = (tile_n * 8) as u8;
            for tile_x in 0..8 {
                if x + tile_x < SCREEN_W {
                    let color_bit = 7 - tile_x;
                    let (r, g, b) = self.pick_rgb(color_bit, byte1, byte2);

                    self.frame_buffer.set_pixel(x + tile_x, y, Pixel(r, g, b, 255));
                }
            }
        }
    }

    fn render_win_scanline(&mut self) {}
    fn render_obj_scanline(&mut self) {}

    fn pick_rgb(&mut self, bit: u8, byte1: u8, byte2: u8) -> (u8, u8, u8) {
        let lo = (byte1 & (1 << bit) != 0) as u8;
        let hi = (byte2 & (1 << bit) != 0) as u8;

        let color_num = (hi << 1) | lo;

        let palette = BGP.read(self.bus);
        let color = ((palette >> (color_num * 2)) & 0b11) as usize;

        PALETTE[color]
    }
}

const PALETTE: [(u8, u8, u8); 4] = [
    (0x9B, 0xBC, 0x0F),
    (0x8B, 0xAC, 0x0F),
    (0x30, 0x62, 0x30),
    (0x0F, 0x38, 0x0F),
];
