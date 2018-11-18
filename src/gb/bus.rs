pub trait Bus {
    fn read8(&self, addr: u16) -> u8;
    fn read16(&self, addr: u16) -> u16;
    fn write8(&mut self, addr: u16, data: u8);
    fn write16(&mut self, addr: u16, data: u16);
}
