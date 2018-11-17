use super::super::bus::Bus;

use super::io::{Reader16, Reader8, Writer16, Writer8};

#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub A: u8,
    pub F: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,

    pub SP: u16,
    pub PC: u16,
}

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
    fn read8<B: Bus>(&self, reg: &mut Registers, _: &mut B) -> u8 {
        use self::Register8::*;

        match *self {
            A => reg.A,
            F => reg.F,
            B => reg.B,
            C => reg.C,
            D => reg.D,
            E => reg.E,
            H => reg.H,
            L => reg.L,
        }
    }
}

impl Writer8 for Register8 {
    fn write8<B: Bus>(&self, reg: &mut Registers, _: &mut B, v: u8) {
        use self::Register8::*;

        match *self {
            A => reg.A = v,
            F => reg.F = v,
            B => reg.B = v,
            C => reg.C = v,
            D => reg.D = v,
            E => reg.E = v,
            H => reg.H = v,
            L => reg.L = v,
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
    fn read16<B: Bus>(&self, reg: &mut Registers, _: &mut B) -> u16 {
        use self::Register16::*;

        match *self {
            Register16::AF => ((reg.A as u16) << 8) + (reg.F as u16),
            Register16::BC => ((reg.B as u16) << 8) + (reg.C as u16),
            Register16::DE => ((reg.D as u16) << 8) + (reg.E as u16),
            Register16::HL => ((reg.H as u16) << 8) + (reg.L as u16),
            Register16::SP => reg.SP,
            Register16::PC => reg.PC,
        }
    }
}

impl Writer16 for Register16 {
    fn write16<B: Bus>(&self, reg: &mut Registers, _: &mut B, v: u16) {
        use self::Register16::*;

        match *self {
            Register16::AF => {
                reg.A = (v >> 8) as u8;
                reg.F = (v & 0xFF) as u8;
            }
            Register16::BC => {
                reg.B = (v >> 8) as u8;
                reg.C = (v & 0xFF) as u8;
            }
            Register16::DE => {
                reg.D = (v >> 8) as u8;
                reg.E = (v & 0xFF) as u8;
            }
            Register16::HL => {
                reg.H = (v >> 8) as u8;
                reg.L = (v & 0xFF) as u8;
            }
            Register16::PC => reg.PC = v,
            Register16::SP => reg.SP = v,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Z, // Zero
    N, // Subtract
    H, // Half Carry
    C, // Carry
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0x00,
            F: 0x00,
            B: 0x00,
            C: 0x00,
            D: 0x00,
            E: 0x00,
            H: 0x00,
            L: 0x00,

            PC: 0x0000,
            SP: 0x0000,
        }
    }

    pub fn enable_flag(&mut self, flag: Flag) -> &mut Self {
        match flag {
            Flag::Z => self.F |= 1 << 7,
            Flag::N => self.F |= 1 << 6,
            Flag::H => self.F |= 1 << 5,
            Flag::C => self.F |= 1 << 4,
        }
        self
    }

    pub fn disable_flag(&mut self, flag: Flag) -> &mut Self {
        match flag {
            Flag::Z => self.F &= !(1 << 7),
            Flag::N => self.F &= !(1 << 6),
            Flag::H => self.F &= !(1 << 5),
            Flag::C => self.F &= !(1 << 4),
        }
        self
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => (self.F & (1 << 7)) != 0,
            Flag::N => (self.F & (1 << 6)) != 0,
            Flag::H => (self.F & (1 << 5)) != 0,
            Flag::C => (self.F & (1 << 4)) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, v: bool) -> &mut Self {
        if v == true {
            self.enable_flag(flag)
        } else {
            self.disable_flag(flag)
        }
    }
}

// TODO: Should be moved to other module
#[derive(Debug, Copy, Clone)]
pub enum Address {
    BC,
    DE,
    HL,

    HLI, // HL+
    HLD, // HL-

    Direct, // Read from 16-bit immediate value
    FF00,   // Read from $FF00 + 8-bit immediate value
    FF00C,  // Read from $FF00 + C register
}

impl Address {
    fn get<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u16 {
        use self::Address::*;

        match *self {
            BC => Register16::BC.read16(reg, bus),
            DE => Register16::DE.read16(reg, bus),
            HL => Register16::HL.read16(reg, bus),

            HLI => Register16::HL.read16(reg, bus).wrapping_add(1),
            HLD => Register16::HL.read16(reg, bus).wrapping_sub(1),

            Direct => Immediate16.read16(reg, bus),
            FF00 => 0xFF00 + Immediate8.read8(reg, bus) as u16,
            FF00C => 0xFF00 + reg.C as u16,
        }
    }
}

impl Reader8 for Address {
    fn read8<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u8 {
        let addr = self.get(reg, bus);
        bus.read8(addr)
    }
}

impl Reader16 for Address {
    fn read16<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u16 {
        let addr = self.get(reg, bus);
        bus.read16(addr)
    }
}

impl Writer8 for Address {
    fn write8<B: Bus>(&self, reg: &mut Registers, bus: &mut B, v: u8) {
        let addr = self.get(reg, bus);
        bus.write8(addr, v);
    }
}

impl Writer16 for Address {
    fn write16<B: Bus>(&self, reg: &mut Registers, bus: &mut B, v: u16) {
        let addr = self.get(reg, bus);
        bus.write16(addr, v);
    }
}

// TODO: Should be moved to other module
#[derive(Debug, Copy, Clone)]
pub struct Immediate8;

impl Reader8 for Immediate8 {
    fn read8<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u8 {
        bus.read8(reg.PC.wrapping_add(1))
    }
}

// TODO: Should be moved to other module
#[derive(Debug, Copy, Clone)]
pub struct Immediate16;

impl Reader16 for Immediate16 {
    fn read16<B: Bus>(&self, reg: &mut Registers, bus: &mut B) -> u16 {
        bus.read16(reg.PC.wrapping_add(1))
    }
}

// TODO: Should be moved to other module
#[derive(Debug, Copy, Clone)]
pub struct Data16(pub u16);

impl Reader16 for Data16 {
    fn read16<B: Bus>(&self, _: &mut Registers, _: &mut B) -> u16 {
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
    pub fn test(&self, reg: &mut Registers) -> bool {
        use self::Condition::*;

        match *self {
            NZ => !reg.get_flag(Flag::Z),
            Z => reg.get_flag(Flag::Z),
            NC => !reg.get_flag(Flag::C),
            C => reg.get_flag(Flag::C),
            T => true,
            F => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::ram::Ram;

    use super::*;

    #[test]
    fn test_registers_flag() {
        let mut reg = Registers::new();

        reg.enable_flag(Flag::Z);
        assert_eq!(0b10000000, reg.F);
        reg.enable_flag(Flag::N);
        assert_eq!(0b11000000, reg.F);
        reg.disable_flag(Flag::Z);
        assert_eq!(0b01000000, reg.F);
    }

    #[test]
    fn test_address_get() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        reg.C = 0xBB;

        assert_eq!(0xFFAA, Address::FF00.get(&mut reg, &mut ram));
        assert_eq!(0xFFBB, Address::FF00C.get(&mut reg, &mut ram))
    }
}
