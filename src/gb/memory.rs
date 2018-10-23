pub struct Memory {
    blob: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            blob: vec![0; 0xffff],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.blob[addr as usize] = data
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.blob[addr as usize]
    }
}
