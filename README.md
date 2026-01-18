# Game Boy
A Game Boy emulator written in Rust.

This project doesn't plan to support Game Boy Color's features, but instead aims to keep the implementation simple and accurate (hopefully! :laughing:). This implementation might help you to learn basics of Game Boy spec.

# Project Status
- Still work in progress
- Able to run/play some Game Boy roms

# Screenshot Gallery
![image](https://user-images.githubusercontent.com/43806767/51608951-38171580-1f5c-11e9-8d6d-bdfa52c4387d.png "Super Mario Land") ![image](https://user-images.githubusercontent.com/43806767/51608995-54b34d80-1f5c-11e9-91f8-b69d43403a6e.png "Dr. Mario")

![image](https://user-images.githubusercontent.com/43806767/51609044-77ddfd00-1f5c-11e9-8ad8-bc5866dc0c16.png "Kirby's Dream Land") ![image](https://user-images.githubusercontent.com/43806767/51609077-89bfa000-1f5c-11e9-9d9c-b0a62e58680f.png "Pokemon Red")

# How to Setup
## Requirements
- Rust
- cargo-web

## Setup Rust
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup default nightly
$ rustup target add wasm32-unknown-unknown
```

## Setup `cargo-web`
```sh
$ cargo install cargo-web
```

# How to Run
Clone this repo and launch `cargo-web`.
```sh
$ git clone https://github.com/rim-buei/gameboy.git
$ cd gameboy
$ cargo web start --bin wasm --target wasm32-unknown-unknown
```

Then, browse `http://localhost:8000`.

# Emulation Accuracy
Currently, this emulator passes [Blargg's](http://gbdev.gg8.se/files/roms/blargg-gb-tests/) CPU instruction test cases (`cpu_instrs`) and CPU instruction timing test cases (`instr_timing`).

![image](https://user-images.githubusercontent.com/43806767/51609112-9fcd6080-1f5c-11e9-81d0-ae538398124e.png "cpu_instrs") ![image](https://user-images.githubusercontent.com/43806767/51609172-bb386b80-1f5c-11e9-8269-943c454d66c4.png "instr_timing")

Meanwhile, cannot pass the other test cases :disappointed:

# Known Issues / Missing Features
There are still known issues and missing features:
- No APU support
- No save file support
- No link cable support
- Support for MBC 2/3 are missing
