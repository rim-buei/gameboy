use super::MemoryBankController;

pub struct NoMbc {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMbc {
    pub fn new(data: Vec<u8>) -> Self {
        NoMbc {
            rom: data,
            ram: vec![0x00; 0x2000],
        }
    }
}

impl MemoryBankController for NoMbc {
    fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000...0x7FFF => self.rom[addr],
            0xA000...0xBFFF => self.ram[addr - 0xA000],
            _ => panic!("inaccessible address"),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        match addr {
            0xA000...0xBFFF => self.ram[addr - 0xA000] = data,
            _ => panic!("inaccessible address"),
        };
    }
}
