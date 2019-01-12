use super::super::bus::Bus;
use super::super::screen::{FrameBuffer, Pixel, SCREEN_H, SCREEN_W};
use super::register::{LCDControl, Register::*};

pub struct Renderer<'a, B: Bus + 'a> {
    frame_buffer: &'a mut FrameBuffer,
    bus: &'a mut B,

    bgwin_colors: [u8; SCREEN_W as usize],
}

impl<'a, B: Bus + 'a> Renderer<'a, B> {
    pub fn new(frame_buffer: &'a mut FrameBuffer, bus: &'a mut B) -> Self {
        Renderer {
            frame_buffer,
            bus,
            bgwin_colors: [0; SCREEN_W as usize],
        }
    }

    pub fn render_scanline(&mut self) {
        self.bgwin_colors = [0; SCREEN_W as usize];

        let y = LY.read(self.bus);
        let control = LCDControl::new(LCDC.read(self.bus));

        if control.bgwin_enabled() {
            self.render_background_scanline(y);

            if control.win_enabled() {
                self.render_window_scanline(y);
            }
        }

        if control.obj_enabled() {
            self.render_sprites_scanline(y);
        }
    }

    fn render_background_scanline(&mut self, y: u8) {
        let tiles_loc = LCDControl::new(LCDC.read(self.bus)).bgwin_tile_loc();
        let map_loc = LCDControl::new(LCDC.read(self.bus)).bg_map_loc();
        let palette = BGP.read(self.bus);

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
            let color_n = get_color_number(color_bit, byte1, byte2);

            let (r, g, b) = get_rgb(palette, color_n);
            self.frame_buffer.set_pixel(x, y, Pixel(r, g, b, 255));
            self.bgwin_colors[x as usize] = color_n;
        }
    }

    // TODO: This implementation is almost same as background
    fn render_window_scanline(&mut self, y: u8) {
        let tiles_loc = LCDControl::new(LCDC.read(self.bus)).bgwin_tile_loc();
        let map_loc = LCDControl::new(LCDC.read(self.bus)).win_map_loc();
        let palette = BGP.read(self.bus);

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
            let color_n = get_color_number(color_bit, byte1, byte2);

            let (r, g, b) = get_rgb(palette, color_n);
            self.frame_buffer.set_pixel(x, y, Pixel(r, g, b, 255));
            self.bgwin_colors[x as usize] = color_n;
        }
    }

    fn render_sprites_scanline(&mut self, y: u8) {
        let y = y as i16;
        let tiles_loc = 0x8000;
        let sprite_height = LCDControl::new(LCDC.read(self.bus)).obj_height() as i16;

        let palette0 = OBP0.read(self.bus);
        let palette1 = OBP1.read(self.bus);

        for sprite_n in 0..40 {
            let offset: u16 = sprite_n * 4;

            let sprite_x = (self.bus.read8(0xFE00 + offset + 1) as i16) - 8;
            let sprite_y = (self.bus.read8(0xFE00 + offset) as i16) - 16;
            if sprite_y > y || (sprite_y + sprite_height) <= y {
                continue;
            }

            let attrs = self.bus.read8(0xFE00 + offset + 3);
            let palette = if attrs & (1 << 4) != 0 { palette1 } else { palette0 };
            let x_flip = attrs & (1 << 5) != 0;
            let y_flip = attrs & (1 << 6) != 0;
            let priority = attrs & (1 << 7) != 0;

            let tile_y = if y_flip {
                sprite_height - 1 - y - sprite_y
            } else {
                y - sprite_y
            };

            let tile_n = self.bus.read8(0xFE00 + offset + 2) as u16;
            let tile_loc = tiles_loc + (tile_n * 16) + (tile_y * 2) as u16;

            let byte1 = self.bus.read8(tile_loc);
            let byte2 = self.bus.read8(tile_loc + 1);

            for tile_x in 0..8 {
                let x = sprite_x + tile_x;
                if x < 0 || SCREEN_W as i16 <= x {
                    continue;
                }

                let color_bit = if x_flip { tile_x } else { 7 - tile_x };
                let color_n = get_color_number(color_bit as u8, byte1, byte2);
                if color_n == 0 {
                    continue;
                }

                let (r, g, b) = get_rgb(palette, color_n);
                if !priority || self.bgwin_colors[x as usize] == 0 {
                    self.frame_buffer.set_pixel(x as u8, y as u8, Pixel(r, g, b, 255));
                }
            }
        }
    }
}

fn get_color_number(bit: u8, byte1: u8, byte2: u8) -> u8 {
    let lo = (byte1 & (1 << bit) != 0) as u8;
    let hi = (byte2 & (1 << bit) != 0) as u8;
    (hi << 1) | lo
}

fn get_rgb(palette: u8, color_n: u8) -> (u8, u8, u8) {
    let color = (palette >> (color_n * 2)) & 0b11;

    PALETTE[color as usize]
}

const PALETTE: [(u8, u8, u8); 4] = [
    (0x9B, 0xBC, 0x0F),
    (0x8B, 0xAC, 0x0F),
    (0x30, 0x62, 0x30),
    (0x0F, 0x38, 0x0F),
];
