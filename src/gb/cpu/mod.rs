mod instruction;
mod io;
mod processor;
mod register;

use std::fmt;

use super::bus::Bus;

use self::instruction::{exec, exec_prefix_cb};
use self::register::Registers;

pub struct Cpu {
    reg: Registers,

    // Interrupt Master Enable Flag
    ime: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: register::Registers::new(),
            ime: false,
        }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = self.reg.PC;
        let opcode = bus.read8(addr);

        let (bytes, cycles) = if opcode != 0xCB {
            // 1-byte instruction
            exec(opcode, &mut self.reg, bus)
        } else {
            // 2-byte instruction
            let addr = self.reg.PC.wrapping_add(1);
            let opcode = bus.read8(addr);

            exec_prefix_cb(opcode, &mut self.reg, bus)
        };

        self.reg.PC = self.reg.PC.wrapping_add(bytes as u16);
        cycles
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pc = self.reg.PC;
        write!(f, "PC: 0x{:04X}", pc)
    }
}
