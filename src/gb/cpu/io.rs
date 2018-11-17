use super::super::bus::Bus;

use super::register::Registers;

pub trait Reader8 {
    fn read8<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u8;
}

pub trait Writer8 {
    fn write8<B: Bus>(&self, reg: &mut Registers, bus: &mut B, v: u8);
}

pub trait Reader16 {
    fn read16<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u16;
}

pub trait Writer16 {
    fn write16<B: Bus>(&self, reg: &mut Registers, bus: &mut B, v: u16);
}
