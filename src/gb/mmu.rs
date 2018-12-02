use super::bus::Bus;
use super::ram::Ram;

pub struct Mmu {
    array: Ram,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            array: Ram::new(vec![0x00; 1 << 16]),
        }
    }

    pub fn load(&mut self, offset: u16, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.write8(offset.wrapping_add(i as u16), *byte);
        }
    }

    pub fn dump(&self) -> Vec<u8> {
        self.array.dump()
    }

    pub fn simulate_bootloader(&mut self) {
        self.array = Ram::new(vec![0x00; 1 << 16]);
        self.array.write8(0xFF05, 0x00);
        self.array.write8(0xFF06, 0x00);
        self.array.write8(0xFF07, 0x00);
        self.array.write8(0xFF10, 0x80);
        self.array.write8(0xFF11, 0xBF);
        self.array.write8(0xFF12, 0xF3);
        self.array.write8(0xFF14, 0xBF);
        self.array.write8(0xFF16, 0x3F);
        self.array.write8(0xFF17, 0x00);
        self.array.write8(0xFF19, 0xBF);
        self.array.write8(0xFF1A, 0x7F);
        self.array.write8(0xFF1B, 0xFF);
        self.array.write8(0xFF1C, 0x9F);
        self.array.write8(0xFF1E, 0xBF);
        self.array.write8(0xFF20, 0xFF);
        self.array.write8(0xFF21, 0x00);
        self.array.write8(0xFF22, 0x00);
        self.array.write8(0xFF23, 0xBF);
        self.array.write8(0xFF24, 0x77);
        self.array.write8(0xFF25, 0xF3);
        self.array.write8(0xFF26, 0xF1);
        self.array.write8(0xFF40, 0x91);
        self.array.write8(0xFF42, 0x00);
        self.array.write8(0xFF43, 0x00);
        self.array.write8(0xFF45, 0x00);
        self.array.write8(0xFF47, 0xFC);
        self.array.write8(0xFF48, 0xFF);
        self.array.write8(0xFF49, 0xFF);
        self.array.write8(0xFF4A, 0x00);
        self.array.write8(0xFF4B, 0x00);
        self.array.write8(0xFFFF, 0x00);

        self.array.write8(0xFF50, 0x01);
    }
}

impl Bus for Mmu {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            // Mirror of 0xC000...0xDDFF (Typically not used)
            0xE000...0xFDFF => self.array.read8(addr - 0x2000),

            _ => self.array.read8(addr),
        }
    }

    fn read16(&self, addr: u16) -> u16 {
        self.array.read16(addr)
    }

    fn write8(&mut self, addr: u16, data: u8) {
        self.array.write8(addr, data);
    }

    fn write16(&mut self, addr: u16, data: u16) {
        self.array.write16(addr, data);
    }
}
