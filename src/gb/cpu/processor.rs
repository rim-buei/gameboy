use super::super::bus::Bus;
use super::io::{Reader16, Reader8, Writer16, Writer8};
use super::oprand::{Address, Condition, Data16, Immediate16, Immediate8, Register16 as R16, Register8 as R8};
use super::state::{Flag, State};

pub struct Processor<'a, B: Bus + 'a> {
    pub state: &'a mut State,
    pub bus: &'a mut B,

    extra_cycle: u8,
}

impl<'a, B: Bus + 'a> Processor<'a, B> {
    pub fn new(state: &'a mut State, bus: &'a mut B) -> Self {
        Processor {
            state: state,
            bus: bus,

            extra_cycle: 0,
        }
    }

    pub fn r(&mut self, opsize: u8, base_cycle: u8) -> (u8, u8) {
        let cycle = base_cycle + self.extra_cycle;
        self.extra_cycle = 0;
        (opsize, cycle)
    }

    pub fn ld8<R: Reader8, W: Writer8>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read8(self.state, self.bus);
        lhs.write8(self.state, self.bus, v);
        self
    }

    pub fn ld8_hli<R: Reader8, W: Writer8>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read8(self.state, self.bus);
        lhs.write8(self.state, self.bus, v);
        self.inc16(R16::HL)
    }

    pub fn ld8_hld<R: Reader8, W: Writer8>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read8(self.state, self.bus);
        lhs.write8(self.state, self.bus, v);
        self.dec16(R16::HL)
    }

    pub fn ld16<R: Reader16, W: Writer16>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read16(self.state, self.bus);
        lhs.write16(self.state, self.bus, v);
        self
    }

    pub fn add8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.state.A as u16;
        let b = rhs.read8(self.state, self.bus) as u16;
        let c = a + b;
        let hcarry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;

        self.state.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.state.disable_flag(Flag::N);
        self.state.set_flag(Flag::H, hcarry);
        self.state.set_flag(Flag::C, c > 0xFF);

        self.state.A = c as u8;
        self
    }

    pub fn add16<R: Reader16>(&mut self, rhs: R) -> &mut Self {
        let a = R16::HL.read16(self.state, self.bus) as u32;
        let b = rhs.read16(self.state, self.bus) as u32;
        let c = a + b;
        let hcarry = ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF;

        self.state.disable_flag(Flag::N);
        self.state.set_flag(Flag::H, hcarry);
        self.state.set_flag(Flag::C, c > 0xFFFF);

        R16::HL.write16(self.state, self.bus, c as u16);
        self
    }

    pub fn add_r16_e8<R16: Reader16, R8: Reader8>(&mut self, lhs: R16, rhs: R8) -> u16 {
        // This implementation might be wrong
        let a = lhs.read16(self.state, self.bus) as u16;
        let b = rhs.read8(self.state, self.bus) as i8;
        let c = if 0 < b {
            a.wrapping_add(b as u16)
        } else {
            a.wrapping_sub(b.abs() as u16)
        };
        let hcarry = (c & 0x0F) < (a & 0x0F);
        let carry = (c & 0xFF) < (a & 0xFF);

        self.state.disable_flag(Flag::Z);
        self.state.disable_flag(Flag::N);
        self.state.set_flag(Flag::H, hcarry);
        self.state.set_flag(Flag::C, carry);

        c
    }

    pub fn add_sp_e8(&mut self) -> &mut Self {
        let addr = self.add_r16_e8(R16::SP, Immediate8);
        R16::SP.write16(self.state, self.bus, addr);
        self
    }

    pub fn ld_hl_sp_e8(&mut self) -> &mut Self {
        let addr = self.add_r16_e8(R16::SP, Immediate8);
        R16::HL.write16(self.state, self.bus, addr);
        self
    }

    pub fn adc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.state.A as u16;
        let b = rhs.read8(self.state, self.bus) as u16;
        let carry = self.state.get_flag(Flag::C) as u16;
        let c = a + b + carry;
        let hcarry = ((a & 0x0F) + (b & 0x0F) + carry) > 0x0F;

        self.state.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.state.disable_flag(Flag::N);
        self.state.set_flag(Flag::H, hcarry);
        self.state.set_flag(Flag::C, c > 0xFF);

        self.state.A = c as u8;
        self
    }

    pub fn inc8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.state, self.bus).wrapping_add(1);

        self.state.set_flag(Flag::Z, v == 0x00);
        self.state.disable_flag(Flag::N);
        self.state.set_flag(Flag::H, (v & 0x0F) == 0x00);

        rw.write8(self.state, self.bus, v);
        self
    }

    pub fn inc16<RW: Reader16 + Writer16>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read16(self.state, self.bus).wrapping_add(1);
        rw.write16(self.state, self.bus, v);
        self
    }

    pub fn sub8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.state.A as i16;
        let b = rhs.read8(self.state, self.bus) as i16;
        let c = a - b;
        let hcarry = ((a & 0x0F) - (b & 0x0F)) < 0;

        self.state.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.state.enable_flag(Flag::N);
        self.state.set_flag(Flag::H, hcarry);
        self.state.set_flag(Flag::C, c < 0);

        self.state.A = c as u8;
        self
    }

    pub fn sbc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.state.A as i16;
        let b = rhs.read8(self.state, self.bus) as i16;
        let carry = self.state.get_flag(Flag::C) as i16;
        let c = a - b - carry;
        let hcarry = ((a & 0x0F) - (b & 0x0F) - carry) < 0;

        self.state.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.state.enable_flag(Flag::N);
        self.state.set_flag(Flag::H, hcarry);
        self.state.set_flag(Flag::C, c < 0);

        self.state.A = c as u8;
        self
    }

    pub fn dec8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.state, self.bus).wrapping_sub(1);

        self.state.set_flag(Flag::Z, v == 0x00);
        self.state.enable_flag(Flag::N);
        self.state.set_flag(Flag::H, (v & 0x0F) == 0x0F);

        rw.write8(self.state, self.bus, v);
        self
    }

    pub fn dec16<RW: Reader16 + Writer16>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read16(self.state, self.bus).wrapping_sub(1);
        rw.write16(self.state, self.bus, v);
        self
    }

    pub fn and8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.state.A & rhs.read8(self.state, self.bus);

        self.state.set_flag(Flag::Z, c == 0x00);
        self.state.disable_flag(Flag::N);
        self.state.enable_flag(Flag::H);
        self.state.disable_flag(Flag::C);

        self.state.A = c;
        self
    }

    pub fn or8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.state.A | rhs.read8(self.state, self.bus);

        self.state.set_flag(Flag::Z, c == 0x00);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.disable_flag(Flag::C);

        self.state.A = c;
        self
    }

    pub fn xor8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.state.A ^ rhs.read8(self.state, self.bus);

        self.state.set_flag(Flag::Z, c == 0x00);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.disable_flag(Flag::C);

        self.state.A = c;
        self
    }

    pub fn cp8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let temp = self.state.A;
        self.sub8(rhs);
        self.state.A = temp;
        self
    }

    pub fn daa(&mut self) -> &mut Self {
        let a = self.state.A;

        let mut b = 0x00;
        if self.state.get_flag(Flag::Z) {
            if self.state.get_flag(Flag::H) {
                b |= 0x06;
            }
            if self.state.get_flag(Flag::C) {
                b |= 0x60;
            }
        } else {
            if self.state.get_flag(Flag::H) || (a & 0x0F) > 0x09 {
                b |= 0x06;
            }
            if self.state.get_flag(Flag::C) || (a & 0xFF) > 0x99 {
                b |= 0x60;
            }
        }

        let c = if self.state.get_flag(Flag::N) {
            a.wrapping_sub(b)
        } else {
            a.wrapping_add(b)
        };

        self.state.set_flag(Flag::Z, c == 0);
        self.state.disable_flag(Flag::H);

        if ((b as u16) << 2) & 0x100 != 0 {
            self.state.enable_flag(Flag::C);
        }

        self.state.A = c;
        self
    }

    pub fn rl8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = (r << 1) | self.state.get_flag(Flag::C) as u8;

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x80) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn rlc8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = (r << 1) | (r >> 7);

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x80) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn rr8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = (r >> 1) | ((self.state.get_flag(Flag::C) as u8) << 7);

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x01) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn rrc8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = (r >> 1) | (r << 7);

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x01) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn rla(&mut self) -> &mut Self {
        self.rl8(R8::A);
        self.state.disable_flag(Flag::Z);
        self
    }

    pub fn rlca(&mut self) -> &mut Self {
        self.rlc8(R8::A);
        self.state.disable_flag(Flag::Z);
        self
    }

    pub fn rra(&mut self) -> &mut Self {
        self.rr8(R8::A);
        self.state.disable_flag(Flag::Z);
        self
    }

    pub fn rrca(&mut self) -> &mut Self {
        self.rrc8(R8::A);
        self.state.disable_flag(Flag::Z);
        self
    }

    pub fn sla8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = r << 1;

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x80) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn sra8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = (r >> 1) | (r & 0x80);

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x01) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn srl8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = r >> 1;

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, (r & 0x01) != 0);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn swap8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let r = rw.read8(self.state, self.bus);
        let w = (r << 4) | (r >> 4);

        self.state.set_flag(Flag::Z, w == 0);
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.disable_flag(Flag::C);

        rw.write8(self.state, self.bus, w);
        self
    }

    pub fn bit8<R: Reader8>(&mut self, bit: u8, r: R) -> &mut Self {
        let v = r.read8(self.state, self.bus);

        self.state.set_flag(Flag::Z, (v & (1 << bit)) == 0);
        self.state.disable_flag(Flag::N);
        self.state.enable_flag(Flag::H);
        self
    }

    pub fn set8<RW: Reader8 + Writer8>(&mut self, bit: u8, rw: RW) -> &mut Self {
        let v = rw.read8(self.state, self.bus) | (1 << bit);

        rw.write8(self.state, self.bus, v);
        self
    }

    pub fn res8<RW: Reader8 + Writer8>(&mut self, bit: u8, rw: RW) -> &mut Self {
        let v = rw.read8(self.state, self.bus) & !(1 << bit);

        rw.write8(self.state, self.bus, v);
        self
    }

    pub fn push16<R: Reader16>(&mut self, r: R) -> &mut Self {
        let sp = R16::SP.read16(self.state, self.bus);
        let v = r.read16(self.state, self.bus);
        self.bus.write16(sp.wrapping_sub(2), v);

        self.dec16(R16::SP);
        self.dec16(R16::SP);
        self
    }

    pub fn pop16<W: Writer16>(&mut self, w: W) -> &mut Self {
        let sp = R16::SP.read16(self.state, self.bus);
        let v = self.bus.read16(sp);
        w.write16(self.state, self.bus, v);

        self.inc16(R16::SP);
        self.inc16(R16::SP);
        self
    }

    pub fn jp<R: Reader16>(&mut self, cond: Condition, r: R) -> &mut Self {
        if cond.test(self.state) {
            let addr = r.read16(self.state, self.bus);
            self.state.PC = addr;

            self.extra_cycle += 4;
        }
        self
    }

    pub fn jr<R: Reader8>(&mut self, cond: Condition, r: R) -> &mut Self {
        if cond.test(self.state) {
            let offset = r.read8(self.state, self.bus) as i8;
            if 0 < offset {
                self.state.PC = self.state.PC.wrapping_add(offset as u16);
            } else {
                self.state.PC = self.state.PC.wrapping_sub(offset.abs() as u16);
            }

            self.extra_cycle += 4;
        }
        self
    }

    pub fn call<R: Reader16>(&mut self, cond: Condition, r: R) -> &mut Self {
        // PC + opcode (1-byte) + oprand (2-byte)
        let next_addr = self.state.PC.wrapping_add(3);

        if cond.test(self.state) {
            // Push next instruction onto stack
            self.push16(Data16(next_addr));

            let addr = r.read16(self.state, self.bus);
            self.state.PC = addr;

            self.extra_cycle += 12;
        } else {
            self.state.PC = next_addr;
        }
        self
    }

    pub fn ret(&mut self, cond: Condition) -> &mut Self {
        if cond.test(self.state) {
            self.pop16(R16::PC);

            self.extra_cycle += 12;
        } else {
            self.state.PC = self.state.PC.wrapping_add(1);
        }
        self
    }

    pub fn reti(&mut self) -> &mut Self {
        self.ret(Condition::T);
        self.ei()
    }

    pub fn rst(&mut self, addr: u16) -> &mut Self {
        self.push16(R16::PC);
        self.state.PC = addr;
        self
    }

    pub fn ei(&mut self) -> &mut Self {
        self.state.IME = true;
        self
    }

    pub fn di(&mut self) -> &mut Self {
        self.state.IME = false;
        self
    }

    // Complement A register
    pub fn cpl(&mut self) -> &mut Self {
        self.state.A ^= 0xFF;
        self.state.enable_flag(Flag::N);
        self.state.enable_flag(Flag::H);
        self
    }

    // Complement carry flag
    pub fn ccf(&mut self) -> &mut Self {
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.set_flag(Flag::C, !self.state.get_flag(Flag::C));
        self
    }

    // Set carry flag
    pub fn scf(&mut self) -> &mut Self {
        self.state.disable_flag(Flag::N);
        self.state.disable_flag(Flag::H);
        self.state.enable_flag(Flag::C);
        self
    }

    pub fn undefined(&mut self, opcode: u8) -> &mut Self {
        println!("Unsupported or unknown opcode specified: 0x{:02X}", opcode);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::ram::Ram;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct FlagZNHC(bool, bool, bool, bool);

    impl FlagZNHC {
        fn new(state: State) -> Self {
            FlagZNHC(
                state.get_flag(Flag::Z),
                state.get_flag(Flag::N),
                state.get_flag(Flag::H),
                state.get_flag(Flag::C),
            )
        }
    }

    #[test]
    fn test_processor_ld8_r8_r8() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00]);
        state.A = 0xAA;

        let mut p = Processor::new(&mut state, &mut ram);
        p.ld8(R8::B, R8::A);
        assert_eq!(0xAA, p.state.B);
    }

    #[test]
    fn test_processor_ld8_hl() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0xAA, 0xBB]);
        state.L = 0x01;

        let mut p = Processor::new(&mut state, &mut ram);
        p.ld8(R8::B, Address::HL);
        assert_eq!(0xBB, p.state.B);

        p.ld8_hld(R8::B, Address::HL);
        assert_eq!(0xBB, p.state.B);
        assert_eq!(0x00, p.state.L);
        p.ld8_hld(R8::B, Address::HL);
        assert_eq!(0xAA, p.state.B);
        assert_eq!(0xFF, p.state.L);
    }

    #[test]
    fn test_processor_ld8_r8_d8() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);

        let mut p = Processor::new(&mut state, &mut ram);
        p.ld8(R8::B, Immediate8);
        assert_eq!(0xAA, p.state.B);
    }

    #[test]
    fn test_processor_ld8_r8_a16() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0x03, 0x00, 0xAA]);

        let mut p = Processor::new(&mut state, &mut ram);
        p.ld8(R8::B, Address::Direct); // Address will be 0x003
        assert_eq!(0xAA, p.state.B);
    }

    #[test]
    fn test_processor_ld16_r16_d16() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0xAB, 0xCD]);

        let mut p = Processor::new(&mut state, &mut ram);
        p.ld16(R16::SP, Immediate16);
        assert_eq!(0xCDAB, p.state.SP);
    }

    #[test]
    fn test_processor_ld16_r16_a16() {
        {
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00, 0x03, 0x00, 0xAB, 0xCD]);

            let mut p = Processor::new(&mut state, &mut ram);
            p.ld16(R16::SP, Address::Direct);
            assert_eq!(0xCDAB, p.state.SP);
        }
        {
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00, 0x03, 0x00, 0x00, 0x00]);
            state.SP = 0xCDAB;

            let mut p = Processor::new(&mut state, &mut ram);
            p.ld16(Address::Direct, R16::SP);
            assert_eq!(vec![0x00, 0x03, 0x00, 0xAB, 0xCD], p.bus.dump());
        }
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
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00]);
            state.A = test.a;
            state.B = test.b;

            let mut p = Processor::new(&mut state, &mut ram);
            p.add8(R8::B);
            assert_eq!(test.c, p.state.A);
            assert_eq!(test.flags, FlagZNHC::new(state));
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
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00]);
            R16::HL.write16(&mut state, &mut ram, test.a);
            R16::BC.write16(&mut state, &mut ram, test.b);

            let mut p = Processor::new(&mut state, &mut ram);
            p.add16(R16::BC);
            assert_eq!(test.c, R16::HL.read16(&mut state, &mut ram));
            assert_eq!(test.flags, FlagZNHC::new(state));
        }
    }

    #[test]
    fn test_processor_add_r16_e8() {
        struct TestCase {
            a: u16,
            b: i8,
            c: u16,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x00FF,
                b: 1,
                c: 0x0100,
                flags: FlagZNHC(false, false, true, true),
            },
            TestCase {
                a: 0x0100,
                b: -1,
                c: 0x00FF,
                flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x0010,
                b: -1,
                c: 0x000F,
                flags: FlagZNHC(false, false, false, true),
            },
            TestCase {
                a: 0x0000,
                b: -1,
                c: 0xFFFF,
                flags: FlagZNHC(false, false, false, false),
            },
        ] {
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00]);
            R16::BC.write16(&mut state, &mut ram, test.a);
            state.D = test.b as u8;

            let mut p = Processor::new(&mut state, &mut ram);
            let c = p.add_r16_e8(R16::BC, R8::D);
            assert_eq!(test.c, c);
            assert_eq!(test.flags, FlagZNHC::new(state));
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
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00]);
            state.enable_flag(Flag::C);
            state.A = test.a;
            state.B = test.b;

            let mut p = Processor::new(&mut state, &mut ram);
            p.adc8(R8::B);
            assert_eq!(test.c, p.state.A);
            assert_eq!(test.flags, FlagZNHC::new(state));
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
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00]);
            state.A = test.a;
            state.B = test.b;

            let mut p = Processor::new(&mut state, &mut ram);
            p.sub8(R8::B);
            assert_eq!(test.c, p.state.A);
            assert_eq!(test.flags, FlagZNHC::new(state));
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
            let mut state = State::new();
            let mut ram = Ram::new(vec![0x00]);
            state.enable_flag(Flag::C);
            state.A = test.a;
            state.B = test.b;

            let mut p = Processor::new(&mut state, &mut ram);
            p.sbc8(R8::B);
            assert_eq!(test.c, p.state.A);
            assert_eq!(test.flags, FlagZNHC::new(state));
        }
    }

    #[test]
    fn test_processor_inc_dec_hl() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0x0F]);
        state.L = 0x01;

        let mut p = Processor::new(&mut state, &mut ram);
        p.inc8(Address::HL);
        assert_eq!(0x10, p.bus.read8(1));
        p.dec8(Address::HL);
        assert_eq!(0x0F, p.bus.read8(1));
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
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;
                state.B = test.b;

                let mut p = Processor::new(&mut state, &mut ram);
                p.and8(R8::B);
                assert_eq!(test.and, p.state.A);
                assert_eq!(test.and_flags, FlagZNHC::new(state));
            }
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;
                state.B = test.b;

                let mut p = Processor::new(&mut state, &mut ram);
                p.or8(R8::B);
                assert_eq!(test.or, p.state.A);
                assert_eq!(test.or_flags, FlagZNHC::new(state));
            }
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;
                state.B = test.b;

                let mut p = Processor::new(&mut state, &mut ram);
                p.xor8(R8::B);
                assert_eq!(test.xor, p.state.A);
                assert_eq!(test.xor_flags, FlagZNHC::new(state));
            }
        }
    }

    #[test]
    fn test_processor_cp8() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00]);
        state.enable_flag(Flag::C);
        state.B = 0x01;

        let mut p = Processor::new(&mut state, &mut ram);
        p.cp8(R8::B);
        assert_eq!(0x00, p.state.A);
        assert_eq!(FlagZNHC(false, true, true, true), FlagZNHC::new(state));
    }

    #[test]
    fn test_processor_rotate() {
        struct TestCase {
            a: u8,
            carry: bool,
            rl: u8,
            rlc: u8,
        };
        for test in &[
            TestCase {
                a: 0b1000_1000,
                carry: true,
                rl: 0b0001_0001,
                rlc: 0b0001_0001,
            },
            TestCase {
                a: 0b1000_1000,
                carry: false,
                rl: 0b0001_0000,
                rlc: 0b0001_0001,
            },
        ] {
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;
                state.set_flag(Flag::C, test.carry);

                let mut p = Processor::new(&mut state, &mut ram);
                p.rl8(R8::A);
                assert_eq!(test.rl, p.state.A);
            }
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;
                state.set_flag(Flag::C, test.carry);

                let mut p = Processor::new(&mut state, &mut ram);
                p.rlc8(R8::A);
                assert_eq!(test.rlc, p.state.A);
            }
        }
    }

    #[test]
    fn test_processor_shift() {
        struct TestCase {
            a: u8,
            sra: u8,
            srl: u8,
            swap: u8,
        };
        for test in &[TestCase {
            a: 0b1000_0001,
            sra: 0b1100_0000,
            srl: 0b0100_0000,
            swap: 0b0001_1000,
        }] {
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;

                let mut p = Processor::new(&mut state, &mut ram);
                p.sra8(R8::A);
                assert_eq!(test.sra, p.state.A);
            }
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;

                let mut p = Processor::new(&mut state, &mut ram);
                p.srl8(R8::A);
                assert_eq!(test.srl, p.state.A);
            }
            {
                let mut state = State::new();
                let mut ram = Ram::new(vec![0x00]);
                state.A = test.a;

                let mut p = Processor::new(&mut state, &mut ram);
                p.swap8(R8::A);
                assert_eq!(test.swap, p.state.A);
            }
        }
    }

    #[test]
    fn test_processor_bit_set() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0x00]);

        let mut p = Processor::new(&mut state, &mut ram);
        p.bit8(7, R8::A);
        assert_eq!(true, p.state.get_flag(Flag::Z));
        p.set8(7, R8::A);
        assert_eq!(true, p.state.get_flag(Flag::Z));
        p.bit8(7, R8::A);
        assert_eq!(false, p.state.get_flag(Flag::Z));
        p.res8(7, R8::A);
        assert_eq!(false, p.state.get_flag(Flag::Z));
        p.bit8(7, R8::A);
        assert_eq!(true, p.state.get_flag(Flag::Z));
    }

    #[test]
    fn test_processor_push_pop() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0x00]);
        R16::SP.write16(&mut state, &mut ram, 0x0002);
        R16::BC.write16(&mut state, &mut ram, 0xABCD);

        let mut p = Processor::new(&mut state, &mut ram);
        p.push16(R16::BC);
        assert_eq!(0x0000, p.state.SP);
        assert_eq!(vec![0xCD, 0xAB], p.bus.dump());
        p.pop16(R16::DE);
        assert_eq!(0x0002, p.state.SP);
        assert_eq!(0xABCD, R16::DE.read16(&mut state, &mut ram));
    }

    #[test]
    fn test_processor_jp() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0xAB, 0xCD]);

        let mut p = Processor::new(&mut state, &mut ram);
        p.jp(Condition::F, Immediate16);
        assert_eq!(0x0000, p.state.PC);
        assert_eq!((0, 0), p.r(0, 0));
        p.jp(Condition::T, Immediate16);
        assert_eq!(0xCDAB, p.state.PC);
        assert_eq!((0, 4), p.r(0, 0));
    }

    #[test]
    fn test_processor_jr() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0xFF /* -1 */]);
        state.PC = 0x0000;

        let mut p = Processor::new(&mut state, &mut ram);
        p.jr(Condition::T, Immediate8);
        assert_eq!(0xFFFF, p.state.PC);
        assert_eq!((0, 4), p.r(0, 0));
    }

    #[test]
    fn test_processor_call_ret() {
        let mut state = State::new();
        let mut ram = Ram::new(vec![0x00, 0xAB, 0xCD, /* stack: */ 0x00, 0x00]);
        state.SP = 0x0005;
        state.PC = 0x0000;

        let mut p = Processor::new(&mut state, &mut ram);
        p.call(Condition::T, Immediate16);
        assert_eq!(0xCDAB, p.state.PC);
        assert_eq!(0x0003, p.state.SP);
        assert_eq!(vec![0x00, 0xAB, 0xCD, 0x03, 0x00], p.bus.dump());
        assert_eq!((0, 12), p.r(0, 0));
        p.ret(Condition::T);
        assert_eq!(0x0003, p.state.PC);
        assert_eq!(0x0005, p.state.SP);
        assert_eq!((0, 12), p.r(0, 0));
    }
}
