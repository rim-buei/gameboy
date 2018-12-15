use super::super::bus::Bus;
use super::io::{Reader16, Reader8, Writer16, Writer8};
use super::state::{Flag, State};

#[derive(Debug, Copy, Clone)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Reader8 for Register8 {
    fn read8<B: Bus>(&self, state: &mut State, _: &mut B) -> u8 {
        use self::Register8::*;

        match *self {
            A => state.A,
            F => state.F,
            B => state.B,
            C => state.C,
            D => state.D,
            E => state.E,
            H => state.H,
            L => state.L,
        }
    }
}

impl Writer8 for Register8 {
    fn write8<B: Bus>(&self, state: &mut State, _: &mut B, v: u8) {
        use self::Register8::*;

        match *self {
            A => state.A = v,
            F => state.F = v,
            B => state.B = v,
            C => state.C = v,
            D => state.D = v,
            E => state.E = v,
            H => state.H = v,
            L => state.L = v,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

impl Reader16 for Register16 {
    fn read16<B: Bus>(&self, state: &mut State, _: &mut B) -> u16 {
        use self::Register16::*;

        match *self {
            AF => ((state.A as u16) << 8) + (state.F as u16),
            BC => ((state.B as u16) << 8) + (state.C as u16),
            DE => ((state.D as u16) << 8) + (state.E as u16),
            HL => ((state.H as u16) << 8) + (state.L as u16),
            SP => state.SP,
            PC => state.PC,
        }
    }
}

impl Writer16 for Register16 {
    fn write16<B: Bus>(&self, state: &mut State, _: &mut B, v: u16) {
        use self::Register16::*;

        match *self {
            AF => {
                state.A = (v >> 8) as u8;
                state.F = (v & 0xF0) as u8;
            }
            BC => {
                state.B = (v >> 8) as u8;
                state.C = (v & 0xFF) as u8;
            }
            DE => {
                state.D = (v >> 8) as u8;
                state.E = (v & 0xFF) as u8;
            }
            HL => {
                state.H = (v >> 8) as u8;
                state.L = (v & 0xFF) as u8;
            }
            PC => state.PC = v,
            SP => state.SP = v,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Address {
    BC,
    DE,
    HL,

    Direct, // Read from 16-bit immediate value
    FF00,   // Read from $FF00 + 8-bit immediate value
    FF00C,  // Read from $FF00 + C register
}

impl Address {
    fn get<B: Bus>(&self, state: &mut State, bus: &mut B) -> u16 {
        use self::Address::*;

        match *self {
            BC => Register16::BC.read16(state, bus),
            DE => Register16::DE.read16(state, bus),
            HL => Register16::HL.read16(state, bus),

            Direct => Immediate16.read16(state, bus),
            FF00 => 0xFF00 + Immediate8.read8(state, bus) as u16,
            FF00C => 0xFF00 + state.C as u16,
        }
    }
}

impl Reader8 for Address {
    fn read8<B: Bus>(&self, state: &mut State, bus: &mut B) -> u8 {
        let addr = self.get(state, bus);
        bus.read8(addr)
    }
}

impl Reader16 for Address {
    fn read16<B: Bus>(&self, state: &mut State, bus: &mut B) -> u16 {
        let addr = self.get(state, bus);
        bus.read16(addr)
    }
}

impl Writer8 for Address {
    fn write8<B: Bus>(&self, state: &mut State, bus: &mut B, v: u8) {
        let addr = self.get(state, bus);
        bus.write8(addr, v);
    }
}

impl Writer16 for Address {
    fn write16<B: Bus>(&self, state: &mut State, bus: &mut B, v: u16) {
        let addr = self.get(state, bus);
        bus.write16(addr, v);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Immediate8;

impl Reader8 for Immediate8 {
    fn read8<B: Bus>(&self, state: &mut State, bus: &mut B) -> u8 {
        bus.read8(state.PC.wrapping_add(1))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Immediate16;

impl Reader16 for Immediate16 {
    fn read16<B: Bus>(&self, state: &mut State, bus: &mut B) -> u16 {
        bus.read16(state.PC.wrapping_add(1))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Data16(pub u16);

impl Reader16 for Data16 {
    fn read16<B: Bus>(&self, _: &mut State, _: &mut B) -> u16 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    NZ, // Zero flag is disabled
    Z,  // Zero flag is enabled
    NC, // Carry flag is disabled
    C,  // Carry flag is enabled

    T, // True
    F, // False
}

impl Condition {
    pub fn test(&self, state: &mut State) -> bool {
        use self::Condition::*;

        match *self {
            NZ => !state.get_flag(Flag::Z),
            Z => state.get_flag(Flag::Z),
            NC => !state.get_flag(Flag::C),
            C => state.get_flag(Flag::C),

            T => true,
            F => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::ram::Ram;

    use super::{Register16 as R16, *};

    #[test]
    fn test_register16() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00]);

        R16::AF.write16(&mut state, &mut ram, 0xFFFF);
        R16::BC.write16(&mut state, &mut ram, 0xFFFF);
        assert_eq!(0xFFF0, R16::AF.read16(&mut state, &mut ram));
        assert_eq!(0xFFFF, R16::BC.read16(&mut state, &mut ram));
    }

    #[test]
    fn test_address_get() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        state.C = 0xBB;

        assert_eq!(0xFFAA, Address::FF00.get(&mut state, &mut ram));
        assert_eq!(0xFFBB, Address::FF00C.get(&mut state, &mut ram));
    }
}
