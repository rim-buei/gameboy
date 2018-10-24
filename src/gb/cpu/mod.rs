use super::ram::Ram;

pub mod registers;

use self::registers::Register16;
use self::registers::Register8;
use self::registers::Registers;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.registers.set8(Register8::A, 1);
        cpu.reset();
        assert_eq!(0, cpu.registers.get8(Register8::A));
    }
}
