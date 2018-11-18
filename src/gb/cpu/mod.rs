mod instruction;
mod io;
mod oprand;
mod processor;
mod state;

use self::instruction::{exec, exec_prefix_cb};
use self::state::State;
use super::bus::Bus;
use std::fmt;

pub struct Cpu {
    state: State,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { state: State::new() }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let addr = self.state.PC;
        let opcode = bus.read8(addr);

        let (bytes, cycles) = if opcode != 0xCB {
            // 1-byte instruction
            exec(opcode, &mut self.state, bus)
        } else {
            // 2-byte instruction
            let addr = self.state.PC.wrapping_add(1);
            let opcode = bus.read8(addr);

            exec_prefix_cb(opcode, &mut self.state, bus)
        };

        self.state.PC = self.state.PC.wrapping_add(bytes as u16);
        cycles
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pc = self.state.PC;
        let hl = ((self.state.H as u16) << 8) + (self.state.L as u16);
        write!(f, "PC: 0x{:04X}, HL 0x{:04X}", pc, hl)
    }
}
