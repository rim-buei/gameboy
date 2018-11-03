use super::super::ram::Ram;

use super::register::Register16 as R16;
use super::register::Register8 as R8;
use super::register::Registers;

pub fn exec(opcode: u8, reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    match opcode {
        0x00 => (1, 4),                               // [NOP] [1  4] [- - - -]
        0x01 => (0, 0),                               // TODO: [LD BC,d16] [3  12] [- - - -]
        0x02 => ld_addr_r8(reg, ram, R16::BC, R8::A), // [LD (BC),A] [1  8] [- - - -]
        0x03 => (0, 0),                               // TODO: [INC BC] [1  8] [- - - -]
        0x04 => inc_r8(reg, R8::B),                   // [INC B] [1  4] [Z 0 H -]
        0x05 => dec_r8(reg, R8::B),                   // [DEC B] [1  4] [Z 1 H -]
        0x06 => ld_r8_n8(reg, ram, R8::B),            // [LD B,d8] [2  8] [- - - -]
        0x07 => (0, 0),                               // TODO: [RLCA] [1  4] [0 0 0 C]
        0x08 => (0, 0),                               // TODO: [LD (a16),SP] [3  20] [- - - -]
        0x09 => (0, 0),                               // TODO: [ADD HL,BC] [1  8] [- 0 H C]
        0x0A => ld_r8_addr(reg, ram, R8::A, R16::BC), // [LD A,(BC)] [1  8] [- - - -]
        0x0B => (0, 0),                               // TODO: [DEC BC] [1  8] [- - - -]
        0x0C => inc_r8(reg, R8::C),                   // [INC C] [1  4] [Z 0 H -]
        0x0D => dec_r8(reg, R8::C),                   // [DEC C] [1  4] [Z 1 H -]
        0x0E => ld_r8_n8(reg, ram, R8::C),            // [LD C,d8] [2  8] [- - - -]
        0x0F => (0, 0),                               // TODO: [RRCA] [1  4] [0 0 0 C]
        0x10 => (0, 0),                               // TODO: [STOP 0] [2  4] [- - - -]
        0x11 => (0, 0),                               // TODO: [LD DE,d16] [3  12] [- - - -]
        0x12 => ld_addr_r8(reg, ram, R16::DE, R8::A), // [LD (DE),A] [1  8] [- - - -]
        0x13 => (0, 0),                               // TODO: [INC DE] [1  8] [- - - -]
        0x14 => inc_r8(reg, R8::D),                   // [INC D] [1  4] [Z 0 H -]
        0x15 => dec_r8(reg, R8::D),                   // [DEC D] [1  4] [Z 1 H -]
        0x16 => ld_r8_n8(reg, ram, R8::D),            // [LD D,d8] [2  8] [- - - -]
        0x17 => (0, 0),                               // TODO: [RLA] [1  4] [0 0 0 C]
        0x18 => (0, 0),                               // TODO: [JR r8] [2  12] [- - - -]
        0x19 => (0, 0),                               // TODO: [ADD HL,DE] [1  8] [- 0 H C]
        0x1A => ld_r8_addr(reg, ram, R8::A, R16::DE), // [LD A,(DE)] [1  8] [- - - -]
        0x1B => (0, 0),                               // TODO: [DEC DE] [1  8] [- - - -]
        0x1C => inc_r8(reg, R8::E),                   // [INC E] [1  4] [Z 0 H -]
        0x1D => dec_r8(reg, R8::E),                   // [DEC E] [1  4] [Z 1 H -]
        0x1E => ld_r8_n8(reg, ram, R8::E),            // [LD E,d8] [2  8] [- - - -]
        0x1F => (0, 0),                               // TODO: [RRA] [1  4] [0 0 0 C]
        0x20 => (0, 0),                               // TODO: [JR NZ,r8] [2  12/8] [- - - -]
        0x21 => (0, 0),                               // TODO: [LD HL,d16] [3  12] [- - - -]
        0x22 => (0, 0),                               // TODO: [LD (HL+),A] [1  8] [- - - -]
        0x23 => (0, 0),                               // TODO: [INC HL] [1  8] [- - - -]
        0x24 => inc_r8(reg, R8::H),                   // [INC H] [1  4] [Z 0 H -]
        0x25 => dec_r8(reg, R8::H),                   // [DEC H] [1  4] [Z 1 H -]
        0x26 => ld_r8_n8(reg, ram, R8::H),            // [LD H,d8] [2  8] [- - - -]
        0x27 => (0, 0),                               // TODO: [DAA] [1  4] [Z - 0 C]
        0x28 => (0, 0),                               // TODO: [JR Z,r8] [2  12/8] [- - - -]
        0x29 => (0, 0),                               // TODO: [ADD HL,HL] [1  8] [- 0 H C]
        0x2A => (0, 0),                               // TODO: [LD A,(HL+)] [1  8] [- - - -]
        0x2B => (0, 0),                               // TODO: [DEC HL] [1  8] [- - - -]
        0x2C => inc_r8(reg, R8::L),                   // [INC L] [1  4] [Z 0 H -]
        0x2D => dec_r8(reg, R8::L),                   // [DEC L] [1  4] [Z 1 H -]
        0x2E => ld_r8_n8(reg, ram, R8::L),            // [LD L,d8] [2  8] [- - - -]
        0x2F => (0, 0),                               // TODO: [CPL] [1  4] [- 1 1 -]
        0x30 => (0, 0),                               // TODO: [JR NC,r8] [2  12/8] [- - - -]
        0x31 => (0, 0),                               // TODO: [LD SP,d16] [3  12] [- - - -]
        0x32 => (0, 0),                               // TODO: [LD (HL-),A] [1  8] [- - - -]
        0x33 => (0, 0),                               // TODO: [INC SP] [1  8] [- - - -]
        0x34 => (0, 0),                               // TODO: [INC (HL)] [1  12] [Z 0 H -]
        0x35 => (0, 0),                               // TODO: [DEC (HL)] [1  12] [Z 1 H -]
        0x36 => (0, 0),                               // TODO: [LD (HL),d8] [2  12] [- - - -]
        0x37 => (0, 0),                               // TODO: [SCF] [1  4] [- 0 0 1]
        0x38 => (0, 0),                               // TODO: [JR C,r8] [2  12/8] [- - - -]
        0x39 => (0, 0),                               // TODO: [ADD HL,SP] [1  8] [- 0 H C]
        0x3A => (0, 0),                               // TODO: [LD A,(HL-)] [1  8] [- - - -]
        0x3B => (0, 0),                               // TODO: [DEC SP] [1  8] [- - - -]
        0x3C => inc_r8(reg, R8::A),                   // [INC A] [1  4] [Z 0 H -]
        0x3D => dec_r8(reg, R8::A),                   // [DEC A] [1  4] [Z 1 H -]
        0x3E => ld_r8_n8(reg, ram, R8::A),            // [LD A,d8] [2  8] [- - - -]
        0x3F => (0, 0),                               // TODO: [CCF] [1  4] [- 0 0 C]
        0x40 => ld_r8_r8(reg, R8::B, R8::B),          // [LD B,B] [1  4] [- - - -]
        0x41 => ld_r8_r8(reg, R8::B, R8::C),          // [LD B,C] [1  4] [- - - -]
        0x42 => ld_r8_r8(reg, R8::B, R8::D),          // [LD B,D] [1  4] [- - - -]
        0x43 => ld_r8_r8(reg, R8::B, R8::E),          // [LD B,E] [1  4] [- - - -]
        0x44 => ld_r8_r8(reg, R8::B, R8::H),          // [LD B,H] [1  4] [- - - -]
        0x45 => ld_r8_r8(reg, R8::B, R8::L),          // [LD B,L] [1  4] [- - - -]
        0x46 => ld_r8_addr(reg, ram, R8::B, R16::HL), // [LD B,(HL)] [1  8] [- - - -]
        0x47 => ld_r8_r8(reg, R8::B, R8::A),          // [LD B,A] [1  4] [- - - -]
        0x48 => ld_r8_r8(reg, R8::C, R8::B),          // [LD C,B] [1  4] [- - - -]
        0x49 => ld_r8_r8(reg, R8::C, R8::C),          // [LD C,C] [1  4] [- - - -]
        0x4A => ld_r8_r8(reg, R8::C, R8::D),          // [LD C,D] [1  4] [- - - -]
        0x4B => ld_r8_r8(reg, R8::C, R8::E),          // [LD C,E] [1  4] [- - - -]
        0x4C => ld_r8_r8(reg, R8::C, R8::H),          // [LD C,H] [1  4] [- - - -]
        0x4D => ld_r8_r8(reg, R8::C, R8::L),          // [LD C,L] [1  4] [- - - -]
        0x4E => ld_r8_addr(reg, ram, R8::C, R16::HL), // [LD C,(HL)] [1  8] [- - - -]
        0x4F => ld_r8_r8(reg, R8::C, R8::A),          // [LD C,A] [1  4] [- - - -]
        0x50 => ld_r8_r8(reg, R8::D, R8::B),          // [LD D,B] [1  4] [- - - -]
        0x51 => ld_r8_r8(reg, R8::D, R8::C),          // [LD D,C] [1  4] [- - - -]
        0x52 => ld_r8_r8(reg, R8::D, R8::D),          // [LD D,D] [1  4] [- - - -]
        0x53 => ld_r8_r8(reg, R8::D, R8::E),          // [LD D,E] [1  4] [- - - -]
        0x54 => ld_r8_r8(reg, R8::D, R8::H),          // [LD D,H] [1  4] [- - - -]
        0x55 => ld_r8_r8(reg, R8::D, R8::L),          // [LD D,L] [1  4] [- - - -]
        0x56 => ld_r8_addr(reg, ram, R8::D, R16::HL), // [LD D,(HL)] [1  8] [- - - -]
        0x57 => ld_r8_r8(reg, R8::D, R8::A),          // [LD D,A] [1  4] [- - - -]
        0x58 => ld_r8_r8(reg, R8::E, R8::B),          // [LD E,B] [1  4] [- - - -]
        0x59 => ld_r8_r8(reg, R8::E, R8::C),          // [LD E,C] [1  4] [- - - -]
        0x5A => ld_r8_r8(reg, R8::E, R8::D),          // [LD E,D] [1  4] [- - - -]
        0x5B => ld_r8_r8(reg, R8::E, R8::E),          // [LD E,E] [1  4] [- - - -]
        0x5C => ld_r8_r8(reg, R8::E, R8::H),          // [LD E,H] [1  4] [- - - -]
        0x5D => ld_r8_r8(reg, R8::E, R8::L),          // [LD E,L] [1  4] [- - - -]
        0x5E => ld_r8_addr(reg, ram, R8::E, R16::HL), // [LD E,(HL)] [1  8] [- - - -]
        0x5F => ld_r8_r8(reg, R8::E, R8::A),          // [LD E,A] [1  4] [- - - -]
        0x60 => ld_r8_r8(reg, R8::H, R8::B),          // [LD H,B] [1  4] [- - - -]
        0x61 => ld_r8_r8(reg, R8::H, R8::C),          // [LD H,C] [1  4] [- - - -]
        0x62 => ld_r8_r8(reg, R8::H, R8::D),          // [LD H,D] [1  4] [- - - -]
        0x63 => ld_r8_r8(reg, R8::H, R8::E),          // [LD H,E] [1  4] [- - - -]
        0x64 => ld_r8_r8(reg, R8::H, R8::H),          // [LD H,H] [1  4] [- - - -]
        0x65 => ld_r8_r8(reg, R8::H, R8::L),          // [LD H,L] [1  4] [- - - -]
        0x66 => ld_r8_addr(reg, ram, R8::H, R16::HL), // [LD H,(HL)] [1  8] [- - - -]
        0x67 => ld_r8_r8(reg, R8::H, R8::A),          // [LD H,A] [1  4] [- - - -]
        0x68 => ld_r8_r8(reg, R8::L, R8::B),          // [LD L,B] [1  4] [- - - -]
        0x69 => ld_r8_r8(reg, R8::L, R8::C),          // [LD L,C] [1  4] [- - - -]
        0x6A => ld_r8_r8(reg, R8::L, R8::D),          // [LD L,D] [1  4] [- - - -]
        0x6B => ld_r8_r8(reg, R8::L, R8::E),          // [LD L,E] [1  4] [- - - -]
        0x6C => ld_r8_r8(reg, R8::L, R8::H),          // [LD L,H] [1  4] [- - - -]
        0x6D => ld_r8_r8(reg, R8::L, R8::L),          // [LD L,L] [1  4] [- - - -]
        0x6E => ld_r8_addr(reg, ram, R8::L, R16::HL), // [LD L,(HL)] [1  8] [- - - -]
        0x6F => ld_r8_r8(reg, R8::L, R8::A),          // [LD L,A] [1  4] [- - - -]
        0x70 => ld_addr_r8(reg, ram, R16::HL, R8::B), // [LD (HL),B] [1  8] [- - - -]
        0x71 => ld_addr_r8(reg, ram, R16::HL, R8::C), // [LD (HL),C] [1  8] [- - - -]
        0x72 => ld_addr_r8(reg, ram, R16::HL, R8::D), // [LD (HL),D] [1  8] [- - - -]
        0x73 => ld_addr_r8(reg, ram, R16::HL, R8::E), // [LD (HL),E] [1  8] [- - - -]
        0x74 => ld_addr_r8(reg, ram, R16::HL, R8::H), // [LD (HL),H] [1  8] [- - - -]
        0x75 => ld_addr_r8(reg, ram, R16::HL, R8::L), // [LD (HL),L] [1  8] [- - - -]
        0x76 => (0, 0),                               // TODO: [HALT] [1  4] [- - - -]
        0x77 => ld_addr_r8(reg, ram, R16::HL, R8::A), // [LD (HL),A] [1  8] [- - - -]
        0x78 => ld_r8_r8(reg, R8::A, R8::B),          // [LD A,B] [1  4] [- - - -]
        0x79 => ld_r8_r8(reg, R8::A, R8::C),          // [LD A,C] [1  4] [- - - -]
        0x7A => ld_r8_r8(reg, R8::A, R8::D),          // [LD A,D] [1  4] [- - - -]
        0x7B => ld_r8_r8(reg, R8::A, R8::E),          // [LD A,E] [1  4] [- - - -]
        0x7C => ld_r8_r8(reg, R8::A, R8::H),          // [LD A,H] [1  4] [- - - -]
        0x7D => ld_r8_r8(reg, R8::A, R8::L),          // [LD A,L] [1  4] [- - - -]
        0x7E => ld_r8_addr(reg, ram, R8::A, R16::HL), // [LD A,(HL)] [1  8] [- - - -]
        0x7F => ld_r8_r8(reg, R8::A, R8::A),          // [LD A,A] [1  4] [- - - -]
        0x80 => add_r8(reg, R8::B),                   // [ADD A,B] [1  4] [Z 0 H C]
        0x81 => add_r8(reg, R8::C),                   // [ADD A,C] [1  4] [Z 0 H C]
        0x82 => add_r8(reg, R8::D),                   // [ADD A,D] [1  4] [Z 0 H C]
        0x83 => add_r8(reg, R8::E),                   // [ADD A,E] [1  4] [Z 0 H C]
        0x84 => add_r8(reg, R8::H),                   // [ADD A,H] [1  4] [Z 0 H C]
        0x85 => add_r8(reg, R8::L),                   // [ADD A,L] [1  4] [Z 0 H C]
        0x86 => add_addr(reg, ram, R16::HL),          // [ADD A,(HL)] [1  8] [Z 0 H C]
        0x87 => add_r8(reg, R8::A),                   // [ADD A,A] [1  4] [Z 0 H C]
        0x88 => adc_r8(reg, R8::B),                   // [ADC A,B] [1  4] [Z 0 H C]
        0x89 => adc_r8(reg, R8::C),                   // [ADC A,C] [1  4] [Z 0 H C]
        0x8A => adc_r8(reg, R8::D),                   // [ADC A,D] [1  4] [Z 0 H C]
        0x8B => adc_r8(reg, R8::E),                   // [ADC A,E] [1  4] [Z 0 H C]
        0x8C => adc_r8(reg, R8::H),                   // [ADC A,H] [1  4] [Z 0 H C]
        0x8D => adc_r8(reg, R8::L),                   // [ADC A,L] [1  4] [Z 0 H C]
        0x8E => adc_addr(reg, ram, R16::HL),          // [ADC A,(HL)] [1  8] [Z 0 H C]
        0x8F => adc_r8(reg, R8::A),                   // [ADC A,A] [1  4] [Z 0 H C]
        0x90 => sub_r8(reg, R8::B),                   // [SUB A,B] [1  4] [Z 1 H C]
        0x91 => sub_r8(reg, R8::C),                   // [SUB A,C] [1  4] [Z 1 H C]
        0x92 => sub_r8(reg, R8::D),                   // [SUB A,D] [1  4] [Z 1 H C]
        0x93 => sub_r8(reg, R8::E),                   // [SUB A,E] [1  4] [Z 1 H C]
        0x94 => sub_r8(reg, R8::H),                   // [SUB A,H] [1  4] [Z 1 H C]
        0x95 => sub_r8(reg, R8::L),                   // [SUB A,L] [1  4] [Z 1 H C]
        0x96 => sub_addr(reg, ram, R16::HL),          // [SUB A,(HL)] [1  8] [Z 1 H C]
        0x97 => sub_r8(reg, R8::A),                   // [SUB A,A] [1  4] [Z 1 H C]
        0x98 => sbc_r8(reg, R8::B),                   // [SBC A,B] [1  4] [Z 1 H C]
        0x99 => sbc_r8(reg, R8::C),                   // [SBC A,C] [1  4] [Z 1 H C]
        0x9A => sbc_r8(reg, R8::D),                   // [SBC A,D] [1  4] [Z 1 H C]
        0x9B => sbc_r8(reg, R8::E),                   // [SBC A,E] [1  4] [Z 1 H C]
        0x9C => sbc_r8(reg, R8::H),                   // [SBC A,H] [1  4] [Z 1 H C]
        0x9D => sbc_r8(reg, R8::L),                   // [SBC A,L] [1  4] [Z 1 H C]
        0x9E => sbc_addr(reg, ram, R16::HL),          // [SBC A,(HL)] [1  8] [Z 1 H C]
        0x9F => sbc_r8(reg, R8::A),                   // [SBC A,A] [1  4] [Z 1 H C]
        0xA0 => and_r8(reg, R8::B),                   // [AND B] [1  4] [Z 0 1 0]
        0xA1 => and_r8(reg, R8::C),                   // [AND C] [1  4] [Z 0 1 0]
        0xA2 => and_r8(reg, R8::D),                   // [AND D] [1  4] [Z 0 1 0]
        0xA3 => and_r8(reg, R8::E),                   // [AND E] [1  4] [Z 0 1 0]
        0xA4 => and_r8(reg, R8::H),                   // [AND H] [1  4] [Z 0 1 0]
        0xA5 => and_r8(reg, R8::L),                   // [AND L] [1  4] [Z 0 1 0]
        0xA6 => and_addr(reg, ram, R16::HL),          // [AND (HL)] [1  8] [Z 0 1 0]
        0xA7 => and_r8(reg, R8::A),                   // [AND A] [1  4] [Z 0 1 0]
        0xA8 => xor_r8(reg, R8::B),                   // [XOR B] [1  4] [Z 0 0 0]
        0xA9 => xor_r8(reg, R8::C),                   // [XOR C] [1  4] [Z 0 0 0]
        0xAA => xor_r8(reg, R8::D),                   // [XOR D] [1  4] [Z 0 0 0]
        0xAB => xor_r8(reg, R8::E),                   // [XOR E] [1  4] [Z 0 0 0]
        0xAC => xor_r8(reg, R8::H),                   // [XOR H] [1  4] [Z 0 0 0]
        0xAD => xor_r8(reg, R8::L),                   // [XOR L] [1  4] [Z 0 0 0]
        0xAE => xor_addr(reg, ram, R16::HL),          // [XOR (HL)] [1  8] [Z 0 0 0]
        0xAF => xor_r8(reg, R8::A),                   // [XOR A] [1  4] [Z 0 0 0]
        0xB0 => or_r8(reg, R8::B),                    // [OR B] [1  4] [Z 0 0 0]
        0xB1 => or_r8(reg, R8::C),                    // [OR C] [1  4] [Z 0 0 0]
        0xB2 => or_r8(reg, R8::D),                    // [OR D] [1  4] [Z 0 0 0]
        0xB3 => or_r8(reg, R8::E),                    // [OR E] [1  4] [Z 0 0 0]
        0xB4 => or_r8(reg, R8::H),                    // [OR H] [1  4] [Z 0 0 0]
        0xB5 => or_r8(reg, R8::L),                    // [OR L] [1  4] [Z 0 0 0]
        0xB6 => or_addr(reg, ram, R16::HL),           // [OR (HL)] [1  8] [Z 0 0 0]
        0xB7 => or_r8(reg, R8::A),                    // [OR A] [1  4] [Z 0 0 0]
        0xB8 => (0, 0),                               // TODO: [CP B] [1  4] [Z 1 H C]
        0xB9 => (0, 0),                               // TODO: [CP C] [1  4] [Z 1 H C]
        0xBA => (0, 0),                               // TODO: [CP D] [1  4] [Z 1 H C]
        0xBB => (0, 0),                               // TODO: [CP E] [1  4] [Z 1 H C]
        0xBC => (0, 0),                               // TODO: [CP H] [1  4] [Z 1 H C]
        0xBD => (0, 0),                               // TODO: [CP L] [1  4] [Z 1 H C]
        0xBE => (0, 0),                               // TODO: [CP (HL)] [1  8] [Z 1 H C]
        0xBF => (0, 0),                               // TODO: [CP A] [1  4] [Z 1 H C]
        0xC0 => (0, 0),                               // TODO: [RET NZ] [1  20/8] [- - - -]
        0xC1 => (0, 0),                               // TODO: [POP BC] [1  12] [- - - -]
        0xC2 => (0, 0),                               // TODO: [JP NZ,a16] [3  16/12] [- - - -]
        0xC3 => (0, 0),                               // TODO: [JP a16] [3  16] [- - - -]
        0xC4 => (0, 0),                               // TODO: [CALL NZ,a16] [3  24/12] [- - - -]
        0xC5 => (0, 0),                               // TODO: [PUSH BC] [1  16] [- - - -]
        0xC6 => add_n8(reg, ram),                     // [ADD A,d8] [2  8] [Z 0 H C]
        0xC7 => (0, 0),                               // TODO: [RST 00H] [1  16] [- - - -]
        0xC8 => (0, 0),                               // TODO: [RET Z] [1  20/8] [- - - -]
        0xC9 => (0, 0),                               // TODO: [RET] [1  16] [- - - -]
        0xCA => (0, 0),                               // TODO: [JP Z,a16] [3  16/12] [- - - -]
        0xCB => unsupported(opcode),                  // [PREFIX CB] [1  4] [- - - -]
        0xCC => (0, 0),                               // TODO: [CALL Z,a16] [3  24/12] [- - - -]
        0xCD => (0, 0),                               // TODO: [CALL a16] [3  24] [- - - -]
        0xCE => adc_n8(reg, ram),                     // [ADC A,d8] [2  8] [Z 0 H C]
        0xCF => (0, 0),                               // TODO: [RST 08H] [1  16] [- - - -]
        0xD0 => (0, 0),                               // TODO: [RET NC] [1  20/8] [- - - -]
        0xD1 => (0, 0),                               // TODO: [POP DE] [1  12] [- - - -]
        0xD2 => (0, 0),                               // TODO: [JP NC,a16] [3  16/12] [- - - -]
        0xD3 => unsupported(opcode),                  // [Unsupported]
        0xD4 => (0, 0),                               // TODO: [CALL NC,a16] [3  24/12] [- - - -]
        0xD5 => (0, 0),                               // TODO: [PUSH DE] [1  16] [- - - -]
        0xD6 => sub_n8(reg, ram),                     // [SUB A,d8] [2  8] [Z 1 H C]
        0xD7 => (0, 0),                               // TODO: [RST 10H] [1  16] [- - - -]
        0xD8 => (0, 0),                               // TODO: [RET C] [1  20/8] [- - - -]
        0xD9 => (0, 0),                               // TODO: [RETI] [1  16] [- - - -]
        0xDA => (0, 0),                               // TODO: [JP C,a16] [3  16/12] [- - - -]
        0xDB => unsupported(opcode),                  // [Unsupported]
        0xDC => (0, 0),                               // TODO: [CALL C,a16] [3  24/12] [- - - -]
        0xDD => unsupported(opcode),                  // [Unsupported]
        0xDE => sbc_n8(reg, ram),                     // [SBC A,d8] [2  8] [Z 1 H C]
        0xDF => (0, 0),                               // TODO: [RST 18H] [1  16] [- - - -]
        0xE0 => (0, 0),                               // TODO: [LDH (a8),A] [2  12] [- - - -]
        0xE1 => (0, 0),                               // TODO: [POP HL] [1  12] [- - - -]
        0xE2 => (0, 0),                               // TODO: [LD (C),A] [2  8] [- - - -]
        0xE3 => unsupported(opcode),                  // [Unsupported]
        0xE4 => unsupported(opcode),                  // [Unsupported]
        0xE5 => (0, 0),                               // TODO: [PUSH HL] [1  16] [- - - -]
        0xE6 => and_n8(reg, ram),                     // [AND d8] [2  8] [Z 0 1 0]
        0xE7 => (0, 0),                               // TODO: [RST 20H] [1  16] [- - - -]
        0xE8 => (0, 0),                               // TODO: [ADD SP,r8] [2  16] [0 0 H C]
        0xE9 => (0, 0),                               // TODO: [JP (HL)] [1  4] [- - - -]
        0xEA => (0, 0),                               // TODO: [LD (a16),A] [3  16] [- - - -]
        0xEB => unsupported(opcode),                  // [Unsupported]
        0xEC => unsupported(opcode),                  // [Unsupported]
        0xED => unsupported(opcode),                  // [Unsupported]
        0xEE => xor_n8(reg, ram),                     // [XOR d8] [2  8] [Z 0 0 0]
        0xEF => (0, 0),                               // TODO: [RST 28H] [1  16] [- - - -]
        0xF0 => (0, 0),                               // TODO: [LDH A,(a8)] [2  12] [- - - -]
        0xF1 => (0, 0),                               // TODO: [POP AF] [1  12] [Z N H C]
        0xF2 => (0, 0),                               // TODO: [LD A,(C)] [2  8] [- - - -]
        0xF3 => (0, 0),                               // TODO: [DI] [1  4] [- - - -]
        0xF4 => unsupported(opcode),                  // [Unsupported]
        0xF5 => (0, 0),                               // TODO: [PUSH AF] [1  16] [- - - -]
        0xF6 => or_n8(reg, ram),                      // [OR d8] [2  8] [Z 0 0 0]
        0xF7 => (0, 0),                               // TODO: [RST 30H] [1  16] [- - - -]
        0xF8 => (0, 0),                               // TODO: [LD HL,SP+r8] [2  12] [0 0 H C]
        0xF9 => (0, 0),                               // TODO: [LD SP,HL] [1  8] [- - - -]
        0xFA => (0, 0),                               // TODO: [LD A,(a16)] [3  16] [- - - -]
        0xFB => (0, 0),                               // TODO: [EI] [1  4] [- - - -]
        0xFC => unsupported(opcode),                  // [Unsupported]
        0xFD => unsupported(opcode),                  // [Unsupported]
        0xFE => (0, 0),                               // TODO: [CP d8] [2  8] [Z 1 H C]
        0xFF => (0, 0),                               // TODO: [RST 38H] [1  16] [- - - -]
        _ => unsupported(opcode),
    }
}

pub fn exec_ex(opcode: u8, reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
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
        _ => unsupported(opcode),
    }
}

fn unsupported(opcode: u8) -> (u8, u8) {
    println!("Unsupported or unknown opcode specified: 0x{:02X}", opcode);
    (1, 0)
}

fn ld_r8_n8(reg: &mut Registers, ram: &mut Ram, lhs: R8) -> (u8, u8) {
    reg.set8(lhs, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn ld_r8_r8(reg: &mut Registers, lhs: R8, rhs: R8) -> (u8, u8) {
    reg.set8(lhs, reg.get8(rhs));
    (1, 4)
}

fn ld_r8_addr(reg: &mut Registers, ram: &mut Ram, lhs: R8, rhs: R16) -> (u8, u8) {
    reg.set8(lhs, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn ld_addr_r8(reg: &mut Registers, ram: &mut Ram, lhs: R16, rhs: R8) -> (u8, u8) {
    ram.write(reg.get16(lhs), reg.get8(rhs));
    (1, 8)
}

fn add_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.add8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn add_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.add8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn add_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.add8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn adc_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.adc8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn adc_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.adc8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn adc_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.adc8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn inc_r8(reg: &mut Registers, lhs: R8) -> (u8, u8) {
    reg.inc8(lhs);
    (1, 4)
}

fn sub_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.sub8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn sub_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.sub8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn sub_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.sub8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn sbc_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.sbc8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn sbc_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.sbc8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn sbc_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.sbc8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn dec_r8(reg: &mut Registers, lhs: R8) -> (u8, u8) {
    reg.dec8(lhs);
    (1, 4)
}

fn and_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.and8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn and_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.and8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn and_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.and8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn or_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.or8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn or_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.or8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn or_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.or8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

fn xor_r8(reg: &mut Registers, rhs: R8) -> (u8, u8) {
    reg.xor8(R8::A, reg.get8(rhs));
    (1, 4)
}

fn xor_addr(reg: &mut Registers, ram: &mut Ram, rhs: R16) -> (u8, u8) {
    reg.xor8(R8::A, ram.read(reg.get16(rhs)));
    (1, 8)
}

fn xor_n8(reg: &mut Registers, ram: &mut Ram) -> (u8, u8) {
    reg.xor8(R8::A, ram.read(reg.get_PC() + 1));
    (2, 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_r8_n8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);

        ld_r8_n8(&mut reg, &mut ram, R8::B);
        assert_eq!(0xAA, reg.get8(R8::B));
    }

    #[test]
    fn test_ld_r8_r8() {
        let mut reg = Registers::new();
        reg.set8(R8::B, 0xAA);

        ld_r8_r8(&mut reg, R8::C, R8::B);
        assert_eq!(0xAA, reg.get8(R8::B));
        assert_eq!(0xAA, reg.get8(R8::C));
    }

    #[test]
    fn test_ld_r8_addr() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        reg.set16(R16::HL, 0x01);

        ld_r8_addr(&mut reg, &mut ram, R8::B, R16::HL);
        assert_eq!(0xAA, reg.get8(R8::B));
    }

    #[test]
    fn test_ld_addr_r8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0xAA]);
        reg.set16(R16::HL, 0x01);
        reg.set8(R8::B, 0xBB);

        ld_addr_r8(&mut reg, &mut ram, R16::HL, R8::B);
        assert_eq!(0xBB, ram.read(0x01));
    }

    #[test]
    fn test_add_addr() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0x01]);
        reg.set16(R16::HL, 0x01);
        reg.set8(R8::A, 0x0F);

        add_addr(&mut reg, &mut ram, R16::HL);
        assert_eq!(0x10, reg.get8(R8::A));
    }

    #[test]
    fn test_add_n8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0x01]);
        reg.set8(R8::A, 0x0F);

        add_n8(&mut reg, &mut ram);
        assert_eq!(0x10, reg.get8(R8::A));
    }

    #[test]
    fn test_sub_addr() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0x01]);
        reg.set16(R16::HL, 0x01);
        reg.set8(R8::A, 0x10);

        sub_addr(&mut reg, &mut ram, R16::HL);
        assert_eq!(0x0F, reg.get8(R8::A));
    }

    #[test]
    fn test_sub_n8() {
        let mut reg = Registers::new();
        let mut ram = Ram::new(vec![0x00, 0x01]);
        reg.set8(R8::A, 0x10);

        sub_n8(&mut reg, &mut ram);
        assert_eq!(0x0F, reg.get8(R8::A));
    }
}
