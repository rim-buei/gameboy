pub struct Memory {
    blob: Vec<u8>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            blob: vec![0; 0xffff],
        }
    }
}
