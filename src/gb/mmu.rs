use super::bus::Bus;
use super::cartridge::Cartridge;
use super::ram::Ram;

pub struct Mmu {
    state: State,
    cart: Cartridge,
    memory: Ram,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            state: State::new(),
            cart: Cartridge::new(vec![0x00; 1 << 15]),
            memory: Ram::new(vec![0x00; 1 << 16]),
        }
    }

    pub fn load(&mut self, offset: u16, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.write8(offset.wrapping_add(i as u16), *byte);
        }
    }

    pub fn load_cartridge(&mut self, cart: Cartridge) {
        self.cart = cart;
    }

    pub fn simulate_bootloader(&mut self) {
        self.memory = Ram::new(vec![0x00; 1 << 16]);
        self.memory.write8(0xFF05, 0x00);
        self.memory.write8(0xFF06, 0x00);
        self.memory.write8(0xFF07, 0x00);
        self.memory.write8(0xFF10, 0x80);
        self.memory.write8(0xFF11, 0xBF);
        self.memory.write8(0xFF12, 0xF3);
        self.memory.write8(0xFF14, 0xBF);
        self.memory.write8(0xFF16, 0x3F);
        self.memory.write8(0xFF17, 0x00);
        self.memory.write8(0xFF19, 0xBF);
        self.memory.write8(0xFF1A, 0x7F);
        self.memory.write8(0xFF1B, 0xFF);
        self.memory.write8(0xFF1C, 0x9F);
        self.memory.write8(0xFF1E, 0xBF);
        self.memory.write8(0xFF20, 0xFF);
        self.memory.write8(0xFF21, 0x00);
        self.memory.write8(0xFF22, 0x00);
        self.memory.write8(0xFF23, 0xBF);
        self.memory.write8(0xFF24, 0x77);
        self.memory.write8(0xFF25, 0xF3);
        self.memory.write8(0xFF26, 0xF1);
        self.memory.write8(0xFF40, 0x91);
        self.memory.write8(0xFF42, 0x00);
        self.memory.write8(0xFF43, 0x00);
        self.memory.write8(0xFF45, 0x00);
        self.memory.write8(0xFF47, 0xFC);
        self.memory.write8(0xFF48, 0xFF);
        self.memory.write8(0xFF49, 0xFF);
        self.memory.write8(0xFF4A, 0x00);
        self.memory.write8(0xFF4B, 0x00);
        self.memory.write8(0xFFFF, 0x00);

        self.memory.write8(0xFF50, 0x01);
    }

    pub fn is_joypad_state_requested(&self) -> bool {
        self.state.joypad_requested
    }

    pub fn receive_joypad_state(&mut self, state: (u8, u8)) {
        let value = self.read8(0xFF00);

        let input = if value & 0x10 == 0x00 {
            state.0
        } else if value & 0x20 == 0x00 {
            state.1
        } else {
            0x0F
        };

        self.write8(0xFF00, value | 0b1100_0000 | input);
        self.state.joypad_requested = false;
    }

    fn dma_transfer(&mut self, value: u8) {
        let start_addr = (value as u16) * 0x100;
        for i in 0..0xA0 {
            self.memory.write8(0xFE00 + i, self.memory.read8(start_addr + i));
        }
    }
}

impl Bus for Mmu {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cart.read(addr),
            0xA000..=0xBFFF => self.cart.read(addr),

            // Mirror of 0xC000..=0xDDFF (Typically not used)
            0xE000..=0xFDFF => self.memory.read8(addr - 0x2000),

            _ => self.memory.read8(addr),
        }
    }

    fn read16(&self, addr: u16) -> u16 {
        self.read8(addr) as u16 | (self.read8(addr.wrapping_add(1)) as u16) << 8
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => self.cart.write(addr, data),
            0xA000..=0xBFFF => self.cart.write(addr, data),

            // Mirror of 0xC000..=0xDDFF (Typically not used)
            0xE000..=0xFDFF => self.memory.write8(addr - 0x2000, data),

            // Joypad register
            0xFF00 => {
                if (data & 0x10) == 0x00 || (data & 0x20) == 0x00 {
                    self.state.joypad_requested = true
                }
                self.memory.write8(addr, data);
            }

            // Divider register
            0xFF04 => {
                // TODO: Should we reset divider's counter as well...?
                self.memory.write8(addr, 0);
            }
            // DMA transfer
            0xFF46 => self.dma_transfer(data),

            _ => self.memory.write8(addr, data),
        };
    }

    fn write16(&mut self, addr: u16, data: u16) {
        self.write8(addr, (data & 0xFF) as u8);
        self.write8(addr.wrapping_add(1), (data >> 8) as u8);
    }
}

struct State {
    joypad_requested: bool,
}

impl State {
    fn new() -> Self {
        State {
            joypad_requested: false,
        }
    }
}
