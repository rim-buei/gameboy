pub struct Rom {
    blob: Vec<u8>,
}

impl Rom {
    pub fn new(blob: Vec<u8>) -> Self {
        Rom { blob }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.blob[addr as usize]
    }
}
