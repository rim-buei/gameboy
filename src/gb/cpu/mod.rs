pub mod register;

mod instruction;
mod io;
mod processor;

use super::bus::Bus;

use self::instruction::{exec, exec_prefix_cb};
use self::register::Registers;

pub fn step<B: Bus>(reg: &mut Registers, bus: &mut B) -> u8 {
    let addr = reg.PC;
    let opcode = bus.read8(addr);

    let (bytes, cycles) = if opcode != 0xCB {
        // 1-byte instruction
        exec(opcode, reg, bus)
    } else {
        // 2-byte instruction
        let addr = reg.PC.wrapping_add(1);
        let opcode = bus.read8(addr);

        exec_prefix_cb(opcode, reg, bus)
    };

    reg.PC = reg.PC.wrapping_add(bytes as u16);
    cycles
}

pub fn reset(reg: &mut Registers) {
    *reg = Registers::new()
}
