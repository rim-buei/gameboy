use super::super::bus::Bus;
use super::super::screen::FrameBuffer;

pub struct Renderer<'a, B: Bus + 'a> {
    frame_buffer: &'a mut FrameBuffer,
    bus: &'a mut B,
}

impl<'a, B: Bus + 'a> Renderer<'a, B> {
    pub fn new(frame_buffer: &'a mut FrameBuffer, bus: &'a mut B) -> Self {
        Renderer { frame_buffer, bus }
    }

    pub fn render_scanline(&mut self) {}
    fn render_bg(&mut self) {}
    fn render_win(&mut self) {}
    fn render_obj(&mut self) {}
}
