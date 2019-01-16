// NOTE: This file is just for debugging
mod gb;

use self::gb::cartridge::Cartridge;
use self::gb::cpu::Cpu;
use self::gb::mmu::Mmu;
use self::gb::ppu::Ppu;
use self::gb::timer::Timer;
use std::io::Write;

fn main() {
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();
    let mut mmu = Mmu::new();
    let mut timer = Timer::new();
    cpu.simulate_bootloader();
    mmu.simulate_bootloader();

    match load_rom_from_first_arg() {
        Ok(cart) => mmu.load_cartridge(cart),
        Err(err) => {
            writeln!(std::io::stderr(), "{}", err.to_string()).unwrap();
            std::process::exit(1);
        }
    }

    loop {
        let cycle = cpu.step(&mut mmu);
        ppu.step(&mut mmu, cycle);
        timer.step(&mut mmu, cycle);

        // TODO: Insert debug code here
    }
}

fn load_rom_from_first_arg() -> Result<Cartridge, String> {
    use std::fs::File;
    use std::io::{BufReader, Read};

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return Err("You must specify a ROM file".to_owned());
    };

    let f = File::open(&args[1]).unwrap();
    let mut buf = BufReader::new(f);
    let mut rom = vec![];
    buf.read_to_end(&mut rom).unwrap();

    Ok(Cartridge::new(rom))
}
