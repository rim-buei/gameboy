mod mbc1;

use self::mbc1::Mbc1;

pub struct Cartridge {
    mbc: Mbc1,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        Cartridge { mbc: Mbc1::new(data) }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mbc.read(addr)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mbc.write(addr, data);
    }
}
