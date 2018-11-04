pub mod register;

mod instruction;
mod io;

use super::ram::Ram;

use self::instruction::exec;
use self::instruction::exec_ex;
use self::register::Register16;
use self::register::Register8;
use self::register::Registers;

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

        exec_ex(opcode, reg, ram)
    };

    reg.PC = reg.PC.wrapping_add(bytes as u16);
    cycles
}

pub fn reset(reg: &mut Registers) {
    *reg = Registers::new()
}
