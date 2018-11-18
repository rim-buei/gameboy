use super::bus::Bus;

pub struct Ram {
    array: Vec<u8>,
}

impl Ram {
    pub fn new(array: Vec<u8>) -> Self {
        Ram { array }
    }

    pub fn dump(&self) -> Vec<u8> {
        self.array.clone()
    }
}

impl Bus for Ram {
    fn read8(&self, addr: u16) -> u8 {
        self.array[addr as usize]
    }

    fn read16(&self, addr: u16) -> u16 {
        self.read8(addr) as u16 | ((self.read8(addr.wrapping_add(1)) as u16) << 8)
    }

    fn write8(&mut self, addr: u16, data: u8) {
        self.array[addr as usize] = data;
    }

    fn write16(&mut self, addr: u16, data: u16) {
        self.write8(addr, (data & 0xFF) as u8);
        self.write8(addr.wrapping_add(1), (data >> 8) as u8);
    }
}
