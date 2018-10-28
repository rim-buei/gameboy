pub mod register;

mod instruction;

use super::ram::Ram;

use self::instruction::exec;
use self::instruction::exec_ex;
use self::register::Register16;
use self::register::Register8;
use self::register::Registers;

pub fn step(reg: &mut Registers, ram: &mut Ram) -> u8 {
    let addr = reg.get_PC();
    let opcode = ram.read(addr);

    let (bytes, cycles) = if opcode != 0xCB {
        // 1-byte instruction
        exec(opcode, reg, ram)
    } else {
        // 2-byte instruction
        let addr = reg.inc_PC().get_PC();
        let opcode = ram.read(addr);

        exec_ex(opcode, reg, ram)
    };

    reg.add16(Register16::PC, bytes as u16);
    cycles
}

pub fn reset(reg: &mut Registers) {
    *reg = Registers::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        let mut reg = Registers::new();
        reg.set8(Register8::A, 1);
        reset(&mut reg);
        assert_eq!(0, reg.get8(Register8::A));
    }
}
