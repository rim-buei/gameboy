pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
        }
    }

    fn reset(&mut self) {
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

impl Registers {
    fn new() -> Registers {
        Registers {
            A: 0x00,
            B: 0x00,
            C: 0x00,
            D: 0x00,
            E: 0x00,
            H: 0x00,
            L: 0x00,

            F: 0x00,

            SP: 0x0000,
            PC: 0x0000,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.registers.A = 1;
        cpu.reset();
        assert_eq!(0, cpu.registers.A);
    }
}
