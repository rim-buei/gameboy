use super::MemoryBankController;

pub struct Mbc0 {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Mbc0 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc0 {
            rom: data,
            ram: vec![0x00; 0x2000],
        }
    }
}

impl MemoryBankController for Mbc0 {
    fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x7FFF => self.rom[addr],
            0xA000..=0xBFFF => self.ram[addr - 0xA000],
            _ => panic!("inaccessible address"),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        match addr {
            0xA000..=0xBFFF => self.ram[addr - 0xA000] = data,
            _ => { /* TODO: Consider if this case should be error */ }
        };
    }
}
