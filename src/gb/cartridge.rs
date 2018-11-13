pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        Cartridge { data }
    }
}
