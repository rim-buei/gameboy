#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct State {
    // 8-bit registers
    pub A: u8,
    pub F: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,

    // 16-bit registers
    pub SP: u16,
    pub PC: u16,

    // Interrupt master enable flag
    pub IME: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Z, // Zero
    N, // Subtract
    H, // Half Carry
    C, // Carry
}

impl State {
    pub fn new() -> Self {
        State {
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

            IME: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_flag() {
        let mut state = State::new();

        state.enable_flag(Flag::Z);
        assert_eq!(0b10000000, state.F);
        state.enable_flag(Flag::N);
        assert_eq!(0b11000000, state.F);
        state.disable_flag(Flag::Z);
        assert_eq!(0b01000000, state.F);
    }
}
