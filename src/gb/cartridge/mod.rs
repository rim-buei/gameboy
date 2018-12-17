mod mbc1;

use self::mbc1::Mbc1;

const CARTRIDGE_TYPE_ADDR: u16 = 0x0147;

pub struct Cartridge {
    mbc: Box<MemoryBankController>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        if data.len() < CARTRIDGE_TYPE_ADDR as usize {
            // TODO: Should be treated as ROM probably
            panic!("broken cartridge");
        }

        match data[CARTRIDGE_TYPE_ADDR as usize] {
            0x01 | 0x02 | 0x03 => Cartridge {
                mbc: Box::new(Mbc1::new(data)),
            },
            _ => {
                // TODO: Add more MBC supports
                panic!("unsupported cartridge type");
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mbc.read(addr)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mbc.write(addr, data);
    }
}

trait MemoryBankController {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}
