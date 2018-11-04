use super::super::ram::Ram;

use super::io::{Reader16, Reader8, Writer16, Writer8};
use super::register::{Address, Flag, Registers};

pub struct Processor<'a>(pub &'a mut Registers, pub &'a mut Ram);

impl<'a> Processor<'a> {
    pub fn r(&mut self, opsize: u8, cycle: u8) -> (u8, u8) {
        (opsize, cycle)
    }

    pub fn ld8<R: Reader8, W: Writer8>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read8(self.0, self.1);
        lhs.write8(self.0, self.1, v);
        self
    }

    pub fn add8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as u16;
        let b = rhs.read8(self.0, self.1) as u16;
        let c = a + b;
        let hc = ((a & 0x0F) + (b & 0x0F)) > 0x0F;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c > 0xFF);

        self.0.A = c as u8;
        self
    }

    pub fn adc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as u16;
        let b = rhs.read8(self.0, self.1) as u16;
        let carry = if self.0.get_flag(Flag::C) { 1 } else { 0 } as u16;
        let c = a + b + carry;
        let hc = ((a & 0x0F) + (b & 0x0F) + carry) > 0x0F;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c > 0xFF);

        self.0.A = c as u8;
        self
    }

    pub fn inc8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.0, self.1).wrapping_add(1);

        self.0.set_flag(Flag::Z, v == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.set_flag(Flag::H, (v & 0x0F) == 0x00);

        rw.write8(self.0, self.1, v);
        self
    }

    pub fn inc16<RW: Reader16 + Writer16>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read16(self.0, self.1).wrapping_add(1);
        rw.write16(self.0, self.1, v);
        self
    }

    pub fn sub8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as i16;
        let b = rhs.read8(self.0, self.1) as i16;
        let c = a - b;
        let hc = ((a & 0x0F) - (b & 0x0F)) < 0;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.enable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c < 0);

        self.0.A = c as u8;
        self
    }

    pub fn sbc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as i16;
        let b = rhs.read8(self.0, self.1) as i16;
        let carry = if self.0.get_flag(Flag::C) { 1 } else { 0 } as i16;
        let c = a - b - carry;
        let hc = ((a & 0x0F) - (b & 0x0F) - carry) < 0;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.enable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c < 0);

        self.0.A = c as u8;
        self
    }

    pub fn dec8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.0, self.1).wrapping_sub(1);

        self.0.set_flag(Flag::Z, v == 0x00);
        self.0.enable_flag(Flag::N);
        self.0.set_flag(Flag::H, (v & 0x0F) == 0x0F);

        rw.write8(self.0, self.1, v);
        self
    }

    pub fn dec16<RW: Reader16 + Writer16>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read16(self.0, self.1).wrapping_sub(1);
        rw.write16(self.0, self.1, v);
        self
    }

    pub fn and8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.0.A & rhs.read8(self.0, self.1);

        self.0.set_flag(Flag::Z, c == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.enable_flag(Flag::H);
        self.0.disable_flag(Flag::C);

        self.0.A = c;
        self
    }

    pub fn or8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.0.A | rhs.read8(self.0, self.1);

        self.0.set_flag(Flag::Z, c == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.disable_flag(Flag::H);
        self.0.disable_flag(Flag::C);

        self.0.A = c;
        self
    }

    pub fn xor8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.0.A ^ rhs.read8(self.0, self.1);

        self.0.set_flag(Flag::Z, c == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.disable_flag(Flag::H);
        self.0.disable_flag(Flag::C);

        self.0.A = c;
        self
    }

    pub fn undefined(&mut self, opcode: u8) -> &mut Self {
        println!("Unsupported or unknown opcode specified: 0x{:02X}", opcode);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::register::Immediate8;
    use super::super::register::Register16 as R16;
    use super::super::register::Register8 as R8;

    use super::*;

    #[derive(Debug, PartialEq)]
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

    #[test]
    fn test_processor_ld8_r8_r8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00]);
        reg.A = 0xAA;

        let mut p = Processor(&mut reg, &mut ram);
        p.ld8(R8::B, R8::A);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_processor_ld8_r8_hl() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        reg.L = 0x01;

        let mut p = Processor(&mut reg, &mut ram);
        p.ld8(R8::B, Address::HL);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_processor_ld8_r8_d8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);

        let mut p = Processor(&mut reg, &mut ram);
        p.ld8(R8::B, Immediate8);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_processor_add8() {
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
            let mut ram = Ram::new(vec![0x00]);
            reg.A = test.a;
            reg.B = test.b;

            let mut p = Processor(&mut reg, &mut ram);
            p.add8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_processor_adc8() {
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
            let mut ram = Ram::new(vec![0x00]);
            reg.enable_flag(Flag::C);
            reg.A = test.a;
            reg.B = test.b;

            let mut p = Processor(&mut reg, &mut ram);
            p.adc8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_processor_sub8() {
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
            let mut ram = Ram::new(vec![0x00]);
            reg.A = test.a;
            reg.B = test.b;

            let mut p = Processor(&mut reg, &mut ram);
            p.sub8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_processor_sbc8() {
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
            let mut ram = Ram::new(vec![0x00]);
            reg.enable_flag(Flag::C);
            reg.A = test.a;
            reg.B = test.b;

            let mut p = Processor(&mut reg, &mut ram);
            p.sbc8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_processor_logical8() {
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
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut p = Processor(&mut reg, &mut ram);
                p.and8(R8::B);
                assert_eq!(test.and, reg.A);
                assert_eq!(test.and_flags, FlagZNHC::new(reg));
            }
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut p = Processor(&mut reg, &mut ram);
                p.or8(R8::B);
                assert_eq!(test.or, reg.A);
                assert_eq!(test.or_flags, FlagZNHC::new(reg));
            }
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut p = Processor(&mut reg, &mut ram);
                p.xor8(R8::B);
                assert_eq!(test.xor, reg.A);
                assert_eq!(test.xor_flags, FlagZNHC::new(reg));
            }
        }
    }
}
