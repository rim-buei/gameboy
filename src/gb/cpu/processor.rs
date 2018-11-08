use super::super::ram::Ram;

use super::io::{Reader16, Reader8, Writer16, Writer8};
use super::register::{Address, Flag, Register16 as R16, Register8 as R8, Registers};

pub struct Processor<'a> {
    pub reg: &'a mut Registers,
    pub ram: &'a mut Ram,

    extra_cycle: u8,
}

impl<'a> Processor<'a> {
    pub fn new(reg: &'a mut Registers, ram: &'a mut Ram) -> Self {
        Processor {
            reg: reg,
            ram: ram,

            extra_cycle: 0,
        }
    }

    pub fn r(&mut self, opsize: u8, base_cycle: u8) -> (u8, u8) {
        let cycle = base_cycle + self.extra_cycle;
        self.extra_cycle = 0;
        (opsize, cycle)
    }

    pub fn ld8<R: Reader8, W: Writer8>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read8(self.reg, self.ram);
        lhs.write8(self.reg, self.ram, v);
        self
    }

    pub fn ld16<R: Reader16, W: Writer16>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read16(self.reg, self.ram);
        lhs.write16(self.reg, self.ram, v);
        self
    }

    pub fn add8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.reg.A as u16;
        let b = rhs.read8(self.reg, self.ram) as u16;
        let c = a + b;
        let hcarry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;

        self.reg.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.reg.disable_flag(Flag::N);
        self.reg.set_flag(Flag::H, hcarry);
        self.reg.set_flag(Flag::C, c > 0xFF);

        self.reg.A = c as u8;
        self
    }

    pub fn add16<R: Reader16>(&mut self, rhs: R) -> &mut Self {
        let a = R16::HL.read16(self.reg, self.ram) as u32;
        let b = rhs.read16(self.reg, self.ram) as u32;
        let c = a + b;
        let hcarry = ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF;

        self.reg.disable_flag(Flag::N);
        self.reg.set_flag(Flag::H, hcarry);
        self.reg.set_flag(Flag::C, c > 0xFFFF);

        R16::HL.write16(self.reg, self.ram, c as u16);
        self
    }

    pub fn add_sp<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        // This implementation might be wrong
        let a = R16::SP.read16(self.reg, self.ram) as u32;
        let b = rhs.read8(self.reg, self.ram) as u32;
        let c = a + b;
        let hcarry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;
        let carry = ((a & 0xFF) + (b & 0xFF)) > 0xFF;

        self.reg.disable_flag(Flag::Z);
        self.reg.disable_flag(Flag::N);
        self.reg.set_flag(Flag::H, hcarry);
        self.reg.set_flag(Flag::C, carry);

        R16::SP.write16(self.reg, self.ram, c as u16);
        self
    }

    pub fn adc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.reg.A as u16;
        let b = rhs.read8(self.reg, self.ram) as u16;
        let carry = if self.reg.get_flag(Flag::C) { 1 } else { 0 } as u16;
        let c = a + b + carry;
        let hcarry = ((a & 0x0F) + (b & 0x0F) + carry) > 0x0F;

        self.reg.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.reg.disable_flag(Flag::N);
        self.reg.set_flag(Flag::H, hcarry);
        self.reg.set_flag(Flag::C, c > 0xFF);

        self.reg.A = c as u8;
        self
    }

    pub fn inc8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.reg, self.ram).wrapping_add(1);

        self.reg.set_flag(Flag::Z, v == 0x00);
        self.reg.disable_flag(Flag::N);
        self.reg.set_flag(Flag::H, (v & 0x0F) == 0x00);

        rw.write8(self.reg, self.ram, v);
        self
    }

    pub fn inc16<RW: Reader16 + Writer16>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read16(self.reg, self.ram).wrapping_add(1);
        rw.write16(self.reg, self.ram, v);
        self
    }

    pub fn sub8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.reg.A as i16;
        let b = rhs.read8(self.reg, self.ram) as i16;
        let c = a - b;
        let hcarry = ((a & 0x0F) - (b & 0x0F)) < 0;

        self.reg.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.reg.enable_flag(Flag::N);
        self.reg.set_flag(Flag::H, hcarry);
        self.reg.set_flag(Flag::C, c < 0);

        self.reg.A = c as u8;
        self
    }

    pub fn sbc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.reg.A as i16;
        let b = rhs.read8(self.reg, self.ram) as i16;
        let carry = if self.reg.get_flag(Flag::C) { 1 } else { 0 } as i16;
        let c = a - b - carry;
        let hcarry = ((a & 0x0F) - (b & 0x0F) - carry) < 0;

        self.reg.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.reg.enable_flag(Flag::N);
        self.reg.set_flag(Flag::H, hcarry);
        self.reg.set_flag(Flag::C, c < 0);

        self.reg.A = c as u8;
        self
    }

    pub fn dec8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.reg, self.ram).wrapping_sub(1);

        self.reg.set_flag(Flag::Z, v == 0x00);
        self.reg.enable_flag(Flag::N);
        self.reg.set_flag(Flag::H, (v & 0x0F) == 0x0F);

        rw.write8(self.reg, self.ram, v);
        self
    }

    pub fn dec16<RW: Reader16 + Writer16>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read16(self.reg, self.ram).wrapping_sub(1);
        rw.write16(self.reg, self.ram, v);
        self
    }

    pub fn and8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.reg.A & rhs.read8(self.reg, self.ram);

        self.reg.set_flag(Flag::Z, c == 0x00);
        self.reg.disable_flag(Flag::N);
        self.reg.enable_flag(Flag::H);
        self.reg.disable_flag(Flag::C);

        self.reg.A = c;
        self
    }

    pub fn or8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.reg.A | rhs.read8(self.reg, self.ram);

        self.reg.set_flag(Flag::Z, c == 0x00);
        self.reg.disable_flag(Flag::N);
        self.reg.disable_flag(Flag::H);
        self.reg.disable_flag(Flag::C);

        self.reg.A = c;
        self
    }

    pub fn xor8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.reg.A ^ rhs.read8(self.reg, self.ram);

        self.reg.set_flag(Flag::Z, c == 0x00);
        self.reg.disable_flag(Flag::N);
        self.reg.disable_flag(Flag::H);
        self.reg.disable_flag(Flag::C);

        self.reg.A = c;
        self
    }

    pub fn cp8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let temp = self.reg.A;
        self.sub8(rhs);
        self.reg.A = temp;
        self
    }

    pub fn push16<R: Reader16>(&mut self, r: R) -> &mut Self {
        let sp = R16::SP.read16(self.reg, self.ram);
        let v = r.read16(self.reg, self.ram);
        self.ram.write(sp.wrapping_sub(1), (v >> 8) as u8);
        self.ram.write(sp.wrapping_sub(2), (v & 0xFF) as u8);

        self.dec16(R16::SP);
        self.dec16(R16::SP);
        self
    }

    pub fn pop16<W: Writer16>(&mut self, w: W) -> &mut Self {
        let sp = R16::SP.read16(self.reg, self.ram);
        let v = self.ram.read(sp) as u16 | ((self.ram.read(sp.wrapping_add(1)) as u16) << 8);
        w.write16(self.reg, self.ram, v);

        self.inc16(R16::SP);
        self.inc16(R16::SP);
        self
    }

    pub fn undefined(&mut self, opcode: u8) -> &mut Self {
        println!("Unsupported or unknown opcode specified: 0x{:02X}", opcode);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::register::{Immediate16, Immediate8};

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

        let mut p = Processor::new(&mut reg, &mut ram);
        p.ld8(R8::B, R8::A);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_processor_ld8_r8_hl() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        reg.L = 0x01;

        let mut p = Processor::new(&mut reg, &mut ram);
        p.ld8(R8::B, Address::HL);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_processor_ld8_r8_d8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);

        let mut p = Processor::new(&mut reg, &mut ram);
        p.ld8(R8::B, Immediate8);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_processor_ld16() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAB, 0xCD]);

        let mut p = Processor::new(&mut reg, &mut ram);
        p.ld16(R16::SP, Immediate16);
        assert_eq!(0xCDAB, reg.SP);
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

            let mut p = Processor::new(&mut reg, &mut ram);
            p.add8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_processor_add16() {
        struct TestCase {
            a: u16,
            b: u16,
            c: u16,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x00FF,
                b: 0x0001,
                c: 0x0100,
                flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x0FFF,
                b: 0x0001,
                c: 0x1000,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0xF000,
                b: 0x1000,
                c: 0x0000,
                flags: FlagZNHC(false, false, false, true),
            },
            TestCase {
                a: 0xFFFF,
                b: 0x0001,
                c: 0x0000,
                flags: FlagZNHC(false, false, true, true),
            },
        ] {
            let mut reg = Registers::new();
            let mut ram = Ram::new(vec![0x00]);
            R16::HL.write16(&mut reg, &mut ram, test.a);
            R16::BC.write16(&mut reg, &mut ram, test.b);

            let mut p = Processor::new(&mut reg, &mut ram);
            p.add16(R16::BC);
            assert_eq!(test.c, R16::HL.read16(&mut reg, &mut ram));
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

            let mut p = Processor::new(&mut reg, &mut ram);
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

            let mut p = Processor::new(&mut reg, &mut ram);
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

            let mut p = Processor::new(&mut reg, &mut ram);
            p.sbc8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_processor_inc_dec_hl() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0x0F]);
        reg.L = 0x01;

        let mut p = Processor::new(&mut reg, &mut ram);
        p.inc8(Address::HL);
        assert_eq!(0x10, p.ram.read(1));
        p.dec8(Address::HL);
        assert_eq!(0x0F, p.ram.read(1));
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

                let mut p = Processor::new(&mut reg, &mut ram);
                p.and8(R8::B);
                assert_eq!(test.and, reg.A);
                assert_eq!(test.and_flags, FlagZNHC::new(reg));
            }
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut p = Processor::new(&mut reg, &mut ram);
                p.or8(R8::B);
                assert_eq!(test.or, reg.A);
                assert_eq!(test.or_flags, FlagZNHC::new(reg));
            }
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut p = Processor::new(&mut reg, &mut ram);
                p.xor8(R8::B);
                assert_eq!(test.xor, reg.A);
                assert_eq!(test.xor_flags, FlagZNHC::new(reg));
            }
        }
    }

    #[test]
    fn test_processor_cp8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00]);
        reg.enable_flag(Flag::C);
        reg.B = 0x01;

        let mut p = Processor::new(&mut reg, &mut ram);
        p.cp8(R8::B);
        assert_eq!(0x00, reg.A);
        assert_eq!(FlagZNHC(false, true, true, true), FlagZNHC::new(reg));
    }

    #[test]
    fn test_processor_push_pop() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0x00]);
        R16::SP.write16(&mut reg, &mut ram, 0x0002);
        R16::BC.write16(&mut reg, &mut ram, 0xABCD);

        let mut p = Processor::new(&mut reg, &mut ram);
        p.push16(R16::BC);
        assert_eq!(0xAB, p.ram.read(0x0001));
        assert_eq!(0xCD, p.ram.read(0x0000));
        assert_eq!(0x0000, p.reg.SP);
        p.pop16(R16::DE);
        assert_eq!(0x0002, p.reg.SP);
        assert_eq!(0xABCD, R16::DE.read16(&mut reg, &mut ram));
    }
}
