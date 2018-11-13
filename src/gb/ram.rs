// TODO: This should be merged into Memory
pub struct Ram {
    blob: Vec<u8>,
}

impl Ram {
    pub fn new(blob: Vec<u8>) -> Self {
        Ram { blob }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.blob[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.blob[addr as usize] = data
    }

    pub fn dump(&self) -> Vec<u8> {
        self.blob.clone()
    }
}
