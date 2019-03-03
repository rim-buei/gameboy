use super::MemoryBankController;

pub struct Mbc5 {
    rom: Vec<u8>,
    rom_bank: usize,
    ram: Vec<u8>,
    ram_bank: usize,

    ram_enabled: bool,
}

impl Mbc5 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc5 {
            rom: data,
            rom_bank: 1,
            ram: vec![0x00; 0x20000],
            ram_bank: 0,

            ram_enabled: false,
        }
    }
}

impl MemoryBankController for Mbc5 {
    fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000...0x3FFF => self.rom[addr],
            0x4000...0x7FFF => self.rom[(addr - 0x4000) + (self.rom_bank * 0x4000)],
            0xA000...0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }

                self.ram[(addr - 0xA000) + (self.ram_bank * 0x2000)]
            }
            _ => panic!("inaccessible address"),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        let data = data as usize;
        match addr {
            0x0000...0x1FFF => match data & 0x0F {
                0x00 => self.ram_enabled = false,
                0x0A => self.ram_enabled = true,
                _ => (),
            },
            0x2000...0x2FFF => self.rom_bank = (self.rom_bank & 0x100) | data,
            0x3000...0x3FFF => self.rom_bank = (self.rom_bank & 0x0FF) | ((data & 0x01) << 8),
            0x4000...0x5FFF => self.ram_bank = (data & 0x0F) as usize,
            _ => panic!("inaccessible address"),
        };
    }
}
