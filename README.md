# Game Boy

A Game Boy emulator written in Rust.

This project does not aim to support Game Boy Color's features. Instead, it focuses on keeping the implementation simple and accurate (hopefully! :laughing:). It may also serve as a helpful reference for learning the basics of the Game Boy specification.

# Project Status

- Work in progress
- Able to run and play some Game Boy ROMs

# Screenshot Gallery

![image](https://user-images.githubusercontent.com/43806767/51608951-38171580-1f5c-11e9-8d6d-bdfa52c4387d.png "Super Mario Land") ![image](https://user-images.githubusercontent.com/43806767/51608995-54b34d80-1f5c-11e9-91f8-b69d43403a6e.png "Dr. Mario")
![image](https://user-images.githubusercontent.com/43806767/51609044-77ddfd00-1f5c-11e9-8ad8-bc5866dc0c16.png "Kirby's Dream Land") ![image](https://user-images.githubusercontent.com/43806767/51609077-89bfa000-1f5c-11e9-9d9c-b0a62e58680f.png "Pokémon Red")

# Setup

## Requirements

- Rust
- cargo-web

## Install Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup target add wasm32-unknown-unknown
```

## Install `cargo-web`

```sh
cargo install cargo-web
```

# How to Run

Clone the repository and launch `cargo-web`:
```sh
git clone https://github.com/rim-buei/gameboy.git
cd gameboy
cargo web start --bin wasm --target wasm32-unknown-unknown
```

Then open `http://localhost:8000` in your browser.

# Emulation Accuracy

Currently, this emulator passes [Blargg’s](http://gbdev.gg8.se/files/roms/blargg-gb-tests/) CPU instruction tests (`cpu_instrs`) and CPU instruction timing tests (`instr_timing`):

![image](https://user-images.githubusercontent.com/43806767/51609112-9fcd6080-1f5c-11e9-81d0-ae538398124e.png "cpu_instrs") ![image](https://user-images.githubusercontent.com/43806767/51609172-bb386b80-1f5c-11e9-8269-943c454d66c4.png "instr_timing")

Other test cases are not yet passing. :disappointed:

# Known Issues / Missing Features

The following features are not yet implemented:
- No APU support
- No save file support
- No link cable support
- MBC2 and MBC3 are not supported
