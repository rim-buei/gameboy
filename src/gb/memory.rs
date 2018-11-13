use super::cartridge::Cartridge;
use super::ram::Ram;

pub struct Memory {
    array: Vec<u8>,
}

impl Memory {
    pub fn new(_: Cartridge) -> Self {
        Memory {
            array: vec![64 * (2 ^ 10); 0x00],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.array[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, data: u8) {
        self.array[addr as usize] = data
    }

    pub fn dump(&self) -> Vec<u8> {
        self.array.clone()
    }
}
