mod mbc0;
mod mbc1;
mod mbc5;

use self::mbc0::Mbc0;
use self::mbc1::Mbc1;
use self::mbc5::Mbc5;

const CARTRIDGE_TYPE_ADDR: u16 = 0x0147;

pub struct Cartridge {
    mbc: Box<dyn MemoryBankController>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        if data.len() < CARTRIDGE_TYPE_ADDR as usize {
            // TODO: Should be treated as ROM probably
            panic!("broken cartridge");
        }

        match data[CARTRIDGE_TYPE_ADDR as usize] {
            0x00 | 0x08 | 0x09 => Cartridge {
                mbc: Box::new(Mbc0::new(data)),
            },
            0x01 | 0x02 | 0x03 => Cartridge {
                mbc: Box::new(Mbc1::new(data)),
            },
            0x05 | 0x06 => {
                panic!("unsupported cartridge type: MBC2");
            }
            0x0F | 0x10 | 0x11 | 0x12 | 0x13 => {
                panic!("unsupported cartridge type: MBC3");
            }
            0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => Cartridge {
                mbc: Box::new(Mbc5::new(data)),
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
