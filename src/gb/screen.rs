pub const SCREEN_W: u8 = 160;
pub const SCREEN_H: u8 = 144;
const SCREEN_W_SZ: usize = SCREEN_W as usize;
const SCREEN_H_SZ: usize = SCREEN_H as usize;

#[derive(Copy, Clone)]
pub struct Pixel(pub u8, pub u8, pub u8, pub u8);

#[derive(Copy, Clone)]
pub struct FrameBuffer {
    data: [[Pixel; SCREEN_W_SZ]; SCREEN_H_SZ],
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            data: [[Pixel(0, 0, 0, 0); SCREEN_W_SZ]; SCREEN_H_SZ],
        }
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> Pixel {
        self.data[y as usize][x as usize]
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, pixel: Pixel) {
        self.data[y as usize][x as usize] = pixel;
    }
}

pub struct Screen {
    frame_buffer: FrameBuffer,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            frame_buffer: FrameBuffer::new(),
        }
    }

    pub fn refresh(&mut self, frame_buffer: &FrameBuffer) {
        self.frame_buffer = *frame_buffer;
    }

    pub fn dump(&self) -> Vec<u8> {
        let wxh = SCREEN_W as u16 * SCREEN_H as u16;
        let mut array: Vec<u8> = Vec::with_capacity(4 * wxh as usize);

        for i in 0..wxh {
            let x = (i % SCREEN_W as u16) as u8;
            let y = (i / SCREEN_W as u16) as u8;
            let pixel = self.frame_buffer.get_pixel(x, y);

            array.push(pixel.0);
            array.push(pixel.1);
            array.push(pixel.2);
            array.push(pixel.3);
        }

        array
    }
}
