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
            self.render_tile_scanline();
        }

        if control.obj_enabled() {
            self.render_obj_scanline();
        }
    }

    fn rendering_window(&mut self) -> bool {
        let y = LY.read(self.bus);
        let window_y = WY.read(self.bus);
        let window_enabled = LCDControl::new(LCDC.read(self.bus)).win_enabled();
        window_enabled && (y >= window_y)
    }

    fn get_map_location(&mut self) -> u16 {
        if self.rendering_window() {
            LCDControl::new(LCDC.read(self.bus)).win_map_loc()
        } else {
            LCDControl::new(LCDC.read(self.bus)).bg_map_loc()
        }
    }

    fn render_tile_scanline(&mut self) {
        let tiles_loc = LCDControl::new(LCDC.read(self.bus)).bgwin_tile_loc();
        let map_loc = self.get_map_location();

        let scroll_x = SCX.read(self.bus);
        let scroll_y = SCY.read(self.bus);
        let window_x = WX.read(self.bus) - 7;
        let window_y = WY.read(self.bus);

        let y = LY.read(self.bus);

        let y_adjusted: u16 = if self.rendering_window() {
            y as u16 - window_y as u16
        } else {
            y as u16 + scroll_y as u16
        };

        let tile_row = y_adjusted / 8 * 32;

        for x in 0..SCREEN_W {
            let x_adjusted: u16 = if self.rendering_window() && (x >= window_x) {
                x as u16 - window_x as u16
            } else {
                x as u16 + scroll_x as u16
            };

            let tile_col = x_adjusted / 8;

            let tile_addr = map_loc + tile_row + tile_col;
            let tile_loc = if tiles_loc == 0x8800 {
                let tile_n = (self.bus.read8(tile_addr) as i8) as u16 + 128;
                tiles_loc + (tile_n * 16) + ((y_adjusted % 8) * 2)
            } else {
                let tile_n = self.bus.read8(tile_addr) as u16;
                tiles_loc + (tile_n * 16) + ((y_adjusted % 8) * 2)
            };

            let byte1 = self.bus.read8(tile_loc);
            let byte2 = self.bus.read8(tile_loc + 1);

            let color_bit = 7 - ((x_adjusted % 8) as u8);
            let (r, g, b) = self.pick_rgb(color_bit, byte1, byte2);

            self.frame_buffer.set_pixel(x, y, Pixel(r, g, b, 255));
        }
    }

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
