#[allow(non_snake_case)]
pub struct Registers {
    A: u8,
    F: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,

    SP: u16,
    PC: u16,
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

#[derive(Debug, Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

pub enum Flag {
    Z,
    N,
    H,
    C,
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

    pub fn get8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.A,
            Register8::F => self.F,
            Register8::B => self.B,
            Register8::C => self.C,
            Register8::D => self.D,
            Register8::E => self.E,
            Register8::H => self.H,
            Register8::L => self.L,
        }
    }

    pub fn set8(&mut self, reg: Register8, v: u8) -> &mut Self {
        match reg {
            Register8::A => self.A = v,
            Register8::F => self.F = v,
            Register8::B => self.B = v,
            Register8::C => self.C = v,
            Register8::D => self.D = v,
            Register8::E => self.E = v,
            Register8::H => self.H = v,
            Register8::L => self.L = v,
        }
        self
    }

    pub fn get16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => ((self.A as u16) << 8) + (self.F as u16),
            Register16::BC => ((self.B as u16) << 8) + (self.C as u16),
            Register16::DE => ((self.D as u16) << 8) + (self.E as u16),
            Register16::HL => ((self.H as u16) << 8) + (self.L as u16),
            Register16::SP => self.SP,
            Register16::PC => self.PC,
        }
    }

    pub fn set16(&mut self, reg: Register16, v: u16) -> &mut Self {
        match reg {
            Register16::AF => {
                self.A = (v >> 8) as u8;
                self.F = (v & 0xFF) as u8;
            }
            Register16::BC => {
                self.B = (v >> 8) as u8;
                self.C = (v & 0xFF) as u8;
            }
            Register16::DE => {
                self.D = (v >> 8) as u8;
                self.E = (v & 0xFF) as u8;
            }
            Register16::HL => {
                self.H = (v >> 8) as u8;
                self.L = (v & 0xFF) as u8;
            }
            Register16::PC => self.PC = v,
            Register16::SP => self.SP = v,
        }
        self
    }

    pub fn add8(&mut self, reg: Register8, n: u8) -> &mut Self {
        let v = self.get8(reg) as u32 + n as u32;
        if v > 0xFF {
            // TODO: Overflow
        }

        self.set8(reg, (v & 0xFF) as u8)
    }

    pub fn inc8(&mut self, reg: Register8) -> &mut Self {
        self.add8(reg, 1)
    }

    pub fn sub8(&mut self, reg: Register8, n: u8) -> &mut Self {
        let mut v = self.get8(reg) as i32 - n as i32;
        if v < 0 {
            // TODO: Underflow
            v += 0x100
        }

        self.set8(reg, (v & 0xFF) as u8)
    }

    pub fn add16(&mut self, reg: Register16, n: u16) -> &mut Self {
        let v = self.get16(reg) as u32 + n as u32;
        if v > 0xFFFF {
            // TODO: Overflow
        }

        self.set16(reg, (v & 0xFFFF) as u16)
    }

    pub fn inc16(&mut self, reg: Register16) -> &mut Self {
        self.add16(reg, 1)
    }

    pub fn sub16(&mut self, reg: Register16, n: u16) -> &mut Self {
        let mut v = self.get16(reg) as i32 - n as i32;
        if v < 0 {
            // TODO: Underflow
            v += 0x10000
        }

        self.set16(reg, (v & 0xFFFF) as u16)
    }

    // For F
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

    // For PC
    pub fn get_PC(&self) -> u16 {
        self.get16(Register16::PC)
    }
    pub fn inc_PC(&mut self) -> &mut Self {
        self.inc16(Register16::PC)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers_add() {
        let mut reg = Registers::new();

        reg.set8(Register8::A, 0x00).add8(Register8::A, 0x10);
        assert_eq!(0x10, reg.A);
        reg.set8(Register8::A, 0xFF).add8(Register8::A, 0x01);
        assert_eq!(0x00, reg.A);

        reg.set16(Register16::PC, 0x0000)
            .add16(Register16::PC, 0x1000);
        assert_eq!(0x1000, reg.PC);
        reg.set16(Register16::PC, 0xFFFF)
            .add16(Register16::PC, 0x0001);
        assert_eq!(0x0000, reg.PC);
    }

    #[test]
    fn test_registers_sub() {
        let mut reg = Registers::new();

        reg.set8(Register8::A, 0x20).sub8(Register8::A, 0x10);
        assert_eq!(0x10, reg.A);
        reg.set8(Register8::A, 0x00).sub8(Register8::A, 0x01);
        assert_eq!(0xFF, reg.A);

        reg.set16(Register16::PC, 0x2000)
            .sub16(Register16::PC, 0x1000);
        assert_eq!(0x1000, reg.PC);
        reg.set16(Register16::PC, 0x0000)
            .sub16(Register16::PC, 0x0001);
        assert_eq!(0xFFFF, reg.PC);
    }

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
}
