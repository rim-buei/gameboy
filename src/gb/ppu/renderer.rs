use super::super::bus::Bus;
use super::super::screen::{FrameBuffer, Pixel, SCREEN_H, SCREEN_W};
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
        let y = LY.read(self.bus);
        let control = LCDControl::new(LCDC.read(self.bus));

        if control.bgwin_enabled() {
            self.render_background_scanline(y);

            if control.win_enabled() {
                self.render_window_scanline(y);
            }
        }

        if control.obj_enabled() {
            self.render_obj_scanline(y);
        }
    }

    fn render_background_scanline(&mut self, y: u8) {
        let tiles_loc = LCDControl::new(LCDC.read(self.bus)).bgwin_tile_loc();
        let map_loc = LCDControl::new(LCDC.read(self.bus)).bg_map_loc();

        let scroll_x = SCX.read(self.bus);
        let scroll_y = SCY.read(self.bus);

        let y_adjusted = y.wrapping_add(scroll_y);
        let tile_row: u16 = y_adjusted as u16 / 8 * 32;

        for x in 0..SCREEN_W {
            let x_adjusted = x.wrapping_add(scroll_x);
            let tile_col: u16 = x_adjusted as u16 / 8;

            let tile_addr = map_loc + tile_row + tile_col;
            let tile_loc = if tiles_loc == 0x8800 {
                let tile_n = (self.bus.read8(tile_addr) as i8) as u16 + 128;
                tiles_loc + (tile_n * 16) + ((y_adjusted % 8) as u16 * 2)
            } else {
                let tile_n = self.bus.read8(tile_addr) as u16;
                tiles_loc + (tile_n * 16) + ((y_adjusted % 8) as u16 * 2)
            };

            let byte1 = self.bus.read8(tile_loc);
            let byte2 = self.bus.read8(tile_loc + 1);

            let color_bit = 7 - ((x_adjusted % 8) as u8);
            let (r, g, b) = self.pick_rgb(color_bit, byte1, byte2);

            self.frame_buffer.set_pixel(x, y, Pixel(r, g, b, 255));
        }
    }

    // TODO: This implementation is almost same as background
    fn render_window_scanline(&mut self, y: u8) {
        let tiles_loc = LCDControl::new(LCDC.read(self.bus)).bgwin_tile_loc();
        let map_loc = LCDControl::new(LCDC.read(self.bus)).win_map_loc();

        let window_x = WX.read(self.bus).wrapping_sub(7);
        let window_y = WY.read(self.bus);
        if window_y > y || window_x >= SCREEN_W || window_y >= SCREEN_H {
            return;
        }

        let y_adjusted = y.wrapping_sub(window_y);
        let tile_row: u16 = y_adjusted as u16 / 8 * 32;

        for x in 0..SCREEN_W {
            let x_adjusted = x.wrapping_sub(window_x);
            let tile_col: u16 = x_adjusted as u16 / 8;

            let tile_addr = map_loc + tile_row + tile_col;
            let tile_loc = if tiles_loc == 0x8800 {
                let tile_n = (self.bus.read8(tile_addr) as i8) as u16 + 128;
                tiles_loc + (tile_n * 16) + ((y_adjusted % 8) as u16 * 2)
            } else {
                let tile_n = self.bus.read8(tile_addr) as u16;
                tiles_loc + (tile_n * 16) + ((y_adjusted % 8) as u16 * 2)
            };

            let byte1 = self.bus.read8(tile_loc);
            let byte2 = self.bus.read8(tile_loc + 1);

            let color_bit = 7 - ((x_adjusted % 8) as u8);
            let (r, g, b) = self.pick_rgb(color_bit, byte1, byte2);

            self.frame_buffer.set_pixel(x, y, Pixel(r, g, b, 255));
        }
    }

    fn render_obj_scanline(&mut self, y: u8) {}

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
