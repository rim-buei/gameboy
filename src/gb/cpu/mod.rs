pub mod register;

mod instruction;
mod io;
mod processor;

use super::ram::Ram;

use self::instruction::{exec, exec_prefix_cb};
use self::register::{Register16, Register8, Registers};

pub fn step(reg: &mut Registers, ram: &mut Ram) -> u8 {
    let addr = reg.PC;
    let opcode = ram.read(addr);

    let (bytes, cycles) = if opcode != 0xCB {
        // 1-byte instruction
        exec(opcode, reg, ram)
    } else {
        // 2-byte instruction
        let addr = reg.PC.wrapping_add(1);
        let opcode = ram.read(addr);

        exec_prefix_cb(opcode, reg, ram)
    };

    reg.PC = reg.PC.wrapping_add(bytes as u16);
    cycles
}

pub fn reset(reg: &mut Registers) {
    *reg = Registers::new()
}
