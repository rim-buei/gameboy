use super::bus::Bus;

#[derive(Debug, PartialEq)]
pub enum Interrupt {
    VBlank = 1 << 0,
    LCDStat = 1 << 1,
    Timer = 1 << 2,
    Serial = 1 << 3,
    Joypad = 1 << 4,

    None = 1 << 7,
}

const IE_REG_ADDR: u16 = 0xFFFF;
const IF_REG_ADDR: u16 = 0xFF0F;

pub fn request<B: Bus>(bus: &mut B, int: Interrupt) {
    if int == Interrupt::None {
        panic!("this interrupt cannot be requested")
    }

    let if_reg = bus.read8(IF_REG_ADDR);
    bus.write8(IF_REG_ADDR, if_reg | int as u8);
}

fn discard<B: Bus>(bus: &mut B, int: Interrupt) {
    if int == Interrupt::None {
        panic!("this interrupt cannot be discarded")
    }

    let if_reg = bus.read8(IF_REG_ADDR);
    bus.write8(IF_REG_ADDR, if_reg & !(int as u8));
}

pub fn receive<B: Bus>(bus: &mut B) -> Interrupt {
    use self::Interrupt::*;

    let ie_reg = bus.read8(IE_REG_ADDR);
    let if_reg = bus.read8(IF_REG_ADDR);
    let v = ie_reg & if_reg;

    if (v & VBlank as u8) != 0 {
        discard(bus, VBlank);
        VBlank
    } else if (v & LCDStat as u8) != 0 {
        discard(bus, LCDStat);
        LCDStat
    } else if (v & Timer as u8) != 0 {
        discard(bus, Timer);
        Timer
    } else if (v & Serial as u8) != 0 {
        discard(bus, Serial);
        Serial
    } else if (v & Joypad as u8) != 0 {
        discard(bus, Joypad);
        Joypad
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::mmu::Mmu;

    use super::*;

    #[test]
    fn test_interrupt() {
        let mut mmu = Mmu::new();

        assert_eq!(Interrupt::None, receive(&mut mmu));

        request(&mut mmu, Interrupt::Timer);
        assert_eq!(Interrupt::None, receive(&mut mmu));

        mmu.write8(IE_REG_ADDR, 0xFF);
        assert_eq!(Interrupt::Timer, receive(&mut mmu));
        // The interrupt will be automatically discarded

        assert_eq!(Interrupt::None, receive(&mut mmu));
    }

    #[test]
    #[should_panic]
    fn test_interrupt_panic() {
        let mut mmu = Mmu::new();
        request(&mut mmu, Interrupt::None);
    }
}
