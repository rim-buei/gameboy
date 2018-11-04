use super::super::ram::Ram;

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
    fn read8(&self, reg: &mut Registers, _: &mut Ram) -> u8 {
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
    fn write8(&self, reg: &mut Registers, _: &mut Ram, v: u8) {
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
    fn read16(&self, reg: &mut Registers, _: &mut Ram) -> u16 {
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
    fn write16(&self, reg: &mut Registers, _: &mut Ram, v: u16) {
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

    pub fn add8(&mut self, reg: Register8, b: u8) -> &mut Self {
        let a = self.get8(reg);
        let c = a.wrapping_add(b);

        self.set_flag(Flag::Z, c == 0x00);
        self.disable_flag(Flag::N);
        self.set_flag(Flag::H, (c ^ b ^ a) & 0x10 == 0x10);
        self.set_flag(Flag::C, c < a);

        self.set8(reg, c)
    }

    pub fn adc8(&mut self, reg: Register8, b: u8) -> &mut Self {
        if self.get_flag(Flag::C) {
            let a4 = self.get8(reg) & 0x0F;
            let b4 = b & 0x0F;

            self.add8(reg, 1);
            let carry = self.get_flag(Flag::C);

            self.add8(reg, b);
            self.set_flag(Flag::H, (a4 + b4 + 1) > 0x0F); // This might be wrong...?
            self.set_flag(Flag::C, self.get_flag(Flag::C) | carry);
            self
        } else {
            self.add8(reg, b)
        }
    }

    // inc8 internally calls self.add8(reg, 1) method
    // but it does not affect the carry flag.
    pub fn inc8(&mut self, reg: Register8) -> &mut Self {
        let temp = self.get_flag(Flag::C);
        self.add8(reg, 1);
        self.set_flag(Flag::C, temp)
    }

    pub fn add16(&mut self, reg: Register16, n: u16) -> &mut Self {
        let v = self.get16(reg) as u32 + n as u32;
        if v > 0xFFFF {
            // TODO: Overflow
        }

        self.set16(reg, (v & 0xFFFF) as u16)
    }

    pub fn sub8(&mut self, reg: Register8, b: u8) -> &mut Self {
        let a = self.get8(reg);
        let c = a.wrapping_sub(b);

        self.set_flag(Flag::Z, c == 0x00);
        self.enable_flag(Flag::N);
        self.set_flag(Flag::H, (a & 0x0F) < (b & 0x0F));
        self.set_flag(Flag::C, a < b);

        self.set8(reg, c)
    }

    pub fn sbc8(&mut self, reg: Register8, b: u8) -> &mut Self {
        if self.get_flag(Flag::C) {
            let a = self.get8(reg);

            self.sub8(reg, 1);
            let carry = self.get_flag(Flag::C);

            self.sub8(reg, b);
            let c = self.get8(reg);
            self.set_flag(Flag::H, (c ^ b ^ a) & 0x10 == 0x10); // This might be wrong...?
            self.set_flag(Flag::C, self.get_flag(Flag::C) | carry);
            self
        } else {
            self.sub8(reg, b)
        }
    }

    // dec8 internally calls self.dec8(reg, 1) method
    // but it does not affect the carry flag.
    pub fn dec8(&mut self, reg: Register8) -> &mut Self {
        let temp = self.get_flag(Flag::C);
        self.sub8(reg, 1);
        self.set_flag(Flag::C, temp)
    }

    pub fn sub16(&mut self, reg: Register16, n: u16) -> &mut Self {
        let mut v = self.get16(reg) as i32 - n as i32;
        if v < 0 {
            // TODO: Underflow
            v += 0x10000
        }

        self.set16(reg, (v & 0xFFFF) as u16)
    }

    pub fn and8(&mut self, reg: Register8, b: u8) -> &mut Self {
        let a = self.get8(reg);
        let c = a & b;

        self.set_flag(Flag::Z, c == 0x00);
        self.disable_flag(Flag::N);
        self.enable_flag(Flag::H);
        self.disable_flag(Flag::C);

        self.set8(reg, c)
    }

    pub fn or8(&mut self, reg: Register8, b: u8) -> &mut Self {
        let a = self.get8(reg);
        let c = a | b;

        self.set_flag(Flag::Z, c == 0x00);
        self.disable_flag(Flag::N);
        self.disable_flag(Flag::H);
        self.disable_flag(Flag::C);

        self.set8(reg, c)
    }

    pub fn xor8(&mut self, reg: Register8, b: u8) -> &mut Self {
        let a = self.get8(reg);
        let c = a ^ b;

        self.set_flag(Flag::Z, c == 0x00);
        self.disable_flag(Flag::N);
        self.disable_flag(Flag::H);
        self.disable_flag(Flag::C);

        self.set8(reg, c)
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

    // For PC
    pub fn get_PC(&self) -> u16 {
        self.get16(Register16::PC)
    }
    pub fn inc_PC(&mut self) -> &mut Self {
        self.set16(Register16::PC, self.get16(Register16::PC).wrapping_add(1))
    }
}

// TODO: Should be moved to other module
#[derive(Debug, Copy, Clone)]
pub enum Address {
    BC,
    DE,
    HL,
}

impl Reader8 for Address {
    fn read8(&self, reg: &mut Registers, ram: &mut Ram) -> u8 {
        use self::Address::*;

        let src = match *self {
            BC => Register16::BC,
            DE => Register16::DE,
            HL => Register16::HL,
        };

        let addr = src.read16(reg, ram);
        ram.read(addr)
    }
}

impl Writer8 for Address {
    fn write8(&self, reg: &mut Registers, ram: &mut Ram, v: u8) {
        use self::Address::*;

        let dst = match *self {
            BC => Register16::BC,
            DE => Register16::DE,
            HL => Register16::HL,
        };

        let addr = dst.read16(reg, ram);
        ram.write(addr, v)
    }
}

// TODO: Should be moved to other module
#[derive(Debug, Copy, Clone)]
pub struct Immediate8;

impl Reader8 for Immediate8 {
    fn read8(&self, reg: &mut Registers, ram: &mut Ram) -> u8 {
        ram.read(reg.PC + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FlagZNHC(bool, bool, bool, bool);

    impl FlagZNHC {
        fn new(reg: Registers) -> Self {
            FlagZNHC(
                reg.get_flag(Flag::Z),
                reg.get_flag(Flag::N),
                reg.get_flag(Flag::H),
                reg.get_flag(Flag::C),
            )
        }
    }

    impl PartialEq for FlagZNHC {
        fn eq(&self, other: &FlagZNHC) -> bool {
            self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
        }
    }

    #[test]
    fn test_registers_add8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x00,
                b: 0x01,
                c: 0x01,
                flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x0F,
                b: 0x01,
                c: 0x10,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0xF0,
                b: 0x10,
                c: 0x00,
                flags: FlagZNHC(true, false, false, true),
            },
            TestCase {
                a: 0xFF,
                b: 0x01,
                c: 0x00,
                flags: FlagZNHC(true, false, true, true),
            },
        ] {
            let mut reg = Registers::new();
            reg.set8(Register8::A, test.a);

            reg.add8(Register8::A, test.b);
            assert_eq!(test.c, reg.get8(Register8::A));
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_registers_adc8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x00,
                b: 0x01,
                c: 0x02,
                flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x0E,
                b: 0x01,
                c: 0x10,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0x00,
                b: 0x0F,
                c: 0x10,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0xF0,
                b: 0x10,
                c: 0x01,
                flags: FlagZNHC(false, false, false, true),
            },
            TestCase {
                a: 0x00,
                b: 0xFF,
                c: 0x00,
                flags: FlagZNHC(true, false, true, true),
            },
        ] {
            let mut reg = Registers::new();
            reg.enable_flag(Flag::C).set8(Register8::A, test.a);

            reg.adc8(Register8::A, test.b);
            assert_eq!(test.c, reg.get8(Register8::A));
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_registers_inc8() {
        let mut reg = Registers::new();

        reg.set8(Register8::A, 0xFF).inc8(Register8::A);
        assert_eq!(0x00, reg.get8(Register8::A));
        assert_eq!(false, reg.get_flag(Flag::C));
    }

    #[test]
    fn test_registers_add16() {
        let mut reg = Registers::new();

        reg.set16(Register16::PC, 0x0000)
            .add16(Register16::PC, 0x1000);
        assert_eq!(0x1000, reg.PC);
        reg.set16(Register16::PC, 0xFFFF)
            .add16(Register16::PC, 0x0001);
        assert_eq!(0x0000, reg.PC);
    }

    #[test]
    fn test_registers_sub8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x02,
                b: 0x01,
                c: 0x01,
                flags: FlagZNHC(false, true, false, false),
            },
            TestCase {
                a: 0x10,
                b: 0x01,
                c: 0x0F,
                flags: FlagZNHC(false, true, true, false),
            },
            TestCase {
                a: 0x00,
                b: 0x10,
                c: 0xF0,
                flags: FlagZNHC(false, true, false, true),
            },
            TestCase {
                a: 0x00,
                b: 0x01,
                c: 0xFF,
                flags: FlagZNHC(false, true, true, true),
            },
        ] {
            let mut reg = Registers::new();
            reg.set8(Register8::A, test.a);

            reg.sub8(Register8::A, test.b);
            assert_eq!(test.c, reg.get8(Register8::A));
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_registers_sbc8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x03,
                b: 0x01,
                c: 0x01,
                flags: FlagZNHC(false, true, false, false),
            },
            TestCase {
                a: 0x11,
                b: 0x01,
                c: 0x0F,
                flags: FlagZNHC(false, true, true, false),
            },
            TestCase {
                a: 0x10,
                b: 0x01,
                c: 0x0E,
                flags: FlagZNHC(false, true, true, false),
            },
            TestCase {
                a: 0x00,
                b: 0x0F,
                c: 0xF0,
                flags: FlagZNHC(false, true, true, true),
            },
            TestCase {
                a: 0x00,
                b: 0xFF,
                c: 0x00,
                flags: FlagZNHC(true, true, true, true),
            },
        ] {
            let mut reg = Registers::new();
            reg.enable_flag(Flag::C);
            reg.set8(Register8::A, test.a);

            reg.sbc8(Register8::A, test.b);
            assert_eq!(test.c, reg.get8(Register8::A));
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_registers_dec8() {
        let mut reg = Registers::new();

        reg.set8(Register8::A, 0x00).dec8(Register8::A);
        assert_eq!(0xFF, reg.get8(Register8::A));
        assert_eq!(false, reg.get_flag(Flag::C));
    }

    #[test]
    fn test_registers_sub16() {
        let mut reg = Registers::new();

        reg.set16(Register16::PC, 0x2000)
            .sub16(Register16::PC, 0x1000);
        assert_eq!(0x1000, reg.PC);
        reg.set16(Register16::PC, 0x0000)
            .sub16(Register16::PC, 0x0001);
        assert_eq!(0xFFFF, reg.PC);
    }

    #[test]
    fn test_registers_logical8() {
        struct TestCase {
            a: u8,
            b: u8,
            and: u8,
            and_flags: FlagZNHC,
            or: u8,
            or_flags: FlagZNHC,
            xor: u8,
            xor_flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x01,
                b: 0x01,
                and: 0x01,
                and_flags: FlagZNHC(false, false, true, false),
                or: 0x01,
                or_flags: FlagZNHC(false, false, false, false),
                xor: 0x00,
                xor_flags: FlagZNHC(true, false, false, false),
            },
            TestCase {
                a: 0x01,
                b: 0x00,
                and: 0x00,
                and_flags: FlagZNHC(true, false, true, false),
                or: 0x01,
                or_flags: FlagZNHC(false, false, false, false),
                xor: 0x01,
                xor_flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x00,
                b: 0x00,
                and: 0x00,
                and_flags: FlagZNHC(true, false, true, false),
                or: 0x00,
                or_flags: FlagZNHC(true, false, false, false),
                xor: 0x00,
                xor_flags: FlagZNHC(true, false, false, false),
            },
        ] {
            let mut reg = Registers::new();

            reg.set8(Register8::A, test.a);
            reg.and8(Register8::A, test.b);
            assert_eq!(test.and, reg.get8(Register8::A));
            assert_eq!(test.and_flags, FlagZNHC::new(reg));

            reg.set8(Register8::A, test.a);
            reg.or8(Register8::A, test.b);
            assert_eq!(test.or, reg.get8(Register8::A));
            assert_eq!(test.or_flags, FlagZNHC::new(reg));

            reg.set8(Register8::A, test.a);
            reg.xor8(Register8::A, test.b);
            assert_eq!(test.xor, reg.get8(Register8::A));
            assert_eq!(test.xor_flags, FlagZNHC::new(reg));
        }
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
