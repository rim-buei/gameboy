use super::bus::Bus;
use super::interrupt::{self, Interrupt};

pub enum Button {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

impl Button {
    fn bit(&self) -> u8 {
        use self::Button::*;

        match *self {
            Right | A => 0b0001,
            Left | B => 0b0010,
            Up | Select => 0b0100,
            Down | Start => 0b1000,
        }
    }
}

pub struct Joypad {
    p14: u8,
    p15: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad { p14: 0x00, p15: 0x00 }
    }

    pub fn press<B: Bus>(&mut self, bus: &mut B, button: Button) {
        use self::Button::*;

        match button {
            Up | Down | Left | Right => self.p14 |= button.bit(),
            A | B | Start | Select => self.p15 |= button.bit(),
        };

        interrupt::request(bus, Interrupt::Joypad);
    }

    pub fn release(&mut self, button: Button) {
        use self::Button::*;

        match button {
            Up | Down | Left | Right => self.p14 ^= button.bit(),
            A | B | Start | Select => self.p15 ^= button.bit(),
        };
    }

    pub fn transfer_state(&self) -> (u8, u8) {
        (self.p14, self.p15)
    }
}

#[cfg(test)]
mod tests {
    use super::super::mmu::Mmu;

    use super::*;

    #[test]
    fn test_joypad() {
        let mut joypad = Joypad::new();
        let mut mmu = Mmu::new();

        assert_eq!((0b0000, 0b0000), joypad.transfer_state());
        joypad.press(&mut mmu, Button::A);
        joypad.press(&mut mmu, Button::B);
        joypad.press(&mut mmu, Button::Down);
        assert_eq!((0b1000, 0b0011), joypad.transfer_state());
        joypad.release(Button::A);
        assert_eq!((0b1000, 0b0010), joypad.transfer_state());
    }
}
