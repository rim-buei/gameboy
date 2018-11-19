[![Travis-CI](https://travis-ci.org/rim-buei/gameboy.svg)](https://travis-ci.org/rim-buei/gameboy)

# Game Boy
A Game Boy emulator written in Rust

# Project Status
- Still work in progress
- Still at early-stage

# Requirements
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
