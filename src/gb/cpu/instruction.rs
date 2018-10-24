use super::super::ram::Ram;

use super::register::Registers;

pub fn exec(opcode: u8, reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    (0, 0)
}

pub fn exec_ex(opcode: u8, reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    (0, 0)
}
