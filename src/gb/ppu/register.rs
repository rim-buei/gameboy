use super::super::bus::Bus;

#[derive(Debug, Copy, Clone)]
pub enum Register {
    LCDC, // LCD Control
    STAT, // LCD Control Status
    SCY,  // Scroll Y
    SCX,  // Scroll X
    LY,   // Y-Coordinate
    LYC,  // LY Compare
    DMA,  // DMA Transfer and Start Address
    BGP,  // BG Palette Data
    OBP0, // Object Palette 0 Data
    OBP1, // Object Palette 1 Data
    WY,   // Window Y Position
    WX,   // Window X Position - 7
}

impl Register {
    pub fn read<B: Bus>(&self, bus: &mut B) -> u8 {
        bus.read8(self.address())
    }

    pub fn write<B: Bus>(&self, bus: &mut B, v: u8) {
        bus.write8(self.address(), v);
    }

    fn address(&self) -> u16 {
        use self::Register::*;

        match *self {
            LCDC => 0xFF40,
            STAT => 0xFF41,
            SCY => 0xFF42,
            SCX => 0xFF43,
            LY => 0xFF44,
            LYC => 0xFF45,
            DMA => 0xFF46,
            BGP => 0xFF47,
            OBP0 => 0xFF48,
            OBP1 => 0xFF49,
            WY => 0xFF4A,
            WX => 0xFF4B,
        }
    }
}
