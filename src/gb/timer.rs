use super::bus::Bus;

pub struct Timer {}

impl Timer {
    pub fn new() -> Self {
        Timer {}
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B, cycle: u8) {}
}
