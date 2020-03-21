use super::MemoryBankController;

enum MemoryModel {
    Model0,
    Model1,
}

pub struct Mbc1 {
    rom: Vec<u8>,
    rom_bank: usize,
    ram: Vec<u8>,
    ram_bank: usize,

    memory_model: MemoryModel,
    ram_enabled: bool,
}

impl Mbc1 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc1 {
            rom: data,
            rom_bank: 1,
            ram: vec![0x00; 0x8000],
            ram_bank: 0,

            memory_model: MemoryModel::Model0,
            ram_enabled: false,
        }
    }
}

impl MemoryBankController for Mbc1 {
    fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3FFF => self.rom[addr],
            0x4000..=0x7FFF => self.rom[(addr - 0x4000) + (self.rom_bank * 0x4000)],
            0xA000..=0xBFFF => {
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
        match addr {
            0x0000..=0x1FFF => match data & 0x0F {
                0x00 => self.ram_enabled = false,
                0x0A => self.ram_enabled = true,
                _ => (),
            },
            0x2000..=0x3FFF => {
                self.rom_bank = (data & 0x1F) as usize | (self.rom_bank & 0xE0);
                self.rom_bank = increment_rom_bank(self.rom_bank);
            }
            0x4000..=0x5FFF => match self.memory_model {
                MemoryModel::Model0 => {
                    self.rom_bank = ((data & 0x03) << 5) as usize | (self.rom_bank & 0x1F);
                    self.rom_bank = increment_rom_bank(self.rom_bank);
                }
                MemoryModel::Model1 => self.ram_bank = (data & 0x03) as usize,
            },
            0x6000..=0x7FFF => match data & 0x01 {
                0x00 => self.memory_model = MemoryModel::Model0,
                0x01 => self.memory_model = MemoryModel::Model1,
                _ => unreachable!(),
            },
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }

                self.ram[(addr - 0xA000) + (self.ram_bank * 0x2000)] = data;
            }
            _ => panic!("inaccessible address"),
        };
    }
}

fn increment_rom_bank(rom_bank: usize) -> usize {
    match rom_bank {
        0x00 | 0x20 | 0x40 | 0x60 => rom_bank + 1,
        _ => rom_bank,
    }
}
