use super::super::ram::Ram;

use super::register::Registers;

pub trait Reader8 {
    fn read8(&self, reg: &mut Registers, ram: &mut Ram) -> u8;
}

pub trait Writer8 {
    fn write8(&self, reg: &mut Registers, ram: &mut Ram, v: u8);
}

pub trait Reader16 {
    fn read16(&self, reg: &mut Registers, ram: &mut Ram) -> u16;
}

pub trait Writer16 {
    fn write16(&self, reg: &mut Registers, ram: &mut Ram, v: u16);
}
