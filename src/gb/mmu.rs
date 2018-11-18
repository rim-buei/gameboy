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

    pub fn dump(&self) -> Vec<u8> {
        self.array.dump()
    }
}

impl Bus for Mmu {
    fn read8(&self, addr: u16) -> u8 {
        self.array.read8(addr)
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
