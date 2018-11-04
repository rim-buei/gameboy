use super::super::ram::Ram;

use super::io::{Reader16, Reader8, Writer16, Writer8};
use super::register::Register16 as R16;
use super::register::Register8 as R8;
use super::register::{Address, Flag, Immediate8, Registers};

pub fn exec(opcode: u8, reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    let mut i = Instruction(reg, ram);

    match opcode {
        0x00 => (1, 4),                            // [NOP] [1  4] [- - - -]
        0x01 => (0, 0),                            // TODO: [LD BC,d16] [3  12] [- - - -]
        0x02 => i.ld8(Address::BC, R8::A).r(1, 8), // [LD (BC),A] [1  8] [- - - -]
        0x03 => (0, 0),                            // TODO: [INC BC] [1  8] [- - - -]
        0x04 => i.inc8(R8::B).r(1, 4),             // [INC B] [1  4] [Z 0 H -]
        0x05 => i.dec8(R8::B).r(1, 4),             // [DEC B] [1  4] [Z 1 H -]
        0x06 => i.ld8(R8::B, Immediate8).r(2, 8),  // [LD B,d8] [2  8] [- - - -]
        0x07 => (0, 0),                            // TODO: [RLCA] [1  4] [0 0 0 C]
        0x08 => (0, 0),                            // TODO: [LD (a16),SP] [3  20] [- - - -]
        0x09 => (0, 0),                            // TODO: [ADD HL,BC] [1  8] [- 0 H C]
        0x0A => i.ld8(R8::A, Address::BC).r(1, 8), // [LD A,(BC)] [1  8] [- - - -]
        0x0B => (0, 0),                            // TODO: [DEC BC] [1  8] [- - - -]
        0x0C => i.inc8(R8::C).r(1, 4),             // [INC C] [1  4] [Z 0 H -]
        0x0D => i.dec8(R8::C).r(1, 4),             // [DEC C] [1  4] [Z 1 H -]
        0x0E => i.ld8(R8::C, Immediate8).r(2, 8),  // [LD C,d8] [2  8] [- - - -]
        0x0F => (0, 0),                            // TODO: [RRCA] [1  4] [0 0 0 C]
        0x10 => (0, 0),                            // TODO: [STOP 0] [2  4] [- - - -]
        0x11 => (0, 0),                            // TODO: [LD DE,d16] [3  12] [- - - -]
        0x12 => i.ld8(Address::DE, R8::A).r(1, 8), // [LD (DE),A] [1  8] [- - - -]
        0x13 => (0, 0),                            // TODO: [INC DE] [1  8] [- - - -]
        0x14 => i.inc8(R8::D).r(1, 4),             // [INC D] [1  4] [Z 0 H -]
        0x15 => i.dec8(R8::D).r(1, 4),             // [DEC D] [1  4] [Z 1 H -]
        0x16 => i.ld8(R8::D, Immediate8).r(2, 8),  // [LD D,d8] [2  8] [- - - -]
        0x17 => (0, 0),                            // TODO: [RLA] [1  4] [0 0 0 C]
        0x18 => (0, 0),                            // TODO: [JR r8] [2  12] [- - - -]
        0x19 => (0, 0),                            // TODO: [ADD HL,DE] [1  8] [- 0 H C]
        0x1A => i.ld8(R8::A, Address::DE).r(1, 8), // [LD A,(DE)] [1  8] [- - - -]
        0x1B => (0, 0),                            // TODO: [DEC DE] [1  8] [- - - -]
        0x1C => i.inc8(R8::E).r(1, 4),             // [INC E] [1  4] [Z 0 H -]
        0x1D => i.dec8(R8::E).r(1, 4),             // [DEC E] [1  4] [Z 1 H -]
        0x1E => i.ld8(R8::E, Immediate8).r(2, 8),  // [LD E,d8] [2  8] [- - - -]
        0x1F => (0, 0),                            // TODO: [RRA] [1  4] [0 0 0 C]
        0x20 => (0, 0),                            // TODO: [JR NZ,r8] [2  12/8] [- - - -]
        0x21 => (0, 0),                            // TODO: [LD HL,d16] [3  12] [- - - -]
        0x22 => (0, 0),                            // TODO: [LD (HL+),A] [1  8] [- - - -]
        0x23 => (0, 0),                            // TODO: [INC HL] [1  8] [- - - -]
        0x24 => i.inc8(R8::H).r(1, 4),             // [INC H] [1  4] [Z 0 H -]
        0x25 => i.dec8(R8::H).r(1, 4),             // [DEC H] [1  4] [Z 1 H -]
        0x26 => i.ld8(R8::H, Immediate8).r(2, 8),  // [LD H,d8] [2  8] [- - - -]
        0x27 => (0, 0),                            // TODO: [DAA] [1  4] [Z - 0 C]
        0x28 => (0, 0),                            // TODO: [JR Z,r8] [2  12/8] [- - - -]
        0x29 => (0, 0),                            // TODO: [ADD HL,HL] [1  8] [- 0 H C]
        0x2A => (0, 0),                            // TODO: [LD A,(HL+)] [1  8] [- - - -]
        0x2B => (0, 0),                            // TODO: [DEC HL] [1  8] [- - - -]
        0x2C => i.inc8(R8::L).r(1, 4),             // [INC L] [1  4] [Z 0 H -]
        0x2D => i.dec8(R8::L).r(1, 4),             // [DEC L] [1  4] [Z 1 H -]
        0x2E => i.ld8(R8::L, Immediate8).r(2, 8),  // [LD L,d8] [2  8] [- - - -]
        0x2F => (0, 0),                            // TODO: [CPL] [1  4] [- 1 1 -]
        0x30 => (0, 0),                            // TODO: [JR NC,r8] [2  12/8] [- - - -]
        0x31 => (0, 0),                            // TODO: [LD SP,d16] [3  12] [- - - -]
        0x32 => (0, 0),                            // TODO: [LD (HL-),A] [1  8] [- - - -]
        0x33 => (0, 0),                            // TODO: [INC SP] [1  8] [- - - -]
        0x34 => (0, 0),                            // TODO: [INC (HL)] [1  12] [Z 0 H -]
        0x35 => (0, 0),                            // TODO: [DEC (HL)] [1  12] [Z 1 H -]
        0x36 => (0, 0),                            // TODO: [LD (HL),d8] [2  12] [- - - -]
        0x37 => (0, 0),                            // TODO: [SCF] [1  4] [- 0 0 1]
        0x38 => (0, 0),                            // TODO: [JR C,r8] [2  12/8] [- - - -]
        0x39 => (0, 0),                            // TODO: [ADD HL,SP] [1  8] [- 0 H C]
        0x3A => (0, 0),                            // TODO: [LD A,(HL-)] [1  8] [- - - -]
        0x3B => (0, 0),                            // TODO: [DEC SP] [1  8] [- - - -]
        0x3C => i.inc8(R8::A).r(1, 4),             // [INC A] [1  4] [Z 0 H -]
        0x3D => i.dec8(R8::A).r(1, 4),             // [DEC A] [1  4] [Z 1 H -]
        0x3E => i.ld8(R8::A, Immediate8).r(2, 8),  // [LD A,d8] [2  8] [- - - -]
        0x3F => (0, 0),                            // TODO: [CCF] [1  4] [- 0 0 C]
        0x40 => i.ld8(R8::B, R8::B).r(1, 4),       // [LD B,B] [1  4] [- - - -]
        0x41 => i.ld8(R8::B, R8::C).r(1, 4),       // [LD B,C] [1  4] [- - - -]
        0x42 => i.ld8(R8::B, R8::D).r(1, 4),       // [LD B,D] [1  4] [- - - -]
        0x43 => i.ld8(R8::B, R8::E).r(1, 4),       // [LD B,E] [1  4] [- - - -]
        0x44 => i.ld8(R8::B, R8::H).r(1, 4),       // [LD B,H] [1  4] [- - - -]
        0x45 => i.ld8(R8::B, R8::L).r(1, 4),       // [LD B,L] [1  4] [- - - -]
        0x46 => i.ld8(R8::B, Address::HL).r(1, 8), // [LD B,(HL)] [1  8] [- - - -]
        0x47 => i.ld8(R8::B, R8::A).r(1, 4),       // [LD B,A] [1  4] [- - - -]
        0x48 => i.ld8(R8::C, R8::B).r(1, 4),       // [LD C,B] [1  4] [- - - -]
        0x49 => i.ld8(R8::C, R8::C).r(1, 4),       // [LD C,C] [1  4] [- - - -]
        0x4A => i.ld8(R8::C, R8::D).r(1, 4),       // [LD C,D] [1  4] [- - - -]
        0x4B => i.ld8(R8::C, R8::E).r(1, 4),       // [LD C,E] [1  4] [- - - -]
        0x4C => i.ld8(R8::C, R8::H).r(1, 4),       // [LD C,H] [1  4] [- - - -]
        0x4D => i.ld8(R8::C, R8::L).r(1, 4),       // [LD C,L] [1  4] [- - - -]
        0x4E => i.ld8(R8::C, Address::HL).r(1, 8), // [LD C,(HL)] [1  8] [- - - -]
        0x4F => i.ld8(R8::C, R8::A).r(1, 4),       // [LD C,A] [1  4] [- - - -]
        0x50 => i.ld8(R8::D, R8::B).r(1, 4),       // [LD D,B] [1  4] [- - - -]
        0x51 => i.ld8(R8::D, R8::C).r(1, 4),       // [LD D,C] [1  4] [- - - -]
        0x52 => i.ld8(R8::D, R8::D).r(1, 4),       // [LD D,D] [1  4] [- - - -]
        0x53 => i.ld8(R8::D, R8::E).r(1, 4),       // [LD D,E] [1  4] [- - - -]
        0x54 => i.ld8(R8::D, R8::H).r(1, 4),       // [LD D,H] [1  4] [- - - -]
        0x55 => i.ld8(R8::D, R8::L).r(1, 4),       // [LD D,L] [1  4] [- - - -]
        0x56 => i.ld8(R8::D, Address::HL).r(1, 8), // [LD D,(HL)] [1  8] [- - - -]
        0x57 => i.ld8(R8::D, R8::A).r(1, 4),       // [LD D,A] [1  4] [- - - -]
        0x58 => i.ld8(R8::E, R8::B).r(1, 4),       // [LD E,B] [1  4] [- - - -]
        0x59 => i.ld8(R8::E, R8::C).r(1, 4),       // [LD E,C] [1  4] [- - - -]
        0x5A => i.ld8(R8::E, R8::D).r(1, 4),       // [LD E,D] [1  4] [- - - -]
        0x5B => i.ld8(R8::E, R8::E).r(1, 4),       // [LD E,E] [1  4] [- - - -]
        0x5C => i.ld8(R8::E, R8::H).r(1, 4),       // [LD E,H] [1  4] [- - - -]
        0x5D => i.ld8(R8::E, R8::L).r(1, 4),       // [LD E,L] [1  4] [- - - -]
        0x5E => i.ld8(R8::E, Address::HL).r(1, 8), // [LD E,(HL)] [1  8] [- - - -]
        0x5F => i.ld8(R8::E, R8::A).r(1, 4),       // [LD E,A] [1  4] [- - - -]
        0x60 => i.ld8(R8::H, R8::B).r(1, 4),       // [LD H,B] [1  4] [- - - -]
        0x61 => i.ld8(R8::H, R8::C).r(1, 4),       // [LD H,C] [1  4] [- - - -]
        0x62 => i.ld8(R8::H, R8::D).r(1, 4),       // [LD H,D] [1  4] [- - - -]
        0x63 => i.ld8(R8::H, R8::E).r(1, 4),       // [LD H,E] [1  4] [- - - -]
        0x64 => i.ld8(R8::H, R8::H).r(1, 4),       // [LD H,H] [1  4] [- - - -]
        0x65 => i.ld8(R8::H, R8::L).r(1, 4),       // [LD H,L] [1  4] [- - - -]
        0x66 => i.ld8(R8::H, Address::HL).r(1, 8), // [LD H,(HL)] [1  8] [- - - -]
        0x67 => i.ld8(R8::H, R8::A).r(1, 4),       // [LD H,A] [1  4] [- - - -]
        0x68 => i.ld8(R8::L, R8::B).r(1, 4),       // [LD L,B] [1  4] [- - - -]
        0x69 => i.ld8(R8::L, R8::C).r(1, 4),       // [LD L,C] [1  4] [- - - -]
        0x6A => i.ld8(R8::L, R8::D).r(1, 4),       // [LD L,D] [1  4] [- - - -]
        0x6B => i.ld8(R8::L, R8::E).r(1, 4),       // [LD L,E] [1  4] [- - - -]
        0x6C => i.ld8(R8::L, R8::H).r(1, 4),       // [LD L,H] [1  4] [- - - -]
        0x6D => i.ld8(R8::L, R8::L).r(1, 4),       // [LD L,L] [1  4] [- - - -]
        0x6E => i.ld8(R8::L, Address::HL).r(1, 8), // [LD L,(HL)] [1  8] [- - - -]
        0x6F => i.ld8(R8::L, R8::A).r(1, 4),       // [LD L,A] [1  4] [- - - -]
        0x70 => i.ld8(Address::HL, R8::B).r(1, 8), // [LD (HL),B] [1  8] [- - - -]
        0x71 => i.ld8(Address::HL, R8::C).r(1, 8), // [LD (HL),C] [1  8] [- - - -]
        0x72 => i.ld8(Address::HL, R8::D).r(1, 8), // [LD (HL),D] [1  8] [- - - -]
        0x73 => i.ld8(Address::HL, R8::E).r(1, 8), // [LD (HL),E] [1  8] [- - - -]
        0x74 => i.ld8(Address::HL, R8::H).r(1, 8), // [LD (HL),H] [1  8] [- - - -]
        0x75 => i.ld8(Address::HL, R8::L).r(1, 8), // [LD (HL),L] [1  8] [- - - -]
        0x76 => (0, 0),                            // TODO: [HALT] [1  4] [- - - -]
        0x77 => i.ld8(Address::HL, R8::A).r(1, 8), // [LD (HL),A] [1  8] [- - - -]
        0x78 => i.ld8(R8::A, R8::B).r(1, 4),       // [LD A,B] [1  4] [- - - -]
        0x79 => i.ld8(R8::A, R8::C).r(1, 4),       // [LD A,C] [1  4] [- - - -]
        0x7A => i.ld8(R8::A, R8::D).r(1, 4),       // [LD A,D] [1  4] [- - - -]
        0x7B => i.ld8(R8::A, R8::E).r(1, 4),       // [LD A,E] [1  4] [- - - -]
        0x7C => i.ld8(R8::A, R8::H).r(1, 4),       // [LD A,H] [1  4] [- - - -]
        0x7D => i.ld8(R8::A, R8::L).r(1, 4),       // [LD A,L] [1  4] [- - - -]
        0x7E => i.ld8(R8::A, Address::HL).r(1, 8), // [LD A,(HL)] [1  8] [- - - -]
        0x7F => i.ld8(R8::A, R8::A).r(1, 4),       // [LD A,A] [1  4] [- - - -]
        0x80 => i.add8(R8::B).r(1, 4),             // [ADD A,B] [1  4] [Z 0 H C]
        0x81 => i.add8(R8::C).r(1, 4),             // [ADD A,C] [1  4] [Z 0 H C]
        0x82 => i.add8(R8::D).r(1, 4),             // [ADD A,D] [1  4] [Z 0 H C]
        0x83 => i.add8(R8::E).r(1, 4),             // [ADD A,E] [1  4] [Z 0 H C]
        0x84 => i.add8(R8::H).r(1, 4),             // [ADD A,H] [1  4] [Z 0 H C]
        0x85 => i.add8(R8::L).r(1, 4),             // [ADD A,L] [1  4] [Z 0 H C]
        0x86 => i.add8(Address::HL).r(1, 8),       // [ADD A,(HL)] [1  8] [Z 0 H C]
        0x87 => i.add8(R8::A).r(1, 4),             // [ADD A,A] [1  4] [Z 0 H C]
        0x88 => i.adc8(R8::B).r(1, 4),             // [ADC A,B] [1  4] [Z 0 H C]
        0x89 => i.adc8(R8::C).r(1, 4),             // [ADC A,C] [1  4] [Z 0 H C]
        0x8A => i.adc8(R8::D).r(1, 4),             // [ADC A,D] [1  4] [Z 0 H C]
        0x8B => i.adc8(R8::E).r(1, 4),             // [ADC A,E] [1  4] [Z 0 H C]
        0x8C => i.adc8(R8::H).r(1, 4),             // [ADC A,H] [1  4] [Z 0 H C]
        0x8D => i.adc8(R8::L).r(1, 4),             // [ADC A,L] [1  4] [Z 0 H C]
        0x8E => i.adc8(Address::HL).r(1, 8),       // [ADC A,(HL)] [1  8] [Z 0 H C]
        0x8F => i.adc8(R8::A).r(1, 4),             // [ADC A,A] [1  4] [Z 0 H C]
        0x90 => i.sub8(R8::B).r(1, 4),             // [SUB A,B] [1  4] [Z 1 H C]
        0x91 => i.sub8(R8::C).r(1, 4),             // [SUB A,C] [1  4] [Z 1 H C]
        0x92 => i.sub8(R8::D).r(1, 4),             // [SUB A,D] [1  4] [Z 1 H C]
        0x93 => i.sub8(R8::E).r(1, 4),             // [SUB A,E] [1  4] [Z 1 H C]
        0x94 => i.sub8(R8::H).r(1, 4),             // [SUB A,H] [1  4] [Z 1 H C]
        0x95 => i.sub8(R8::L).r(1, 4),             // [SUB A,L] [1  4] [Z 1 H C]
        0x96 => i.sub8(Address::HL).r(1, 8),       // [SUB A,(HL)] [1  8] [Z 1 H C]
        0x97 => i.sub8(R8::A).r(1, 4),             // [SUB A,A] [1  4] [Z 1 H C]
        0x98 => i.sbc8(R8::B).r(1, 4),             // [SBC A,B] [1  4] [Z 1 H C]
        0x99 => i.sbc8(R8::C).r(1, 4),             // [SBC A,C] [1  4] [Z 1 H C]
        0x9A => i.sbc8(R8::D).r(1, 4),             // [SBC A,D] [1  4] [Z 1 H C]
        0x9B => i.sbc8(R8::E).r(1, 4),             // [SBC A,E] [1  4] [Z 1 H C]
        0x9C => i.sbc8(R8::H).r(1, 4),             // [SBC A,H] [1  4] [Z 1 H C]
        0x9D => i.sbc8(R8::L).r(1, 4),             // [SBC A,L] [1  4] [Z 1 H C]
        0x9E => i.sbc8(Address::HL).r(1, 8),       // [SBC A,(HL)] [1  8] [Z 1 H C]
        0x9F => i.sbc8(R8::A).r(1, 4),             // [SBC A,A] [1  4] [Z 1 H C]
        0xA0 => i.and8(R8::B).r(1, 4),             // [AND B] [1  4] [Z 0 1 0]
        0xA1 => i.and8(R8::C).r(1, 4),             // [AND C] [1  4] [Z 0 1 0]
        0xA2 => i.and8(R8::D).r(1, 4),             // [AND D] [1  4] [Z 0 1 0]
        0xA3 => i.and8(R8::E).r(1, 4),             // [AND E] [1  4] [Z 0 1 0]
        0xA4 => i.and8(R8::H).r(1, 4),             // [AND H] [1  4] [Z 0 1 0]
        0xA5 => i.and8(R8::L).r(1, 4),             // [AND L] [1  4] [Z 0 1 0]
        0xA6 => i.and8(Address::HL).r(1, 8),       // [AND (HL)] [1  8] [Z 0 1 0]
        0xA7 => i.and8(R8::A).r(1, 4),             // [AND A] [1  4] [Z 0 1 0]
        0xA8 => i.xor8(R8::B).r(1, 4),             // [XOR B] [1  4] [Z 0 0 0]
        0xA9 => i.xor8(R8::C).r(1, 4),             // [XOR C] [1  4] [Z 0 0 0]
        0xAA => i.xor8(R8::D).r(1, 4),             // [XOR D] [1  4] [Z 0 0 0]
        0xAB => i.xor8(R8::E).r(1, 4),             // [XOR E] [1  4] [Z 0 0 0]
        0xAC => i.xor8(R8::H).r(1, 4),             // [XOR H] [1  4] [Z 0 0 0]
        0xAD => i.xor8(R8::L).r(1, 4),             // [XOR L] [1  4] [Z 0 0 0]
        0xAE => i.xor8(Address::HL).r(1, 8),       // [XOR (HL)] [1  8] [Z 0 0 0]
        0xAF => i.xor8(R8::A).r(1, 4),             // [XOR A] [1  4] [Z 0 0 0]
        0xB0 => i.or8(R8::B).r(1, 4),              // [OR B] [1  4] [Z 0 0 0]
        0xB1 => i.or8(R8::C).r(1, 4),              // [OR C] [1  4] [Z 0 0 0]
        0xB2 => i.or8(R8::D).r(1, 4),              // [OR D] [1  4] [Z 0 0 0]
        0xB3 => i.or8(R8::E).r(1, 4),              // [OR E] [1  4] [Z 0 0 0]
        0xB4 => i.or8(R8::H).r(1, 4),              // [OR H] [1  4] [Z 0 0 0]
        0xB5 => i.or8(R8::L).r(1, 4),              // [OR L] [1  4] [Z 0 0 0]
        0xB6 => i.or8(Address::HL).r(1, 8),        // [OR (HL)] [1  8] [Z 0 0 0]
        0xB7 => i.or8(R8::A).r(1, 4),              // [OR A] [1  4] [Z 0 0 0]
        0xB8 => (0, 0),                            // TODO: [CP B] [1  4] [Z 1 H C]
        0xB9 => (0, 0),                            // TODO: [CP C] [1  4] [Z 1 H C]
        0xBA => (0, 0),                            // TODO: [CP D] [1  4] [Z 1 H C]
        0xBB => (0, 0),                            // TODO: [CP E] [1  4] [Z 1 H C]
        0xBC => (0, 0),                            // TODO: [CP H] [1  4] [Z 1 H C]
        0xBD => (0, 0),                            // TODO: [CP L] [1  4] [Z 1 H C]
        0xBE => (0, 0),                            // TODO: [CP (HL)] [1  8] [Z 1 H C]
        0xBF => (0, 0),                            // TODO: [CP A] [1  4] [Z 1 H C]
        0xC0 => (0, 0),                            // TODO: [RET NZ] [1  20/8] [- - - -]
        0xC1 => (0, 0),                            // TODO: [POP BC] [1  12] [- - - -]
        0xC2 => (0, 0),                            // TODO: [JP NZ,a16] [3  16/12] [- - - -]
        0xC3 => (0, 0),                            // TODO: [JP a16] [3  16] [- - - -]
        0xC4 => (0, 0),                            // TODO: [CALL NZ,a16] [3  24/12] [- - - -]
        0xC5 => (0, 0),                            // TODO: [PUSH BC] [1  16] [- - - -]
        0xC6 => i.add8(Immediate8).r(2, 8),        // [ADD A,d8] [2  8] [Z 0 H C]
        0xC7 => (0, 0),                            // TODO: [RST 00H] [1  16] [- - - -]
        0xC8 => (0, 0),                            // TODO: [RET Z] [1  20/8] [- - - -]
        0xC9 => (0, 0),                            // TODO: [RET] [1  16] [- - - -]
        0xCA => (0, 0),                            // TODO: [JP Z,a16] [3  16/12] [- - - -]
        0xCB => i.undefined(opcode).r(1, 0),       // [PREFIX CB] [1  4] [- - - -]
        0xCC => (0, 0),                            // TODO: [CALL Z,a16] [3  24/12] [- - - -]
        0xCD => (0, 0),                            // TODO: [CALL a16] [3  24] [- - - -]
        0xCE => i.adc8(Immediate8).r(2, 8),        // [ADC A,d8] [2  8] [Z 0 H C]
        0xCF => (0, 0),                            // TODO: [RST 08H] [1  16] [- - - -]
        0xD0 => (0, 0),                            // TODO: [RET NC] [1  20/8] [- - - -]
        0xD1 => (0, 0),                            // TODO: [POP DE] [1  12] [- - - -]
        0xD2 => (0, 0),                            // TODO: [JP NC,a16] [3  16/12] [- - - -]
        0xD3 => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xD4 => (0, 0),                            // TODO: [CALL NC,a16] [3  24/12] [- - - -]
        0xD5 => (0, 0),                            // TODO: [PUSH DE] [1  16] [- - - -]
        0xD6 => i.sub8(Immediate8).r(2, 8),        // [SUB A,d8] [2  8] [Z 1 H C]
        0xD7 => (0, 0),                            // TODO: [RST 10H] [1  16] [- - - -]
        0xD8 => (0, 0),                            // TODO: [RET C] [1  20/8] [- - - -]
        0xD9 => (0, 0),                            // TODO: [RETI] [1  16] [- - - -]
        0xDA => (0, 0),                            // TODO: [JP C,a16] [3  16/12] [- - - -]
        0xDB => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xDC => (0, 0),                            // TODO: [CALL C,a16] [3  24/12] [- - - -]
        0xDD => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xDE => i.sbc8(Immediate8).r(2, 8),        // [SBC A,d8] [2  8] [Z 1 H C]
        0xDF => (0, 0),                            // TODO: [RST 18H] [1  16] [- - - -]
        0xE0 => (0, 0),                            // TODO: [LDH (a8),A] [2  12] [- - - -]
        0xE1 => (0, 0),                            // TODO: [POP HL] [1  12] [- - - -]
        0xE2 => (0, 0),                            // TODO: [LD (C),A] [2  8] [- - - -]
        0xE3 => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xE4 => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xE5 => (0, 0),                            // TODO: [PUSH HL] [1  16] [- - - -]
        0xE6 => i.and8(Immediate8).r(2, 8),        // [AND d8] [2  8] [Z 0 1 0]
        0xE7 => (0, 0),                            // TODO: [RST 20H] [1  16] [- - - -]
        0xE8 => (0, 0),                            // TODO: [ADD SP,r8] [2  16] [0 0 H C]
        0xE9 => (0, 0),                            // TODO: [JP (HL)] [1  4] [- - - -]
        0xEA => (0, 0),                            // TODO: [LD (a16),A] [3  16] [- - - -]
        0xEB => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xEC => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xED => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xEE => i.xor8(Immediate8).r(2, 8),        // [XOR d8] [2  8] [Z 0 0 0]
        0xEF => (0, 0),                            // TODO: [RST 28H] [1  16] [- - - -]
        0xF0 => (0, 0),                            // TODO: [LDH A,(a8)] [2  12] [- - - -]
        0xF1 => (0, 0),                            // TODO: [POP AF] [1  12] [Z N H C]
        0xF2 => (0, 0),                            // TODO: [LD A,(C)] [2  8] [- - - -]
        0xF3 => (0, 0),                            // TODO: [DI] [1  4] [- - - -]
        0xF4 => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xF5 => (0, 0),                            // TODO: [PUSH AF] [1  16] [- - - -]
        0xF6 => i.or8(Immediate8).r(2, 8),         // [OR d8] [2  8] [Z 0 0 0]
        0xF7 => (0, 0),                            // TODO: [RST 30H] [1  16] [- - - -]
        0xF8 => (0, 0),                            // TODO: [LD HL,SP+r8] [2  12] [0 0 H C]
        0xF9 => (0, 0),                            // TODO: [LD SP,HL] [1  8] [- - - -]
        0xFA => (0, 0),                            // TODO: [LD A,(a16)] [3  16] [- - - -]
        0xFB => (0, 0),                            // TODO: [EI] [1  4] [- - - -]
        0xFC => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xFD => i.undefined(opcode).r(1, 0),       // [Undefined]
        0xFE => (0, 0),                            // TODO: [CP d8] [2  8] [Z 1 H C]
        0xFF => (0, 0),                            // TODO: [RST 38H] [1  16] [- - - -]
        _ => i.undefined(opcode).r(1, 0),
    }
}

pub fn exec_ex(opcode: u8, reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    let mut i = Instruction(reg, ram);

    match opcode {
        0x00 => (0, 0), // TODO: [RLC B] [2  8] [Z 0 0 C]
        0x01 => (0, 0), // TODO: [RLC C] [2  8] [Z 0 0 C]
        0x02 => (0, 0), // TODO: [RLC D] [2  8] [Z 0 0 C]
        0x03 => (0, 0), // TODO: [RLC E] [2  8] [Z 0 0 C]
        0x04 => (0, 0), // TODO: [RLC H] [2  8] [Z 0 0 C]
        0x05 => (0, 0), // TODO: [RLC L] [2  8] [Z 0 0 C]
        0x06 => (0, 0), // TODO: [RLC (HL)] [2  16] [Z 0 0 C]
        0x07 => (0, 0), // TODO: [RLC A] [2  8] [Z 0 0 C]
        0x08 => (0, 0), // TODO: [RRC B] [2  8] [Z 0 0 C]
        0x09 => (0, 0), // TODO: [RRC C] [2  8] [Z 0 0 C]
        0x0A => (0, 0), // TODO: [RRC D] [2  8] [Z 0 0 C]
        0x0B => (0, 0), // TODO: [RRC E] [2  8] [Z 0 0 C]
        0x0C => (0, 0), // TODO: [RRC H] [2  8] [Z 0 0 C]
        0x0D => (0, 0), // TODO: [RRC L] [2  8] [Z 0 0 C]
        0x0E => (0, 0), // TODO: [RRC (HL)] [2  16] [Z 0 0 C]
        0x0F => (0, 0), // TODO: [RRC A] [2  8] [Z 0 0 C]
        0x10 => (0, 0), // TODO: [RL B] [2  8] [Z 0 0 C]
        0x11 => (0, 0), // TODO: [RL C] [2  8] [Z 0 0 C]
        0x12 => (0, 0), // TODO: [RL D] [2  8] [Z 0 0 C]
        0x13 => (0, 0), // TODO: [RL E] [2  8] [Z 0 0 C]
        0x14 => (0, 0), // TODO: [RL H] [2  8] [Z 0 0 C]
        0x15 => (0, 0), // TODO: [RL L] [2  8] [Z 0 0 C]
        0x16 => (0, 0), // TODO: [RL (HL)] [2  16] [Z 0 0 C]
        0x17 => (0, 0), // TODO: [RL A] [2  8] [Z 0 0 C]
        0x18 => (0, 0), // TODO: [RR B] [2  8] [Z 0 0 C]
        0x19 => (0, 0), // TODO: [RR C] [2  8] [Z 0 0 C]
        0x1A => (0, 0), // TODO: [RR D] [2  8] [Z 0 0 C]
        0x1B => (0, 0), // TODO: [RR E] [2  8] [Z 0 0 C]
        0x1C => (0, 0), // TODO: [RR H] [2  8] [Z 0 0 C]
        0x1D => (0, 0), // TODO: [RR L] [2  8] [Z 0 0 C]
        0x1E => (0, 0), // TODO: [RR (HL)] [2  16] [Z 0 0 C]
        0x1F => (0, 0), // TODO: [RR A] [2  8] [Z 0 0 C]
        0x20 => (0, 0), // TODO: [SLA B] [2  8] [Z 0 0 C]
        0x21 => (0, 0), // TODO: [SLA C] [2  8] [Z 0 0 C]
        0x22 => (0, 0), // TODO: [SLA D] [2  8] [Z 0 0 C]
        0x23 => (0, 0), // TODO: [SLA E] [2  8] [Z 0 0 C]
        0x24 => (0, 0), // TODO: [SLA H] [2  8] [Z 0 0 C]
        0x25 => (0, 0), // TODO: [SLA L] [2  8] [Z 0 0 C]
        0x26 => (0, 0), // TODO: [SLA (HL)] [2  16] [Z 0 0 C]
        0x27 => (0, 0), // TODO: [SLA A] [2  8] [Z 0 0 C]
        0x28 => (0, 0), // TODO: [SRA B] [2  8] [Z 0 0 0]
        0x29 => (0, 0), // TODO: [SRA C] [2  8] [Z 0 0 0]
        0x2A => (0, 0), // TODO: [SRA D] [2  8] [Z 0 0 0]
        0x2B => (0, 0), // TODO: [SRA E] [2  8] [Z 0 0 0]
        0x2C => (0, 0), // TODO: [SRA H] [2  8] [Z 0 0 0]
        0x2D => (0, 0), // TODO: [SRA L] [2  8] [Z 0 0 0]
        0x2E => (0, 0), // TODO: [SRA (HL)] [2  16] [Z 0 0 0]
        0x2F => (0, 0), // TODO: [SRA A] [2  8] [Z 0 0 0]
        0x30 => (0, 0), // TODO: [SWAP B] [2  8] [Z 0 0 0]
        0x31 => (0, 0), // TODO: [SWAP C] [2  8] [Z 0 0 0]
        0x32 => (0, 0), // TODO: [SWAP D] [2  8] [Z 0 0 0]
        0x33 => (0, 0), // TODO: [SWAP E] [2  8] [Z 0 0 0]
        0x34 => (0, 0), // TODO: [SWAP H] [2  8] [Z 0 0 0]
        0x35 => (0, 0), // TODO: [SWAP L] [2  8] [Z 0 0 0]
        0x36 => (0, 0), // TODO: [SWAP (HL)] [2  16] [Z 0 0 0]
        0x37 => (0, 0), // TODO: [SWAP A] [2  8] [Z 0 0 0]
        0x38 => (0, 0), // TODO: [SRL B] [2  8] [Z 0 0 C]
        0x39 => (0, 0), // TODO: [SRL C] [2  8] [Z 0 0 C]
        0x3A => (0, 0), // TODO: [SRL D] [2  8] [Z 0 0 C]
        0x3B => (0, 0), // TODO: [SRL E] [2  8] [Z 0 0 C]
        0x3C => (0, 0), // TODO: [SRL H] [2  8] [Z 0 0 C]
        0x3D => (0, 0), // TODO: [SRL L] [2  8] [Z 0 0 C]
        0x3E => (0, 0), // TODO: [SRL (HL)] [2  16] [Z 0 0 C]
        0x3F => (0, 0), // TODO: [SRL A] [2  8] [Z 0 0 C]
        0x40 => (0, 0), // TODO: [BIT 0,B] [2  8] [Z 0 1 -]
        0x41 => (0, 0), // TODO: [BIT 0,C] [2  8] [Z 0 1 -]
        0x42 => (0, 0), // TODO: [BIT 0,D] [2  8] [Z 0 1 -]
        0x43 => (0, 0), // TODO: [BIT 0,E] [2  8] [Z 0 1 -]
        0x44 => (0, 0), // TODO: [BIT 0,H] [2  8] [Z 0 1 -]
        0x45 => (0, 0), // TODO: [BIT 0,L] [2  8] [Z 0 1 -]
        0x46 => (0, 0), // TODO: [BIT 0,(HL)] [2  16] [Z 0 1 -]
        0x47 => (0, 0), // TODO: [BIT 0,A] [2  8] [Z 0 1 -]
        0x48 => (0, 0), // TODO: [BIT 1,B] [2  8] [Z 0 1 -]
        0x49 => (0, 0), // TODO: [BIT 1,C] [2  8] [Z 0 1 -]
        0x4A => (0, 0), // TODO: [BIT 1,D] [2  8] [Z 0 1 -]
        0x4B => (0, 0), // TODO: [BIT 1,E] [2  8] [Z 0 1 -]
        0x4C => (0, 0), // TODO: [BIT 1,H] [2  8] [Z 0 1 -]
        0x4D => (0, 0), // TODO: [BIT 1,L] [2  8] [Z 0 1 -]
        0x4E => (0, 0), // TODO: [BIT 1,(HL)] [2  16] [Z 0 1 -]
        0x4F => (0, 0), // TODO: [BIT 1,A] [2  8] [Z 0 1 -]
        0x50 => (0, 0), // TODO: [BIT 2,B] [2  8] [Z 0 1 -]
        0x51 => (0, 0), // TODO: [BIT 2,C] [2  8] [Z 0 1 -]
        0x52 => (0, 0), // TODO: [BIT 2,D] [2  8] [Z 0 1 -]
        0x53 => (0, 0), // TODO: [BIT 2,E] [2  8] [Z 0 1 -]
        0x54 => (0, 0), // TODO: [BIT 2,H] [2  8] [Z 0 1 -]
        0x55 => (0, 0), // TODO: [BIT 2,L] [2  8] [Z 0 1 -]
        0x56 => (0, 0), // TODO: [BIT 2,(HL)] [2  16] [Z 0 1 -]
        0x57 => (0, 0), // TODO: [BIT 2,A] [2  8] [Z 0 1 -]
        0x58 => (0, 0), // TODO: [BIT 3,B] [2  8] [Z 0 1 -]
        0x59 => (0, 0), // TODO: [BIT 3,C] [2  8] [Z 0 1 -]
        0x5A => (0, 0), // TODO: [BIT 3,D] [2  8] [Z 0 1 -]
        0x5B => (0, 0), // TODO: [BIT 3,E] [2  8] [Z 0 1 -]
        0x5C => (0, 0), // TODO: [BIT 3,H] [2  8] [Z 0 1 -]
        0x5D => (0, 0), // TODO: [BIT 3,L] [2  8] [Z 0 1 -]
        0x5E => (0, 0), // TODO: [BIT 3,(HL)] [2  16] [Z 0 1 -]
        0x5F => (0, 0), // TODO: [BIT 3,A] [2  8] [Z 0 1 -]
        0x60 => (0, 0), // TODO: [BIT 4,B] [2  8] [Z 0 1 -]
        0x61 => (0, 0), // TODO: [BIT 4,C] [2  8] [Z 0 1 -]
        0x62 => (0, 0), // TODO: [BIT 4,D] [2  8] [Z 0 1 -]
        0x63 => (0, 0), // TODO: [BIT 4,E] [2  8] [Z 0 1 -]
        0x64 => (0, 0), // TODO: [BIT 4,H] [2  8] [Z 0 1 -]
        0x65 => (0, 0), // TODO: [BIT 4,L] [2  8] [Z 0 1 -]
        0x66 => (0, 0), // TODO: [BIT 4,(HL)] [2  16] [Z 0 1 -]
        0x67 => (0, 0), // TODO: [BIT 4,A] [2  8] [Z 0 1 -]
        0x68 => (0, 0), // TODO: [BIT 5,B] [2  8] [Z 0 1 -]
        0x69 => (0, 0), // TODO: [BIT 5,C] [2  8] [Z 0 1 -]
        0x6A => (0, 0), // TODO: [BIT 5,D] [2  8] [Z 0 1 -]
        0x6B => (0, 0), // TODO: [BIT 5,E] [2  8] [Z 0 1 -]
        0x6C => (0, 0), // TODO: [BIT 5,H] [2  8] [Z 0 1 -]
        0x6D => (0, 0), // TODO: [BIT 5,L] [2  8] [Z 0 1 -]
        0x6E => (0, 0), // TODO: [BIT 5,(HL)] [2  16] [Z 0 1 -]
        0x6F => (0, 0), // TODO: [BIT 5,A] [2  8] [Z 0 1 -]
        0x70 => (0, 0), // TODO: [BIT 6,B] [2  8] [Z 0 1 -]
        0x71 => (0, 0), // TODO: [BIT 6,C] [2  8] [Z 0 1 -]
        0x72 => (0, 0), // TODO: [BIT 6,D] [2  8] [Z 0 1 -]
        0x73 => (0, 0), // TODO: [BIT 6,E] [2  8] [Z 0 1 -]
        0x74 => (0, 0), // TODO: [BIT 6,H] [2  8] [Z 0 1 -]
        0x75 => (0, 0), // TODO: [BIT 6,L] [2  8] [Z 0 1 -]
        0x76 => (0, 0), // TODO: [BIT 6,(HL)] [2  16] [Z 0 1 -]
        0x77 => (0, 0), // TODO: [BIT 6,A] [2  8] [Z 0 1 -]
        0x78 => (0, 0), // TODO: [BIT 7,B] [2  8] [Z 0 1 -]
        0x79 => (0, 0), // TODO: [BIT 7,C] [2  8] [Z 0 1 -]
        0x7A => (0, 0), // TODO: [BIT 7,D] [2  8] [Z 0 1 -]
        0x7B => (0, 0), // TODO: [BIT 7,E] [2  8] [Z 0 1 -]
        0x7C => (0, 0), // TODO: [BIT 7,H] [2  8] [Z 0 1 -]
        0x7D => (0, 0), // TODO: [BIT 7,L] [2  8] [Z 0 1 -]
        0x7E => (0, 0), // TODO: [BIT 7,(HL)] [2  16] [Z 0 1 -]
        0x7F => (0, 0), // TODO: [BIT 7,A] [2  8] [Z 0 1 -]
        0x80 => (0, 0), // TODO: [RES 0,B] [2  8] [- - - -]
        0x81 => (0, 0), // TODO: [RES 0,C] [2  8] [- - - -]
        0x82 => (0, 0), // TODO: [RES 0,D] [2  8] [- - - -]
        0x83 => (0, 0), // TODO: [RES 0,E] [2  8] [- - - -]
        0x84 => (0, 0), // TODO: [RES 0,H] [2  8] [- - - -]
        0x85 => (0, 0), // TODO: [RES 0,L] [2  8] [- - - -]
        0x86 => (0, 0), // TODO: [RES 0,(HL)] [2  16] [- - - -]
        0x87 => (0, 0), // TODO: [RES 0,A] [2  8] [- - - -]
        0x88 => (0, 0), // TODO: [RES 1,B] [2  8] [- - - -]
        0x89 => (0, 0), // TODO: [RES 1,C] [2  8] [- - - -]
        0x8A => (0, 0), // TODO: [RES 1,D] [2  8] [- - - -]
        0x8B => (0, 0), // TODO: [RES 1,E] [2  8] [- - - -]
        0x8C => (0, 0), // TODO: [RES 1,H] [2  8] [- - - -]
        0x8D => (0, 0), // TODO: [RES 1,L] [2  8] [- - - -]
        0x8E => (0, 0), // TODO: [RES 1,(HL)] [2  16] [- - - -]
        0x8F => (0, 0), // TODO: [RES 1,A] [2  8] [- - - -]
        0x90 => (0, 0), // TODO: [RES 2,B] [2  8] [- - - -]
        0x91 => (0, 0), // TODO: [RES 2,C] [2  8] [- - - -]
        0x92 => (0, 0), // TODO: [RES 2,D] [2  8] [- - - -]
        0x93 => (0, 0), // TODO: [RES 2,E] [2  8] [- - - -]
        0x94 => (0, 0), // TODO: [RES 2,H] [2  8] [- - - -]
        0x95 => (0, 0), // TODO: [RES 2,L] [2  8] [- - - -]
        0x96 => (0, 0), // TODO: [RES 2,(HL)] [2  16] [- - - -]
        0x97 => (0, 0), // TODO: [RES 2,A] [2  8] [- - - -]
        0x98 => (0, 0), // TODO: [RES 3,B] [2  8] [- - - -]
        0x99 => (0, 0), // TODO: [RES 3,C] [2  8] [- - - -]
        0x9A => (0, 0), // TODO: [RES 3,D] [2  8] [- - - -]
        0x9B => (0, 0), // TODO: [RES 3,E] [2  8] [- - - -]
        0x9C => (0, 0), // TODO: [RES 3,H] [2  8] [- - - -]
        0x9D => (0, 0), // TODO: [RES 3,L] [2  8] [- - - -]
        0x9E => (0, 0), // TODO: [RES 3,(HL)] [2  16] [- - - -]
        0x9F => (0, 0), // TODO: [RES 3,A] [2  8] [- - - -]
        0xA0 => (0, 0), // TODO: [RES 4,B] [2  8] [- - - -]
        0xA1 => (0, 0), // TODO: [RES 4,C] [2  8] [- - - -]
        0xA2 => (0, 0), // TODO: [RES 4,D] [2  8] [- - - -]
        0xA3 => (0, 0), // TODO: [RES 4,E] [2  8] [- - - -]
        0xA4 => (0, 0), // TODO: [RES 4,H] [2  8] [- - - -]
        0xA5 => (0, 0), // TODO: [RES 4,L] [2  8] [- - - -]
        0xA6 => (0, 0), // TODO: [RES 4,(HL)] [2  16] [- - - -]
        0xA7 => (0, 0), // TODO: [RES 4,A] [2  8] [- - - -]
        0xA8 => (0, 0), // TODO: [RES 5,B] [2  8] [- - - -]
        0xA9 => (0, 0), // TODO: [RES 5,C] [2  8] [- - - -]
        0xAA => (0, 0), // TODO: [RES 5,D] [2  8] [- - - -]
        0xAB => (0, 0), // TODO: [RES 5,E] [2  8] [- - - -]
        0xAC => (0, 0), // TODO: [RES 5,H] [2  8] [- - - -]
        0xAD => (0, 0), // TODO: [RES 5,L] [2  8] [- - - -]
        0xAE => (0, 0), // TODO: [RES 5,(HL)] [2  16] [- - - -]
        0xAF => (0, 0), // TODO: [RES 5,A] [2  8] [- - - -]
        0xB0 => (0, 0), // TODO: [RES 6,B] [2  8] [- - - -]
        0xB1 => (0, 0), // TODO: [RES 6,C] [2  8] [- - - -]
        0xB2 => (0, 0), // TODO: [RES 6,D] [2  8] [- - - -]
        0xB3 => (0, 0), // TODO: [RES 6,E] [2  8] [- - - -]
        0xB4 => (0, 0), // TODO: [RES 6,H] [2  8] [- - - -]
        0xB5 => (0, 0), // TODO: [RES 6,L] [2  8] [- - - -]
        0xB6 => (0, 0), // TODO: [RES 6,(HL)] [2  16] [- - - -]
        0xB7 => (0, 0), // TODO: [RES 6,A] [2  8] [- - - -]
        0xB8 => (0, 0), // TODO: [RES 7,B] [2  8] [- - - -]
        0xB9 => (0, 0), // TODO: [RES 7,C] [2  8] [- - - -]
        0xBA => (0, 0), // TODO: [RES 7,D] [2  8] [- - - -]
        0xBB => (0, 0), // TODO: [RES 7,E] [2  8] [- - - -]
        0xBC => (0, 0), // TODO: [RES 7,H] [2  8] [- - - -]
        0xBD => (0, 0), // TODO: [RES 7,L] [2  8] [- - - -]
        0xBE => (0, 0), // TODO: [RES 7,(HL)] [2  16] [- - - -]
        0xBF => (0, 0), // TODO: [RES 7,A] [2  8] [- - - -]
        0xC0 => (0, 0), // TODO: [SET 0,B] [2  8] [- - - -]
        0xC1 => (0, 0), // TODO: [SET 0,C] [2  8] [- - - -]
        0xC2 => (0, 0), // TODO: [SET 0,D] [2  8] [- - - -]
        0xC3 => (0, 0), // TODO: [SET 0,E] [2  8] [- - - -]
        0xC4 => (0, 0), // TODO: [SET 0,H] [2  8] [- - - -]
        0xC5 => (0, 0), // TODO: [SET 0,L] [2  8] [- - - -]
        0xC6 => (0, 0), // TODO: [SET 0,(HL)] [2  16] [- - - -]
        0xC7 => (0, 0), // TODO: [SET 0,A] [2  8] [- - - -]
        0xC8 => (0, 0), // TODO: [SET 1,B] [2  8] [- - - -]
        0xC9 => (0, 0), // TODO: [SET 1,C] [2  8] [- - - -]
        0xCA => (0, 0), // TODO: [SET 1,D] [2  8] [- - - -]
        0xCB => (0, 0), // TODO: [SET 1,E] [2  8] [- - - -]
        0xCC => (0, 0), // TODO: [SET 1,H] [2  8] [- - - -]
        0xCD => (0, 0), // TODO: [SET 1,L] [2  8] [- - - -]
        0xCE => (0, 0), // TODO: [SET 1,(HL)] [2  16] [- - - -]
        0xCF => (0, 0), // TODO: [SET 1,A] [2  8] [- - - -]
        0xD0 => (0, 0), // TODO: [SET 2,B] [2  8] [- - - -]
        0xD1 => (0, 0), // TODO: [SET 2,C] [2  8] [- - - -]
        0xD2 => (0, 0), // TODO: [SET 2,D] [2  8] [- - - -]
        0xD3 => (0, 0), // TODO: [SET 2,E] [2  8] [- - - -]
        0xD4 => (0, 0), // TODO: [SET 2,H] [2  8] [- - - -]
        0xD5 => (0, 0), // TODO: [SET 2,L] [2  8] [- - - -]
        0xD6 => (0, 0), // TODO: [SET 2,(HL)] [2  16] [- - - -]
        0xD7 => (0, 0), // TODO: [SET 2,A] [2  8] [- - - -]
        0xD8 => (0, 0), // TODO: [SET 3,B] [2  8] [- - - -]
        0xD9 => (0, 0), // TODO: [SET 3,C] [2  8] [- - - -]
        0xDA => (0, 0), // TODO: [SET 3,D] [2  8] [- - - -]
        0xDB => (0, 0), // TODO: [SET 3,E] [2  8] [- - - -]
        0xDC => (0, 0), // TODO: [SET 3,H] [2  8] [- - - -]
        0xDD => (0, 0), // TODO: [SET 3,L] [2  8] [- - - -]
        0xDE => (0, 0), // TODO: [SET 3,(HL)] [2  16] [- - - -]
        0xDF => (0, 0), // TODO: [SET 3,A] [2  8] [- - - -]
        0xE0 => (0, 0), // TODO: [SET 4,B] [2  8] [- - - -]
        0xE1 => (0, 0), // TODO: [SET 4,C] [2  8] [- - - -]
        0xE2 => (0, 0), // TODO: [SET 4,D] [2  8] [- - - -]
        0xE3 => (0, 0), // TODO: [SET 4,E] [2  8] [- - - -]
        0xE4 => (0, 0), // TODO: [SET 4,H] [2  8] [- - - -]
        0xE5 => (0, 0), // TODO: [SET 4,L] [2  8] [- - - -]
        0xE6 => (0, 0), // TODO: [SET 4,(HL)] [2  16] [- - - -]
        0xE7 => (0, 0), // TODO: [SET 4,A] [2  8] [- - - -]
        0xE8 => (0, 0), // TODO: [SET 5,B] [2  8] [- - - -]
        0xE9 => (0, 0), // TODO: [SET 5,C] [2  8] [- - - -]
        0xEA => (0, 0), // TODO: [SET 5,D] [2  8] [- - - -]
        0xEB => (0, 0), // TODO: [SET 5,E] [2  8] [- - - -]
        0xEC => (0, 0), // TODO: [SET 5,H] [2  8] [- - - -]
        0xED => (0, 0), // TODO: [SET 5,L] [2  8] [- - - -]
        0xEE => (0, 0), // TODO: [SET 5,(HL)] [2  16] [- - - -]
        0xEF => (0, 0), // TODO: [SET 5,A] [2  8] [- - - -]
        0xF0 => (0, 0), // TODO: [SET 6,B] [2  8] [- - - -]
        0xF1 => (0, 0), // TODO: [SET 6,C] [2  8] [- - - -]
        0xF2 => (0, 0), // TODO: [SET 6,D] [2  8] [- - - -]
        0xF3 => (0, 0), // TODO: [SET 6,E] [2  8] [- - - -]
        0xF4 => (0, 0), // TODO: [SET 6,H] [2  8] [- - - -]
        0xF5 => (0, 0), // TODO: [SET 6,L] [2  8] [- - - -]
        0xF6 => (0, 0), // TODO: [SET 6,(HL)] [2  16] [- - - -]
        0xF7 => (0, 0), // TODO: [SET 6,A] [2  8] [- - - -]
        0xF8 => (0, 0), // TODO: [SET 7,B] [2  8] [- - - -]
        0xF9 => (0, 0), // TODO: [SET 7,C] [2  8] [- - - -]
        0xFA => (0, 0), // TODO: [SET 7,D] [2  8] [- - - -]
        0xFB => (0, 0), // TODO: [SET 7,E] [2  8] [- - - -]
        0xFC => (0, 0), // TODO: [SET 7,H] [2  8] [- - - -]
        0xFD => (0, 0), // TODO: [SET 7,L] [2  8] [- - - -]
        0xFE => (0, 0), // TODO: [SET 7,(HL)] [2  16] [- - - -]
        0xFF => (0, 0), // TODO: [SET 7,A] [2  8] [- - - -]
        _ => i.undefined(opcode).r(1, 0),
    }
}

struct Instruction<'a>(&'a mut Registers, &'a mut Ram);

impl<'a> Instruction<'a> {
    fn r(&mut self, opsize: u8, cycle: u8) -> (u8, u8) {
        (opsize, cycle)
    }

    fn ld8<R: Reader8, W: Writer8>(&mut self, lhs: W, rhs: R) -> &mut Self {
        let v = rhs.read8(self.0, self.1);
        lhs.write8(self.0, self.1, v);
        self
    }

    fn add8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as u16;
        let b = rhs.read8(self.0, self.1) as u16;
        let c = a + b;
        let hc = ((a & 0x0F) + (b & 0x0F)) > 0x0F;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c > 0xFF);

        self.0.A = c as u8;
        self
    }

    fn adc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as u16;
        let b = rhs.read8(self.0, self.1) as u16;
        let carry = if self.0.get_flag(Flag::C) { 1 } else { 0 } as u16;
        let c = a + b + carry;
        let hc = ((a & 0x0F) + (b & 0x0F) + carry) > 0x0F;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c > 0xFF);

        self.0.A = c as u8;
        self
    }

    fn inc8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.0, self.1).wrapping_add(1);

        self.0.set_flag(Flag::Z, v == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.set_flag(Flag::H, (v & 0x0F) == 0x00);

        rw.write8(self.0, self.1, v);
        self
    }

    fn sub8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as i16;
        let b = rhs.read8(self.0, self.1) as i16;
        let c = a - b;
        let hc = ((a & 0x0F) - (b & 0x0F)) < 0;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.enable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c < 0);

        self.0.A = c as u8;
        self
    }

    fn sbc8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let a = self.0.A as i16;
        let b = rhs.read8(self.0, self.1) as i16;
        let carry = if self.0.get_flag(Flag::C) { 1 } else { 0 } as i16;
        let c = a - b - carry;
        let hc = ((a & 0x0F) - (b & 0x0F) - carry) < 0;

        self.0.set_flag(Flag::Z, (c & 0xFF) == 0x00);
        self.0.enable_flag(Flag::N);
        self.0.set_flag(Flag::H, hc);
        self.0.set_flag(Flag::C, c < 0);

        self.0.A = c as u8;
        self
    }

    fn dec8<RW: Reader8 + Writer8>(&mut self, rw: RW) -> &mut Self {
        let v = rw.read8(self.0, self.1).wrapping_sub(1);

        self.0.set_flag(Flag::Z, v == 0x00);
        self.0.enable_flag(Flag::N);
        self.0.set_flag(Flag::H, (v & 0x0F) == 0x0F);

        rw.write8(self.0, self.1, v);
        self
    }

    fn and8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.0.A & rhs.read8(self.0, self.1);

        self.0.set_flag(Flag::Z, c == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.enable_flag(Flag::H);
        self.0.disable_flag(Flag::C);

        self.0.A = c;
        self
    }

    fn or8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.0.A | rhs.read8(self.0, self.1);

        self.0.set_flag(Flag::Z, c == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.disable_flag(Flag::H);
        self.0.disable_flag(Flag::C);

        self.0.A = c;
        self
    }

    fn xor8<R: Reader8>(&mut self, rhs: R) -> &mut Self {
        let c = self.0.A ^ rhs.read8(self.0, self.1);

        self.0.set_flag(Flag::Z, c == 0x00);
        self.0.disable_flag(Flag::N);
        self.0.disable_flag(Flag::H);
        self.0.disable_flag(Flag::C);

        self.0.A = c;
        self
    }

    fn undefined(&mut self, opcode: u8) -> &mut Self {
        println!("Unsupported or unknown opcode specified: 0x{:02X}", opcode);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct FlagZNHC(bool, bool, bool, bool);

    impl FlagZNHC {
        fn new(reg: Registers) -> Self {
            FlagZNHC(
                reg.get_flag(Flag::Z),
                reg.get_flag(Flag::N),
                reg.get_flag(Flag::H),
                reg.get_flag(Flag::C),
            )
        }
    }

    #[test]
    fn test_ld8_r8_r8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00]);
        reg.A = 0xAA;

        let mut i = Instruction(&mut reg, &mut ram);
        i.ld8(R8::B, R8::A);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_ld8_r8_hl() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        reg.L = 0x01;

        let mut i = Instruction(&mut reg, &mut ram);
        i.ld8(R8::B, Address::HL);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_ld8_r8_d8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);

        let mut i = Instruction(&mut reg, &mut ram);
        i.ld8(R8::B, Immediate8);
        assert_eq!(0xAA, reg.B);
    }

    #[test]
    fn test_add8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x00,
                b: 0x01,
                c: 0x01,
                flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x0F,
                b: 0x01,
                c: 0x10,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0xF0,
                b: 0x10,
                c: 0x00,
                flags: FlagZNHC(true, false, false, true),
            },
            TestCase {
                a: 0xFF,
                b: 0x01,
                c: 0x00,
                flags: FlagZNHC(true, false, true, true),
            },
        ] {
            let mut reg = Registers::new();
            let mut ram = Ram::new(vec![0x00]);
            reg.A = test.a;
            reg.B = test.b;

            let mut i = Instruction(&mut reg, &mut ram);
            i.add8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_adc8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x00,
                b: 0x01,
                c: 0x02,
                flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x0E,
                b: 0x01,
                c: 0x10,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0x00,
                b: 0x0F,
                c: 0x10,
                flags: FlagZNHC(false, false, true, false),
            },
            TestCase {
                a: 0xF0,
                b: 0x10,
                c: 0x01,
                flags: FlagZNHC(false, false, false, true),
            },
            TestCase {
                a: 0x00,
                b: 0xFF,
                c: 0x00,
                flags: FlagZNHC(true, false, true, true),
            },
        ] {
            let mut reg = Registers::new();
            let mut ram = Ram::new(vec![0x00]);
            reg.enable_flag(Flag::C);
            reg.A = test.a;
            reg.B = test.b;

            let mut i = Instruction(&mut reg, &mut ram);
            i.adc8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_sub8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x02,
                b: 0x01,
                c: 0x01,
                flags: FlagZNHC(false, true, false, false),
            },
            TestCase {
                a: 0x10,
                b: 0x01,
                c: 0x0F,
                flags: FlagZNHC(false, true, true, false),
            },
            TestCase {
                a: 0x00,
                b: 0x10,
                c: 0xF0,
                flags: FlagZNHC(false, true, false, true),
            },
            TestCase {
                a: 0x00,
                b: 0x01,
                c: 0xFF,
                flags: FlagZNHC(false, true, true, true),
            },
        ] {
            let mut reg = Registers::new();
            let mut ram = Ram::new(vec![0x00]);
            reg.A = test.a;
            reg.B = test.b;

            let mut i = Instruction(&mut reg, &mut ram);
            i.sub8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_sbc8() {
        struct TestCase {
            a: u8,
            b: u8,
            c: u8,
            flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x03,
                b: 0x01,
                c: 0x01,
                flags: FlagZNHC(false, true, false, false),
            },
            TestCase {
                a: 0x11,
                b: 0x01,
                c: 0x0F,
                flags: FlagZNHC(false, true, true, false),
            },
            TestCase {
                a: 0x10,
                b: 0x01,
                c: 0x0E,
                flags: FlagZNHC(false, true, true, false),
            },
            TestCase {
                a: 0x00,
                b: 0x0F,
                c: 0xF0,
                flags: FlagZNHC(false, true, true, true),
            },
            TestCase {
                a: 0x00,
                b: 0xFF,
                c: 0x00,
                flags: FlagZNHC(true, true, true, true),
            },
        ] {
            let mut reg = Registers::new();
            let mut ram = Ram::new(vec![0x00]);
            reg.enable_flag(Flag::C);
            reg.A = test.a;
            reg.B = test.b;

            let mut i = Instruction(&mut reg, &mut ram);
            i.sbc8(R8::B);
            assert_eq!(test.c, reg.A);
            assert_eq!(test.flags, FlagZNHC::new(reg));
        }
    }

    #[test]
    fn test_logical8() {
        struct TestCase {
            a: u8,
            b: u8,
            and: u8,
            and_flags: FlagZNHC,
            or: u8,
            or_flags: FlagZNHC,
            xor: u8,
            xor_flags: FlagZNHC,
        };
        for test in &[
            TestCase {
                a: 0x01,
                b: 0x01,
                and: 0x01,
                and_flags: FlagZNHC(false, false, true, false),
                or: 0x01,
                or_flags: FlagZNHC(false, false, false, false),
                xor: 0x00,
                xor_flags: FlagZNHC(true, false, false, false),
            },
            TestCase {
                a: 0x01,
                b: 0x00,
                and: 0x00,
                and_flags: FlagZNHC(true, false, true, false),
                or: 0x01,
                or_flags: FlagZNHC(false, false, false, false),
                xor: 0x01,
                xor_flags: FlagZNHC(false, false, false, false),
            },
            TestCase {
                a: 0x00,
                b: 0x00,
                and: 0x00,
                and_flags: FlagZNHC(true, false, true, false),
                or: 0x00,
                or_flags: FlagZNHC(true, false, false, false),
                xor: 0x00,
                xor_flags: FlagZNHC(true, false, false, false),
            },
        ] {
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut i = Instruction(&mut reg, &mut ram);
                i.and8(R8::B);
                assert_eq!(test.and, reg.A);
                assert_eq!(test.and_flags, FlagZNHC::new(reg));
            }
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut i = Instruction(&mut reg, &mut ram);
                i.or8(R8::B);
                assert_eq!(test.or, reg.A);
                assert_eq!(test.or_flags, FlagZNHC::new(reg));
            }
            {
                let mut reg = Registers::new();
                let mut ram = Ram::new(vec![0x00]);
                reg.A = test.a;
                reg.B = test.b;

                let mut i = Instruction(&mut reg, &mut ram);
                i.xor8(R8::B);
                assert_eq!(test.xor, reg.A);
                assert_eq!(test.xor_flags, FlagZNHC::new(reg));
            }
        }
    }
}
