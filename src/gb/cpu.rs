use super::ram::Ram;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
        }
    }

    pub fn step(&mut self, ram: &mut Ram) {
        // TODO: Implementation
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new()
    }
}

#[allow(non_snake_case)]
struct Registers {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,

    F: u8,

    SP: u16,
    PC: u16,
}

#[derive(Debug, Copy, Clone)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

#[derive(Debug, Copy, Clone)]
enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Registers {
    fn new() -> Self {
        Registers {
            A: 0x00,
            B: 0x00,
            C: 0x00,
            D: 0x00,
            E: 0x00,
            H: 0x00,
            L: 0x00,

            F: 0x00,

            PC: 0x0000,
            SP: 0x0000,
        }
    }

    fn get8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.A,
            Register8::B => self.B,
            Register8::C => self.C,
            Register8::D => self.D,
            Register8::E => self.E,
            Register8::H => self.H,
            Register8::L => self.L,
            Register8::F => self.F,
        }
    }

    fn set8(&mut self, reg: Register8, v: u8) {
        match reg {
            Register8::A => self.A = v,
            Register8::B => self.B = v,
            Register8::C => self.C = v,
            Register8::D => self.D = v,
            Register8::E => self.E = v,
            Register8::H => self.H = v,
            Register8::L => self.L = v,
            Register8::F => self.F = v,
        }
    }

    fn get16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => ((self.A as u16) << 8) + (self.F as u16),
            Register16::BC => ((self.B as u16) << 8) + (self.C as u16),
            Register16::DE => ((self.D as u16) << 8) + (self.E as u16),
            Register16::HL => ((self.H as u16) << 8) + (self.L as u16),
            Register16::SP => self.SP,
            Register16::PC => self.PC,
        }
    }

    fn set16(&mut self, reg: Register16, v: u16) {
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
            Register16::SP => self.SP = v,
            Register16::PC => self.PC = v,
        }
    }

    fn add8(&mut self, reg: Register8, n: u8) {
        let v = self.get8(reg) as u32 + n as u32;
        if v > 0xFF {
            // TODO: Overflow
        }

        self.set8(reg, (v & 0xFF) as u8)
    }

    fn sub8(&mut self, reg: Register8, n: u8) {
        let mut v = self.get8(reg) as i32 - n as i32;
        if v < 0 {
            // TODO: Underflow
            v += 0x100
        }

        self.set8(reg, (v & 0xFF) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.registers.A = 1;
        cpu.reset();
        assert_eq!(0, cpu.registers.A);
    }

    #[test]
    fn registers_add() {
        let mut reg = Registers::new();

        reg.set8(Register8::A, 0);
        reg.add8(Register8::A, 1);
        assert_eq!(1, reg.A);

        reg.set8(Register8::A, 0xFF);
        reg.add8(Register8::A, 1);
        assert_eq!(0, reg.A);
    }

    #[test]
    fn registers_sub() {
        let mut reg = Registers::new();

        reg.set8(Register8::A, 0);
        reg.sub8(Register8::A, 1);
        assert_eq!(0xFF, reg.A);

        reg.set8(Register8::A, 2);
        reg.sub8(Register8::A, 1);
        assert_eq!(1, reg.A);
    }
}
