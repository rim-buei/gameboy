mod instruction;
mod io;
mod oprand;
mod processor;
mod state;

use self::instruction::{exec, exec_prefix_cb, interrupt};
use self::state::State;
use super::bus::Bus;
use super::interrupt::{self, Interrupt};
use std::fmt;

pub struct Cpu {
    state: State,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { state: State::new() }
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> u8 {
        self.process_halt(bus);
        self.process_interrupt(bus) + self.process_instruction(bus)
    }

    fn process_instruction<B: Bus>(&mut self, bus: &mut B) -> u8 {
        if self.state.halted {
            return 4;
        }

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

    fn process_halt<B: Bus>(&mut self, bus: &mut B) {
        if !self.state.halted {
            return;
        }

        if self.state.interrupts_before_halt != interrupt::dump_raw_flags(bus) {
            self.state.halted = false;
        }
    }

    fn process_interrupt<B: Bus>(&mut self, bus: &mut B) -> u8 {
        if !self.state.IME {
            return 0;
        }
        self.state.IME = false;

        let pc = match interrupt::receive(bus) {
            Interrupt::VBlank => 0x40,
            Interrupt::LCDStat => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,

            Interrupt::None => return 0,
        };
        interrupt(pc, &mut self.state, bus)
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pc = self.state.PC;
        let af = ((self.state.A as u16) << 8) + (self.state.F as u16);
        let bc = ((self.state.B as u16) << 8) + (self.state.C as u16);
        let de = ((self.state.D as u16) << 8) + (self.state.E as u16);
        let hl = ((self.state.H as u16) << 8) + (self.state.L as u16);
        write!(
            f,
            "[CPU] PC: 0x{:04X}, AF 0x{:04X}, BC 0x{:04X}, DE 0x{:04X}, HL 0x{:04X}",
            pc, af, bc, de, hl,
        )
    }
}
