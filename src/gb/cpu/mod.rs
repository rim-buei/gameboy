pub mod registers;

use super::ram::Ram;

use self::registers::Register8;
use self::registers::Registers;

pub fn step(reg: &mut Registers, ram: &mut Ram) {
    // TODO: Implementation
}

pub fn reset(reg: &mut Registers) {
    *reg = Registers::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        let mut reg = Registers::new();
        reg.set8(Register8::A, 1);
        reset(&mut reg);
        assert_eq!(0, reg.get8(Register8::A));
    }
}
