[![Travis-CI](https://travis-ci.org/rim-buei/gameboy.svg)](https://travis-ci.org/rim-buei/gameboy)

# Game Boy
A Game Boy emulator written in Rust.

This project doesn't plan to support Game Boy Color's features, but instead aims to keep the implementation simple and accurate (hopefully! :laughing:). This implementation might help you to learn basics of Game Boy spec.

# Project Status
- Still work in progress
- Able to run/play some Game Boy roms

# Screenshot Gallery
![image](https://user-images.githubusercontent.com/43806767/51184897-f8777a80-1918-11e9-81dd-d15cb33dd901.png "Super Mario Land") ![image](https://user-images.githubusercontent.com/43806767/51185063-83587500-1919-11e9-9ff4-684cb7fcb8e7.png "Dr. Mario")

![image](https://user-images.githubusercontent.com/43806767/51185395-58225580-191a-11e9-8955-fd50b3956668.png "Pokemon Red") ![image](https://user-images.githubusercontent.com/43806767/51185596-f57d8980-191a-11e9-9ed9-f70a598a8682.png "Kirby's Dream Land")

# How to Setup
## Requirements
- Rust
- cargo-web

## Setup Rust
```sh
$ curl https://sh.rustup.rs -sSf | sh
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

![image](https://user-images.githubusercontent.com/43806767/50381598-5aecaa80-06ce-11e9-8415-6df6c5a5e1fe.png) ![image](https://user-images.githubusercontent.com/43806767/50383488-f8a89f80-06f7-11e9-8371-15995756fbe7.png)

Meanwhile, cannot pass the other test cases :disappointed:

# Known Issues / Missing Features
There are still known issues and missing features:
- No APU support
- No save file support
- No link cable support
- Support for MBC 2/3/5 are missing
