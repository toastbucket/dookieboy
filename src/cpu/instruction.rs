use crate::cpu::{Register8Bit, Register16Bit, RstVec};

#[derive(Debug, Copy, Clone)]
pub enum BranchCondition {
    NONE,
    NZ,
    NC,
    Z,
    C,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Noop(),
    Inc(Register8Bit),
    Inc16(Register16Bit),
    Dec(Register8Bit),
    Dec16(Register16Bit),
    And(Register8Bit),
    AndFromMem(), // always uses HL
    AndImm(),
    Or(Register8Bit),
    OrFromMem(), // always uses HL
    OrImm(),
    Xor(Register8Bit),
    XorFromMem(), // always uses HL
    XorImm(),
    Cp(Register8Bit),
    CpFromMem(), // always uses HL
    CpImm(),
    Rra(),
    Rrca(),
    Rla(),
    Rlca(),
    Add(Register8Bit),
    AddImm(),
    AddFromMem(), // always uses HL
    Add16(Register16Bit),
    AddSpS8(),
    Adc(Register8Bit),
    AdcFromMem(), // always uses HL
    Sub(Register8Bit),
    SubImm(),
    SubFromMem(), // always uses HL
    Sbc(Register8Bit),
    SbcFromMem(), // always uses HL
    LdRegister(Register8Bit, Register8Bit),
    LdImm(Register8Bit),
    LdToMem(Register8Bit, Register16Bit),
    LdFromMem(Register8Bit, Register16Bit),
    LdToMemInc(), // Always A and HL
    LdToMemDec(), // Always A and HL
    LdFromMemInc(), // Always A and HL
    LdFromMemDec(), // Always A and HL
    LdRegister16Imm(Register16Bit),
    LdToUpperMem(),
    LdFromUpperMem(),
    JumpAbs(BranchCondition),
    JumpAbsFromReg(), // always uses HL
    JumpRel(BranchCondition),
    Push(Register16Bit),
    Pop(Register16Bit),
    Ret(BranchCondition),
    Rst(RstVec),
    Call(BranchCondition),
    ToggleCarryFlag(),
    SetCarryFlag(),
    Invert(),
    Stop(),
    Halt(),
    CbInstruction(), // Special dummy instruction for 16 bit instruction calls
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            // NOOP
            0x00 => Some(Instruction::Noop()),
            // INC r
            0x04 => Some(Instruction::Inc(Register8Bit::B)),
            0x0c => Some(Instruction::Inc(Register8Bit::C)),
            0x14 => Some(Instruction::Inc(Register8Bit::D)),
            0x1c => Some(Instruction::Inc(Register8Bit::E)),
            0x24 => Some(Instruction::Inc(Register8Bit::H)),
            0x2c => Some(Instruction::Inc(Register8Bit::L)),
            0x3c => Some(Instruction::Inc(Register8Bit::A)),
            // INC r16
            0x03 => Some(Instruction::Inc16(Register16Bit::BC)),
            0x13 => Some(Instruction::Inc16(Register16Bit::DE)),
            0x23 => Some(Instruction::Inc16(Register16Bit::HL)),
            0x33 => Some(Instruction::Inc16(Register16Bit::SP)),
            // DEC r
            0x05 => Some(Instruction::Dec(Register8Bit::B)),
            0x0d => Some(Instruction::Dec(Register8Bit::C)),
            0x15 => Some(Instruction::Dec(Register8Bit::D)),
            0x1d => Some(Instruction::Dec(Register8Bit::E)),
            0x25 => Some(Instruction::Dec(Register8Bit::H)),
            0x2d => Some(Instruction::Dec(Register8Bit::L)),
            0x3d => Some(Instruction::Dec(Register8Bit::A)),
            // DEC r16
            0x0b => Some(Instruction::Dec16(Register16Bit::BC)),
            0x1b => Some(Instruction::Dec16(Register16Bit::DE)),
            0x2b => Some(Instruction::Dec16(Register16Bit::HL)),
            0x3b => Some(Instruction::Dec16(Register16Bit::SP)),
            // AND r
            0xa0 => Some(Instruction::And(Register8Bit::B)),
            0xa1 => Some(Instruction::And(Register8Bit::C)),
            0xa2 => Some(Instruction::And(Register8Bit::D)),
            0xa3 => Some(Instruction::And(Register8Bit::E)),
            0xa4 => Some(Instruction::And(Register8Bit::H)),
            0xa5 => Some(Instruction::And(Register8Bit::L)),
            0xa7 => Some(Instruction::And(Register8Bit::A)),
            // AND (HL)
            0xa6 => Some(Instruction::AndFromMem()),
            // AND n
            0xe6 => Some(Instruction::AndImm()),
            // OR A,r
            0xb0 => Some(Instruction::Or(Register8Bit::B)),
            0xb1 => Some(Instruction::Or(Register8Bit::C)),
            0xb2 => Some(Instruction::Or(Register8Bit::D)),
            0xb3 => Some(Instruction::Or(Register8Bit::E)),
            0xb4 => Some(Instruction::Or(Register8Bit::H)),
            0xb5 => Some(Instruction::Or(Register8Bit::L)),
            0xb7 => Some(Instruction::Or(Register8Bit::A)),
            // OR (HL)
            0xb6 => Some(Instruction::OrFromMem()),
            // OR n
            0xf6 => Some(Instruction::OrImm()),
            // XOR A,r
            0xa8 => Some(Instruction::Xor(Register8Bit::B)),
            0xa9 => Some(Instruction::Xor(Register8Bit::C)),
            0xaa => Some(Instruction::Xor(Register8Bit::D)),
            0xab => Some(Instruction::Xor(Register8Bit::E)),
            0xac => Some(Instruction::Xor(Register8Bit::H)),
            0xad => Some(Instruction::Xor(Register8Bit::L)),
            0xaf => Some(Instruction::Xor(Register8Bit::A)),
            // XOR (HL)
            0xae => Some(Instruction::XorFromMem()),
            // XOR n
            0xee => Some(Instruction::XorImm()),
            // CP A,r
            0xb8 => Some(Instruction::Cp(Register8Bit::B)),
            0xb9 => Some(Instruction::Cp(Register8Bit::C)),
            0xba => Some(Instruction::Cp(Register8Bit::D)),
            0xbb => Some(Instruction::Cp(Register8Bit::E)),
            0xbc => Some(Instruction::Cp(Register8Bit::H)),
            0xbd => Some(Instruction::Cp(Register8Bit::L)),
            0xbf => Some(Instruction::Cp(Register8Bit::A)),
            // CP (HL)
            0xbe => Some(Instruction::CpFromMem()),
            // CP n
            0xfe => Some(Instruction::CpImm()),
            // RLA
            0x17 => Some(Instruction::Rla()),
            // RLCA
            0x07 => Some(Instruction::Rlca()),
            // RRA
            0x1f => Some(Instruction::Rra()),
            // RRCA
            0x0f => Some(Instruction::Rrca()),
            // ADD A,r
            0x80 => Some(Instruction::Add(Register8Bit::B)),
            0x81 => Some(Instruction::Add(Register8Bit::C)),
            0x82 => Some(Instruction::Add(Register8Bit::D)),
            0x83 => Some(Instruction::Add(Register8Bit::E)),
            0x84 => Some(Instruction::Add(Register8Bit::H)),
            0x85 => Some(Instruction::Add(Register8Bit::L)),
            0x87 => Some(Instruction::Add(Register8Bit::A)),
            // ADD A,d8
            0xc6 => Some(Instruction::AddImm()),
            // ADD A,(HL)
            0x86 => Some(Instruction::AddFromMem()),
            // ADD HL,(nn)
            0x09 => Some(Instruction::Add16(Register16Bit::BC)),
            0x19 => Some(Instruction::Add16(Register16Bit::DE)),
            0x29 => Some(Instruction::Add16(Register16Bit::HL)),
            0x39 => Some(Instruction::Add16(Register16Bit::SP)),
            // ADD SP,s8
            0xe8 => Some(Instruction::AddSpS8()),
            // ADC A,r
            0x88 => Some(Instruction::Adc(Register8Bit::B)),
            0x89 => Some(Instruction::Adc(Register8Bit::C)),
            0x8a => Some(Instruction::Adc(Register8Bit::D)),
            0x8b => Some(Instruction::Adc(Register8Bit::E)),
            0x8c => Some(Instruction::Adc(Register8Bit::H)),
            0x8d => Some(Instruction::Adc(Register8Bit::L)),
            0x8f => Some(Instruction::Adc(Register8Bit::A)),
            // ADC A,(HL)
            0x8e => Some(Instruction::AdcFromMem()),
            // SUB A,r
            0x90 => Some(Instruction::Sub(Register8Bit::B)),
            0x91 => Some(Instruction::Sub(Register8Bit::C)),
            0x92 => Some(Instruction::Sub(Register8Bit::D)),
            0x93 => Some(Instruction::Sub(Register8Bit::E)),
            0x94 => Some(Instruction::Sub(Register8Bit::H)),
            0x95 => Some(Instruction::Sub(Register8Bit::L)),
            0x97 => Some(Instruction::Sub(Register8Bit::A)),
            // SUB A,d8
            0xd6 => Some(Instruction::SubImm()),
            // SUB A,(HL)
            0x96 => Some(Instruction::SubFromMem()),
            // SBC A,r
            0x98 => Some(Instruction::Sbc(Register8Bit::B)),
            0x99 => Some(Instruction::Sbc(Register8Bit::C)),
            0x9a => Some(Instruction::Sbc(Register8Bit::D)),
            0x9b => Some(Instruction::Sbc(Register8Bit::E)),
            0x9c => Some(Instruction::Sbc(Register8Bit::H)),
            0x9d => Some(Instruction::Sbc(Register8Bit::L)),
            0x9f => Some(Instruction::Sbc(Register8Bit::A)),
            // SBC A,(HL)
            0x9e => Some(Instruction::SbcFromMem()),
            // LD B, X
            0x40 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::B)),
            0x41 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::C)),
            0x42 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::D)),
            0x43 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::E)),
            0x44 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::H)),
            0x45 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::L)),
            0x47 => Some(Instruction::LdRegister(Register8Bit::B, Register8Bit::A)),
            // LD C, X
            0x48 => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::B)),
            0x49 => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::C)),
            0x4a => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::D)),
            0x4b => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::E)),
            0x4c => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::H)),
            0x4d => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::L)),
            0x4f => Some(Instruction::LdRegister(Register8Bit::C, Register8Bit::A)),
            // LD D, X
            0x50 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::B)),
            0x51 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::C)),
            0x52 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::D)),
            0x53 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::E)),
            0x54 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::H)),
            0x55 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::L)),
            0x57 => Some(Instruction::LdRegister(Register8Bit::D, Register8Bit::A)),
            // LD E, X
            0x58 => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::B)),
            0x59 => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::C)),
            0x5a => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::D)),
            0x5b => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::E)),
            0x5c => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::H)),
            0x5d => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::L)),
            0x5f => Some(Instruction::LdRegister(Register8Bit::E, Register8Bit::A)),
            // LD H, X
            0x60 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::B)),
            0x61 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::C)),
            0x62 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::D)),
            0x63 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::E)),
            0x64 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::H)),
            0x65 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::L)),
            0x67 => Some(Instruction::LdRegister(Register8Bit::H, Register8Bit::A)),
            // LD L, X
            0x68 => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::B)),
            0x69 => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::C)), // nice
            0x6a => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::D)),
            0x6b => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::E)),
            0x6c => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::H)),
            0x6d => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::L)),
            0x6f => Some(Instruction::LdRegister(Register8Bit::L, Register8Bit::A)),
            // LD A, X
            0x78 => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::B)),
            0x79 => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::C)),
            0x7a => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::D)),
            0x7b => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::E)),
            0x7c => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::H)),
            0x7d => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::L)),
            0x7f => Some(Instruction::LdRegister(Register8Bit::A, Register8Bit::A)),
            // LD X, nn
            0x06 => Some(Instruction::LdImm(Register8Bit::B)),
            0x16 => Some(Instruction::LdImm(Register8Bit::D)),
            0x26 => Some(Instruction::LdImm(Register8Bit::H)),
            0x0e => Some(Instruction::LdImm(Register8Bit::C)),
            0x1e => Some(Instruction::LdImm(Register8Bit::E)),
            0x2e => Some(Instruction::LdImm(Register8Bit::L)),
            0x3e => Some(Instruction::LdImm(Register8Bit::A)),
            // LD (HL), X
            0x70 => Some(Instruction::LdToMem(Register8Bit::B, Register16Bit::HL)),
            0x71 => Some(Instruction::LdToMem(Register8Bit::C, Register16Bit::HL)),
            0x72 => Some(Instruction::LdToMem(Register8Bit::D, Register16Bit::HL)),
            0x73 => Some(Instruction::LdToMem(Register8Bit::E, Register16Bit::HL)),
            0x74 => Some(Instruction::LdToMem(Register8Bit::H, Register16Bit::HL)),
            0x75 => Some(Instruction::LdToMem(Register8Bit::L, Register16Bit::HL)),
            0x77 => Some(Instruction::LdToMem(Register8Bit::A, Register16Bit::HL)),
            // LD X, (HL)
            0x46 => Some(Instruction::LdFromMem(Register8Bit::B, Register16Bit::HL)),
            0x4e => Some(Instruction::LdFromMem(Register8Bit::C, Register16Bit::HL)),
            0x56 => Some(Instruction::LdFromMem(Register8Bit::D, Register16Bit::HL)),
            0x5e => Some(Instruction::LdFromMem(Register8Bit::E, Register16Bit::HL)),
            0x66 => Some(Instruction::LdFromMem(Register8Bit::H, Register16Bit::HL)),
            0x6e => Some(Instruction::LdFromMem(Register8Bit::L, Register16Bit::HL)),
            0x7e => Some(Instruction::LdFromMem(Register8Bit::A, Register16Bit::HL)),
            // LD A, (XX)
            0x02 => Some(Instruction::LdToMem(Register8Bit::A, Register16Bit::BC)),
            0x12 => Some(Instruction::LdToMem(Register8Bit::A, Register16Bit::DE)),
            0x0a => Some(Instruction::LdFromMem(Register8Bit::A, Register16Bit::BC)),
            0x1a => Some(Instruction::LdFromMem(Register8Bit::A, Register16Bit::DE)),
            // LD A, HL(x)crement
            0x22 => Some(Instruction::LdToMemInc()),
            0x32 => Some(Instruction::LdToMemDec()),
            0x2a => Some(Instruction::LdFromMemInc()),
            0x3a => Some(Instruction::LdFromMemDec()),
            // LD XX, nn
            0x01 => Some(Instruction::LdRegister16Imm(Register16Bit::BC)),
            0x11 => Some(Instruction::LdRegister16Imm(Register16Bit::DE)),
            0x21 => Some(Instruction::LdRegister16Imm(Register16Bit::HL)),
            0x31 => Some(Instruction::LdRegister16Imm(Register16Bit::SP)),
            // LD (n) A
            0xe0 => Some(Instruction::LdToUpperMem()),
            // LD A (n)
            0xf0 => Some(Instruction::LdFromUpperMem()),
            // JP
            0xc2 => Some(Instruction::JumpAbs(BranchCondition::NZ)),
            0xd2 => Some(Instruction::JumpAbs(BranchCondition::NC)),
            0xc3 => Some(Instruction::JumpAbs(BranchCondition::NONE)),
            0xca => Some(Instruction::JumpAbs(BranchCondition::Z)),
            0xda => Some(Instruction::JumpAbs(BranchCondition::C)),
            // JP (HL)
            0xe9 => Some(Instruction::JumpAbsFromReg()),
            // JR
            0x20 => Some(Instruction::JumpRel(BranchCondition::NZ)),
            0x30 => Some(Instruction::JumpRel(BranchCondition::NC)),
            0x18 => Some(Instruction::JumpRel(BranchCondition::NONE)),
            0x28 => Some(Instruction::JumpRel(BranchCondition::Z)),
            0x38 => Some(Instruction::JumpRel(BranchCondition::C)),
            // PUSH (XX)
            0xc5 => Some(Instruction::Push(Register16Bit::BC)),
            0xd5 => Some(Instruction::Push(Register16Bit::DE)),
            0xe5 => Some(Instruction::Push(Register16Bit::HL)),
            0xf5 => Some(Instruction::Push(Register16Bit::AF)),
            // POP (XX)
            0xc1 => Some(Instruction::Pop(Register16Bit::BC)),
            0xd1 => Some(Instruction::Pop(Register16Bit::DE)),
            0xe1 => Some(Instruction::Pop(Register16Bit::HL)),
            0xf1 => Some(Instruction::Pop(Register16Bit::AF)),
            // RET
            0xc0 => Some(Instruction::Ret(BranchCondition::NZ)),
            0xd0 => Some(Instruction::Ret(BranchCondition::NC)),
            0xc8 => Some(Instruction::Ret(BranchCondition::Z)),
            0xd8 => Some(Instruction::Ret(BranchCondition::C)),
            0xc9 => Some(Instruction::Ret(BranchCondition::NONE)),
            // RST
            0xc7 => Some(Instruction::Rst(RstVec::ZERO)),
            0xcf => Some(Instruction::Rst(RstVec::ONE)),
            0xd7 => Some(Instruction::Rst(RstVec::TWO)),
            0xdf => Some(Instruction::Rst(RstVec::THREE)),
            0xe7 => Some(Instruction::Rst(RstVec::FOUR)),
            0xef => Some(Instruction::Rst(RstVec::FIVE)),
            0xf7 => Some(Instruction::Rst(RstVec::SIX)),
            0xff => Some(Instruction::Rst(RstVec::SEVEN)),
            // CALL
            0xc4 => Some(Instruction::Call(BranchCondition::NZ)),
            0xd4 => Some(Instruction::Call(BranchCondition::NC)),
            0xcc => Some(Instruction::Call(BranchCondition::Z)),
            0xdc => Some(Instruction::Call(BranchCondition::C)),
            0xcd => Some(Instruction::Call(BranchCondition::NONE)),
            // CCF
            0x3f => Some(Instruction::ToggleCarryFlag()),
            // SCF
            0x37 => Some(Instruction::SetCarryFlag()),
            // CPL
            0x2f => Some(Instruction::Invert()),
            // STOP
            0x10 => Some(Instruction::Stop()),
            // HALT
            0x76 => Some(Instruction::Halt()),
            // Special dummy instruction for 16 bit instruction calls
            0xcb => Some(Instruction::CbInstruction()),
            _ => None
        }
    }

    pub fn as_byte(self) -> u8 {
        match self {
            // NOOP
            Instruction::Noop() => 0x00,
            // INC r
            Instruction::Inc(Register8Bit::B) => 0x04,
            Instruction::Inc(Register8Bit::C) => 0x0c,
            Instruction::Inc(Register8Bit::D) => 0x14,
            Instruction::Inc(Register8Bit::E) => 0x1c,
            Instruction::Inc(Register8Bit::H) => 0x24,
            Instruction::Inc(Register8Bit::L) => 0x2c,
            Instruction::Inc(Register8Bit::A) => 0x3c,
            // INC r16
            Instruction::Inc16(Register16Bit::BC) => 0x03,
            Instruction::Inc16(Register16Bit::DE) => 0x13,
            Instruction::Inc16(Register16Bit::HL) => 0x23,
            Instruction::Inc16(Register16Bit::SP) => 0x33,
            // DEC r
            Instruction::Dec(Register8Bit::B) => 0x05,
            Instruction::Dec(Register8Bit::C) => 0x0d,
            Instruction::Dec(Register8Bit::D) => 0x15,
            Instruction::Dec(Register8Bit::E) => 0x1d,
            Instruction::Dec(Register8Bit::H) => 0x25,
            Instruction::Dec(Register8Bit::L) => 0x2d,
            Instruction::Dec(Register8Bit::A) => 0x3d,
            // DEC r16
            Instruction::Dec16(Register16Bit::BC) => 0x0b,
            Instruction::Dec16(Register16Bit::DE) => 0x1b,
            Instruction::Dec16(Register16Bit::HL) => 0x2b,
            Instruction::Dec16(Register16Bit::SP) => 0x3b,
            // AND r
            Instruction::And(Register8Bit::B) => 0xa0,
            Instruction::And(Register8Bit::C) => 0xa1,
            Instruction::And(Register8Bit::D) => 0xa2,
            Instruction::And(Register8Bit::E) => 0xa3,
            Instruction::And(Register8Bit::H) => 0xa4,
            Instruction::And(Register8Bit::L) => 0xa5,
            Instruction::And(Register8Bit::A) => 0xa7,
            // AND (HL)
            Instruction::AndFromMem() => 0xa6,
            // AND n
            Instruction::AndImm() => 0xe6,
            // OR r
            Instruction::Or(Register8Bit::B) => 0xb0,
            Instruction::Or(Register8Bit::C) => 0xb1,
            Instruction::Or(Register8Bit::D) => 0xb2,
            Instruction::Or(Register8Bit::E) => 0xb3,
            Instruction::Or(Register8Bit::H) => 0xb4,
            Instruction::Or(Register8Bit::L) => 0xb5,
            Instruction::Or(Register8Bit::A) => 0xb7,
            // OR (HL)
            Instruction::OrFromMem() => 0xb6,
            // OR n
            Instruction::OrImm() => 0xf6,
            // XOR r
            Instruction::Xor(Register8Bit::B) => 0xa8,
            Instruction::Xor(Register8Bit::C) => 0xa9,
            Instruction::Xor(Register8Bit::D) => 0xaa,
            Instruction::Xor(Register8Bit::E) => 0xab,
            Instruction::Xor(Register8Bit::H) => 0xac,
            Instruction::Xor(Register8Bit::L) => 0xad,
            Instruction::Xor(Register8Bit::A) => 0xaf,
            // XOR (HL)
            Instruction::XorFromMem() => 0xae,
            // XOR n
            Instruction::XorImm() => 0xee,
            // CP r
            Instruction::Cp(Register8Bit::B) => 0xb8,
            Instruction::Cp(Register8Bit::C) => 0xb9,
            Instruction::Cp(Register8Bit::D) => 0xba,
            Instruction::Cp(Register8Bit::E) => 0xbb,
            Instruction::Cp(Register8Bit::H) => 0xbc,
            Instruction::Cp(Register8Bit::L) => 0xbd,
            Instruction::Cp(Register8Bit::A) => 0xbf,
            // CP (HL)
            Instruction::CpFromMem() => 0xbe,
            // CP n
            Instruction::CpImm() => 0xfe,
            // RLA
            Instruction::Rla() => 0x17,
            // RLCA
            Instruction::Rlca() => 0x07,
            // RRA
            Instruction::Rra() => 0x1f,
            // RRCA
            Instruction::Rrca() => 0x0f,
            // ADD A,r
            Instruction::Add(Register8Bit::B) => 0x80,
            Instruction::Add(Register8Bit::C) => 0x81,
            Instruction::Add(Register8Bit::D) => 0x82,
            Instruction::Add(Register8Bit::E) => 0x83,
            Instruction::Add(Register8Bit::H) => 0x84,
            Instruction::Add(Register8Bit::L) => 0x85,
            Instruction::Add(Register8Bit::A) => 0x87,
            // ADD A,d8
            Instruction::AddImm() => 0xc6,
            // ADD A,(HL)
            Instruction::AddFromMem() => 0x86,
            // ADD HL,(nn)
            Instruction::Add16(Register16Bit::BC) => 0x09,
            Instruction::Add16(Register16Bit::DE) => 0x19,
            Instruction::Add16(Register16Bit::HL) => 0x29,
            Instruction::Add16(Register16Bit::SP) => 0x39,
            // ADD SP,s8
            Instruction::AddSpS8() => 0xe8,
            // ADC A,R
            Instruction::Adc(Register8Bit::B) => 0x88,
            Instruction::Adc(Register8Bit::C) => 0x89,
            Instruction::Adc(Register8Bit::D) => 0x8a,
            Instruction::Adc(Register8Bit::E) => 0x8b,
            Instruction::Adc(Register8Bit::H) => 0x8c,
            Instruction::Adc(Register8Bit::L) => 0x8d,
            Instruction::Adc(Register8Bit::A) => 0x8f,
            // ADC A,(HL)
            Instruction::AdcFromMem() => 0x8e,
            // SUB A,r
            Instruction::Sub(Register8Bit::B) => 0x90,
            Instruction::Sub(Register8Bit::C) => 0x91,
            Instruction::Sub(Register8Bit::D) => 0x92,
            Instruction::Sub(Register8Bit::E) => 0x93,
            Instruction::Sub(Register8Bit::H) => 0x94,
            Instruction::Sub(Register8Bit::L) => 0x95,
            Instruction::Sub(Register8Bit::A) => 0x97,
            // SUB A,d8
            Instruction::SubImm() => 0xd6,
            // SUB A,(HL)
            Instruction::SubFromMem() => 0x96,
            // SBC A,r
            Instruction::Sbc(Register8Bit::B) => 0x98,
            Instruction::Sbc(Register8Bit::C) => 0x99,
            Instruction::Sbc(Register8Bit::D) => 0x9a,
            Instruction::Sbc(Register8Bit::E) => 0x9b,
            Instruction::Sbc(Register8Bit::H) => 0x9c,
            Instruction::Sbc(Register8Bit::L) => 0x9d,
            Instruction::Sbc(Register8Bit::A) => 0x9f,
            // SBC A,(HL)
            Instruction::SbcFromMem() => 0x9e,
            // LD B, Y
            Instruction::LdRegister(Register8Bit::B, Register8Bit::B) => 0x40,
            Instruction::LdRegister(Register8Bit::B, Register8Bit::C) => 0x41,
            Instruction::LdRegister(Register8Bit::B, Register8Bit::D) => 0x42,
            Instruction::LdRegister(Register8Bit::B, Register8Bit::E) => 0x43,
            Instruction::LdRegister(Register8Bit::B, Register8Bit::H) => 0x44,
            Instruction::LdRegister(Register8Bit::B, Register8Bit::L) => 0x45,
            Instruction::LdRegister(Register8Bit::B, Register8Bit::A) => 0x47,
            // LD C, X
            Instruction::LdRegister(Register8Bit::C, Register8Bit::B) => 0x48,
            Instruction::LdRegister(Register8Bit::C, Register8Bit::C) => 0x49,
            Instruction::LdRegister(Register8Bit::C, Register8Bit::D) => 0x4a,
            Instruction::LdRegister(Register8Bit::C, Register8Bit::E) => 0x4b,
            Instruction::LdRegister(Register8Bit::C, Register8Bit::H) => 0x4c,
            Instruction::LdRegister(Register8Bit::C, Register8Bit::L) => 0x4d,
            Instruction::LdRegister(Register8Bit::C, Register8Bit::A) => 0x4f,
            // LD D, X
            Instruction::LdRegister(Register8Bit::D, Register8Bit::B) => 0x50,
            Instruction::LdRegister(Register8Bit::D, Register8Bit::C) => 0x51,
            Instruction::LdRegister(Register8Bit::D, Register8Bit::D) => 0x52,
            Instruction::LdRegister(Register8Bit::D, Register8Bit::E) => 0x53,
            Instruction::LdRegister(Register8Bit::D, Register8Bit::H) => 0x54,
            Instruction::LdRegister(Register8Bit::D, Register8Bit::L) => 0x55,
            Instruction::LdRegister(Register8Bit::D, Register8Bit::A) => 0x57,
            // LD E, X
            Instruction::LdRegister(Register8Bit::E, Register8Bit::B) => 0x58,
            Instruction::LdRegister(Register8Bit::E, Register8Bit::C) => 0x59,
            Instruction::LdRegister(Register8Bit::E, Register8Bit::D) => 0x5a,
            Instruction::LdRegister(Register8Bit::E, Register8Bit::E) => 0x5b,
            Instruction::LdRegister(Register8Bit::E, Register8Bit::H) => 0x5c,
            Instruction::LdRegister(Register8Bit::E, Register8Bit::L) => 0x5d,
            Instruction::LdRegister(Register8Bit::E, Register8Bit::A) => 0x5f,
            // LD H, X
            Instruction::LdRegister(Register8Bit::H, Register8Bit::B) => 0x60,
            Instruction::LdRegister(Register8Bit::H, Register8Bit::C) => 0x61,
            Instruction::LdRegister(Register8Bit::H, Register8Bit::D) => 0x62,
            Instruction::LdRegister(Register8Bit::H, Register8Bit::E) => 0x63,
            Instruction::LdRegister(Register8Bit::H, Register8Bit::H) => 0x64,
            Instruction::LdRegister(Register8Bit::H, Register8Bit::L) => 0x65,
            Instruction::LdRegister(Register8Bit::H, Register8Bit::A) => 0x67,
            // LD L, X
            Instruction::LdRegister(Register8Bit::L, Register8Bit::B) => 0x68,
            Instruction::LdRegister(Register8Bit::L, Register8Bit::C) => 0x69, // nice 
            Instruction::LdRegister(Register8Bit::L, Register8Bit::D) => 0x6a, 
            Instruction::LdRegister(Register8Bit::L, Register8Bit::E) => 0x6b, 
            Instruction::LdRegister(Register8Bit::L, Register8Bit::H) => 0x6c,
            Instruction::LdRegister(Register8Bit::L, Register8Bit::L) => 0x6d,
            Instruction::LdRegister(Register8Bit::L, Register8Bit::A) => 0x6f,
            // LD A, X
            Instruction::LdRegister(Register8Bit::A, Register8Bit::B) => 0x78,
            Instruction::LdRegister(Register8Bit::A, Register8Bit::C) => 0x79,
            Instruction::LdRegister(Register8Bit::A, Register8Bit::D) => 0x7a,
            Instruction::LdRegister(Register8Bit::A, Register8Bit::E) => 0x7b,
            Instruction::LdRegister(Register8Bit::A, Register8Bit::H) => 0x7c,
            Instruction::LdRegister(Register8Bit::A, Register8Bit::L) => 0x7d,
            Instruction::LdRegister(Register8Bit::A, Register8Bit::A) => 0x7f,
            // LD X, nn
            Instruction::LdImm(Register8Bit::B) => 0x06,
            Instruction::LdImm(Register8Bit::D) => 0x16,
            Instruction::LdImm(Register8Bit::H) => 0x26,
            Instruction::LdImm(Register8Bit::C) => 0x0e,
            Instruction::LdImm(Register8Bit::E) => 0x1e,
            Instruction::LdImm(Register8Bit::L) => 0x2e,
            Instruction::LdImm(Register8Bit::A) => 0x3e,
            // LD (HL), X
            Instruction::LdToMem(Register8Bit::B, Register16Bit::HL) => 0x70,
            Instruction::LdToMem(Register8Bit::C, Register16Bit::HL) => 0x71,
            Instruction::LdToMem(Register8Bit::D, Register16Bit::HL) => 0x72,
            Instruction::LdToMem(Register8Bit::E, Register16Bit::HL) => 0x73,
            Instruction::LdToMem(Register8Bit::H, Register16Bit::HL) => 0x74,
            Instruction::LdToMem(Register8Bit::L, Register16Bit::HL) => 0x75,
            Instruction::LdToMem(Register8Bit::A, Register16Bit::HL) => 0x77,
            // LD X, (HL)
            Instruction::LdFromMem(Register8Bit::B, Register16Bit::HL) => 0x46,
            Instruction::LdFromMem(Register8Bit::C, Register16Bit::HL) => 0x4e,
            Instruction::LdFromMem(Register8Bit::D, Register16Bit::HL) => 0x56,
            Instruction::LdFromMem(Register8Bit::E, Register16Bit::HL) => 0x5e,
            Instruction::LdFromMem(Register8Bit::H, Register16Bit::HL) => 0x66,
            Instruction::LdFromMem(Register8Bit::L, Register16Bit::HL) => 0x6e,
            Instruction::LdFromMem(Register8Bit::A, Register16Bit::HL) => 0x7e,
            // LD A, (XX)
            Instruction::LdToMem(Register8Bit::A, Register16Bit::BC) => 0x02,
            Instruction::LdToMem(Register8Bit::A, Register16Bit::DE) => 0x12,
            Instruction::LdFromMem(Register8Bit::A, Register16Bit::BC) => 0x0a,
            Instruction::LdFromMem(Register8Bit::A, Register16Bit::DE) => 0x1a,
            // LD A, HL(x)crement
            Instruction::LdToMemInc() => 0x22,
            Instruction::LdToMemDec() => 0x32,
            Instruction::LdFromMemInc() => 0x2a,
            Instruction::LdFromMemDec() => 0x3a,
            // LD XX, nn
            Instruction::LdRegister16Imm(Register16Bit::BC) => 0x01,
            Instruction::LdRegister16Imm(Register16Bit::DE) => 0x11,
            Instruction::LdRegister16Imm(Register16Bit::HL) => 0x21,
            Instruction::LdRegister16Imm(Register16Bit::SP) => 0x31,
            // LD (n) A
            Instruction::LdToUpperMem() => 0xe0,
            // LD A (n)
            Instruction::LdFromUpperMem() => 0xf0,
            // JP
            Instruction::JumpAbs(BranchCondition::NZ) => 0xc2,
            Instruction::JumpAbs(BranchCondition::NC) => 0xd2,
            Instruction::JumpAbs(BranchCondition::NONE) => 0xc3,
            Instruction::JumpAbs(BranchCondition::Z) => 0xca,
            Instruction::JumpAbs(BranchCondition::C) => 0xda,
            // JP (HL)
            Instruction::JumpAbsFromReg() => 0xe9,
            // JR
            Instruction::JumpRel(BranchCondition::NZ) => 0x20,
            Instruction::JumpRel(BranchCondition::NC) => 0x30,
            Instruction::JumpRel(BranchCondition::NONE) =>0x18,
            Instruction::JumpRel(BranchCondition::Z) => 0x28,
            Instruction::JumpRel(BranchCondition::C) => 0x38,
            // PUSH (XX)
            Instruction::Push(Register16Bit::BC) => 0xc5,
            Instruction::Push(Register16Bit::DE) => 0xd5,
            Instruction::Push(Register16Bit::HL) => 0xe5,
            Instruction::Push(Register16Bit::AF) => 0xf5,
            // POP (XX)
            Instruction::Pop(Register16Bit::BC) => 0xc1,
            Instruction::Pop(Register16Bit::DE) => 0xd1,
            Instruction::Pop(Register16Bit::HL) => 0xe1,
            Instruction::Pop(Register16Bit::AF) => 0xf1,
            // RET
            Instruction::Ret(BranchCondition::NZ) => 0xc0,
            Instruction::Ret(BranchCondition::NC) => 0xd0,
            Instruction::Ret(BranchCondition::Z) => 0xc8,
            Instruction::Ret(BranchCondition::C) => 0xd8,
            Instruction::Ret(BranchCondition::NONE) => 0xc9,
            // RST
            Instruction::Rst(RstVec::ZERO) => 0xc7,
            Instruction::Rst(RstVec::ONE) => 0xcf,
            Instruction::Rst(RstVec::TWO) => 0xd7,
            Instruction::Rst(RstVec::THREE) => 0xdf,
            Instruction::Rst(RstVec::FOUR) => 0xe7,
            Instruction::Rst(RstVec::FIVE) => 0xef,
            Instruction::Rst(RstVec::SIX) => 0xf7,
            Instruction::Rst(RstVec::SEVEN) => 0xff,
            // CALL
            Instruction::Call(BranchCondition::NZ) => 0xc4,
            Instruction::Call(BranchCondition::NC) => 0xd4,
            Instruction::Call(BranchCondition::Z) => 0xcc,
            Instruction::Call(BranchCondition::C) => 0xdc,
            Instruction::Call(BranchCondition::NONE) => 0xcd,
            // CCF
            Instruction::ToggleCarryFlag() => 0x3f,
            // SCF
            Instruction::SetCarryFlag() => 0x37,
            // CPL
            Instruction::Invert() => 0x2f,
            // STOP
            Instruction::Stop() => 0x10,
            // HALT
            Instruction::Halt() => 0x76,
            // Special dummy instruction for 16 bit instruction calls
            Instruction::CbInstruction() => 0xcb,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CbInstruction {
    Rlc(Register8Bit),
    RlcMem(),
    Rrc(Register8Bit),
    RrcMem(),
    Rl(Register8Bit),
    RlMem(),
    Rr(Register8Bit),
    RrMem(),
    Bit(Register8Bit, usize),
    Sla(Register8Bit),
    SlaMem(),
    Sra(Register8Bit),
    SraMem(),
    Srl(Register8Bit),
    SrlMem(),
    Res(Register8Bit, usize),
    Set(Register8Bit, usize),
    ResMem(usize),
    BitMem(usize),
    SetMem(usize),
    Swap(Register8Bit),
    SwapMem(),
}

impl CbInstruction {
    pub fn from_byte(byte: u8) -> Option<CbInstruction> {
        match byte {
            // RLC n
            0x00 => Some(CbInstruction::Rlc(Register8Bit::B)),
            0x01 => Some(CbInstruction::Rlc(Register8Bit::C)),
            0x02 => Some(CbInstruction::Rlc(Register8Bit::D)),
            0x03 => Some(CbInstruction::Rlc(Register8Bit::E)),
            0x04 => Some(CbInstruction::Rlc(Register8Bit::H)),
            0x05 => Some(CbInstruction::Rlc(Register8Bit::L)),
            0x07 => Some(CbInstruction::Rlc(Register8Bit::A)),
            // RLC (HL)
            0x06 => Some(CbInstruction::RlcMem()),
            // RRC n
            0x08 => Some(CbInstruction::Rrc(Register8Bit::B)),
            0x09 => Some(CbInstruction::Rrc(Register8Bit::C)),
            0x0a => Some(CbInstruction::Rrc(Register8Bit::D)),
            0x0b => Some(CbInstruction::Rrc(Register8Bit::E)),
            0x0c => Some(CbInstruction::Rrc(Register8Bit::H)),
            0x0d => Some(CbInstruction::Rrc(Register8Bit::L)),
            0x0f => Some(CbInstruction::Rrc(Register8Bit::A)),
            // RRC (HL)
            0x0e => Some(CbInstruction::RrcMem()),
            // RL n
            0x10 => Some(CbInstruction::Rl(Register8Bit::B)),
            0x11 => Some(CbInstruction::Rl(Register8Bit::C)),
            0x12 => Some(CbInstruction::Rl(Register8Bit::D)),
            0x13 => Some(CbInstruction::Rl(Register8Bit::E)),
            0x14 => Some(CbInstruction::Rl(Register8Bit::H)),
            0x15 => Some(CbInstruction::Rl(Register8Bit::L)),
            0x17 => Some(CbInstruction::Rl(Register8Bit::A)),
            // RL (HL)
            0x16 => Some(CbInstruction::RlMem()),
            // RR n
            0x18 => Some(CbInstruction::Rr(Register8Bit::B)),
            0x19 => Some(CbInstruction::Rr(Register8Bit::C)),
            0x1a => Some(CbInstruction::Rr(Register8Bit::D)),
            0x1b => Some(CbInstruction::Rr(Register8Bit::E)),
            0x1c => Some(CbInstruction::Rr(Register8Bit::H)),
            0x1d => Some(CbInstruction::Rr(Register8Bit::L)),
            0x1f => Some(CbInstruction::Rr(Register8Bit::A)),
            // RR (HL)
            0x1e => Some(CbInstruction::RrMem()),
            // BIT b,r
            // 0 bit
            0x40 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x41 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x42 => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x43 => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x44 => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x45 => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x47 => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 1 bit
            0x48 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x49 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x4a => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x4b => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x4c => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x4d => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x4f => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 2 bit
            0x50 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x51 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x52 => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x53 => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x54 => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x55 => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x57 => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 3 bit
            0x58 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x59 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x5a => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x5b => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x5c => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x5d => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x5f => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 4 bit
            0x60 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x61 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x62 => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x63 => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x64 => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x65 => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x67 => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 5 bit
            0x68 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x69 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x6a => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x6b => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x6c => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x6d => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x6f => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 6 bit
            0x70 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x71 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x72 => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x73 => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x74 => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x75 => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x77 => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // 7 bit
            0x78 => Some(CbInstruction::Bit(Register8Bit::B, 0)),
            0x79 => Some(CbInstruction::Bit(Register8Bit::C, 0)),
            0x7a => Some(CbInstruction::Bit(Register8Bit::D, 0)),
            0x7b => Some(CbInstruction::Bit(Register8Bit::E, 0)),
            0x7c => Some(CbInstruction::Bit(Register8Bit::H, 0)),
            0x7d => Some(CbInstruction::Bit(Register8Bit::L, 0)),
            0x7f => Some(CbInstruction::Bit(Register8Bit::A, 0)),
            // BIT b,(HL)
            0x46 => Some(CbInstruction::BitMem(0)),
            0x4e => Some(CbInstruction::BitMem(1)),
            0x56 => Some(CbInstruction::BitMem(2)),
            0x5e => Some(CbInstruction::BitMem(3)),
            0x66 => Some(CbInstruction::BitMem(4)),
            0x6e => Some(CbInstruction::BitMem(5)),
            0x76 => Some(CbInstruction::BitMem(6)),
            0x7e => Some(CbInstruction::BitMem(7)),
            // RES b,r
            // SLA n
            0x20 => Some(CbInstruction::Sla(Register8Bit::B)),
            0x21 => Some(CbInstruction::Sla(Register8Bit::C)),
            0x22 => Some(CbInstruction::Sla(Register8Bit::D)),
            0x23 => Some(CbInstruction::Sla(Register8Bit::E)),
            0x24 => Some(CbInstruction::Sla(Register8Bit::H)),
            0x25 => Some(CbInstruction::Sla(Register8Bit::L)),
            0x27 => Some(CbInstruction::Sla(Register8Bit::A)),
            // SLA (HL)
            0x26 => Some(CbInstruction::SlaMem()),
            // SRA n
            0x28 => Some(CbInstruction::Sra(Register8Bit::B)),
            0x29 => Some(CbInstruction::Sra(Register8Bit::C)),
            0x2a => Some(CbInstruction::Sra(Register8Bit::D)),
            0x2b => Some(CbInstruction::Sra(Register8Bit::E)),
            0x2c => Some(CbInstruction::Sra(Register8Bit::H)),
            0x2d => Some(CbInstruction::Sra(Register8Bit::L)),
            0x2f => Some(CbInstruction::Sra(Register8Bit::A)),
            // SRA (HL)
            0x2e => Some(CbInstruction::SraMem()),
            // SRL n
            0x38 => Some(CbInstruction::Srl(Register8Bit::B)),
            0x39 => Some(CbInstruction::Srl(Register8Bit::C)),
            0x3a => Some(CbInstruction::Srl(Register8Bit::D)),
            0x3b => Some(CbInstruction::Srl(Register8Bit::E)),
            0x3c => Some(CbInstruction::Srl(Register8Bit::H)),
            0x3d => Some(CbInstruction::Srl(Register8Bit::L)),
            0x3f => Some(CbInstruction::Srl(Register8Bit::A)),
            // SRL n (HL)
            0x3e => Some(CbInstruction::SrlMem()),
            // RES n
            // 0 bit
            0x80 => Some(CbInstruction::Res(Register8Bit::B, 0)),
            0x81 => Some(CbInstruction::Res(Register8Bit::C, 0)),
            0x82 => Some(CbInstruction::Res(Register8Bit::D, 0)),
            0x83 => Some(CbInstruction::Res(Register8Bit::E, 0)),
            0x84 => Some(CbInstruction::Res(Register8Bit::H, 0)),
            0x85 => Some(CbInstruction::Res(Register8Bit::L, 0)),
            0x87 => Some(CbInstruction::Res(Register8Bit::A, 0)),
            // 1 bit
            0x88 => Some(CbInstruction::Res(Register8Bit::B, 1)),
            0x89 => Some(CbInstruction::Res(Register8Bit::C, 1)),
            0x8a => Some(CbInstruction::Res(Register8Bit::D, 1)),
            0x8b => Some(CbInstruction::Res(Register8Bit::E, 1)),
            0x8c => Some(CbInstruction::Res(Register8Bit::H, 1)),
            0x8d => Some(CbInstruction::Res(Register8Bit::L, 1)),
            0x8f => Some(CbInstruction::Res(Register8Bit::A, 1)),
            // 2 bit
            0x90 => Some(CbInstruction::Res(Register8Bit::B, 2)),
            0x91 => Some(CbInstruction::Res(Register8Bit::C, 2)),
            0x92 => Some(CbInstruction::Res(Register8Bit::D, 2)),
            0x93 => Some(CbInstruction::Res(Register8Bit::E, 2)),
            0x94 => Some(CbInstruction::Res(Register8Bit::H, 2)),
            0x95 => Some(CbInstruction::Res(Register8Bit::L, 2)),
            0x97 => Some(CbInstruction::Res(Register8Bit::A, 2)),
            // 3 bit
            0x98 => Some(CbInstruction::Res(Register8Bit::B, 3)),
            0x99 => Some(CbInstruction::Res(Register8Bit::C, 3)),
            0x9a => Some(CbInstruction::Res(Register8Bit::D, 3)),
            0x9b => Some(CbInstruction::Res(Register8Bit::E, 3)),
            0x9c => Some(CbInstruction::Res(Register8Bit::H, 3)),
            0x9d => Some(CbInstruction::Res(Register8Bit::L, 3)),
            0x9f => Some(CbInstruction::Res(Register8Bit::A, 3)),
            // 4 bit
            0xa0 => Some(CbInstruction::Res(Register8Bit::B, 4)),
            0xa1 => Some(CbInstruction::Res(Register8Bit::C, 4)),
            0xa2 => Some(CbInstruction::Res(Register8Bit::D, 4)),
            0xa3 => Some(CbInstruction::Res(Register8Bit::E, 4)),
            0xa4 => Some(CbInstruction::Res(Register8Bit::H, 4)),
            0xa5 => Some(CbInstruction::Res(Register8Bit::L, 4)),
            0xa7 => Some(CbInstruction::Res(Register8Bit::A, 4)),
            // 5 bit
            0xa8 => Some(CbInstruction::Res(Register8Bit::B, 5)),
            0xa9 => Some(CbInstruction::Res(Register8Bit::C, 5)),
            0xaa => Some(CbInstruction::Res(Register8Bit::D, 5)),
            0xab => Some(CbInstruction::Res(Register8Bit::E, 5)),
            0xac => Some(CbInstruction::Res(Register8Bit::H, 5)),
            0xad => Some(CbInstruction::Res(Register8Bit::L, 5)),
            0xaf => Some(CbInstruction::Res(Register8Bit::A, 5)),
            // 6 bit
            0xb0 => Some(CbInstruction::Res(Register8Bit::B, 6)),
            0xb1 => Some(CbInstruction::Res(Register8Bit::C, 6)),
            0xb2 => Some(CbInstruction::Res(Register8Bit::D, 6)),
            0xb3 => Some(CbInstruction::Res(Register8Bit::E, 6)),
            0xb4 => Some(CbInstruction::Res(Register8Bit::H, 6)),
            0xb5 => Some(CbInstruction::Res(Register8Bit::L, 6)),
            0xb7 => Some(CbInstruction::Res(Register8Bit::A, 6)),
            // 7 bit
            0xb8 => Some(CbInstruction::Res(Register8Bit::B, 7)),
            0xb9 => Some(CbInstruction::Res(Register8Bit::C, 7)),
            0xba => Some(CbInstruction::Res(Register8Bit::D, 7)),
            0xbb => Some(CbInstruction::Res(Register8Bit::E, 7)),
            0xbc => Some(CbInstruction::Res(Register8Bit::H, 7)),
            0xbd => Some(CbInstruction::Res(Register8Bit::L, 7)),
            0xbf => Some(CbInstruction::Res(Register8Bit::A, 7)),
            // RES b,(HL)
            0x86 => Some(CbInstruction::ResMem(0)),
            0x8e => Some(CbInstruction::ResMem(1)),
            0x96 => Some(CbInstruction::ResMem(2)),
            0x9e => Some(CbInstruction::ResMem(3)),
            0xa6 => Some(CbInstruction::ResMem(4)),
            0xae => Some(CbInstruction::ResMem(5)),
            0xb6 => Some(CbInstruction::ResMem(6)),
            0xbe => Some(CbInstruction::ResMem(7)),
            // SET b,r
            // 0 bit
            0xc0 => Some(CbInstruction::Set(Register8Bit::B, 0)),
            0xc1 => Some(CbInstruction::Set(Register8Bit::C, 0)),
            0xc2 => Some(CbInstruction::Set(Register8Bit::D, 0)),
            0xc3 => Some(CbInstruction::Set(Register8Bit::E, 0)),
            0xc4 => Some(CbInstruction::Set(Register8Bit::H, 0)),
            0xc5 => Some(CbInstruction::Set(Register8Bit::L, 0)),
            0xc7 => Some(CbInstruction::Set(Register8Bit::A, 0)),
            // 1 bit
            0xc8 => Some(CbInstruction::Set(Register8Bit::B, 1)),
            0xc9 => Some(CbInstruction::Set(Register8Bit::C, 1)),
            0xca => Some(CbInstruction::Set(Register8Bit::D, 1)),
            0xcb => Some(CbInstruction::Set(Register8Bit::E, 1)),
            0xcc => Some(CbInstruction::Set(Register8Bit::H, 1)),
            0xcd => Some(CbInstruction::Set(Register8Bit::L, 1)),
            0xcf => Some(CbInstruction::Set(Register8Bit::A, 1)),
            // 2 bit
            0xd0 => Some(CbInstruction::Set(Register8Bit::B, 2)),
            0xd1 => Some(CbInstruction::Set(Register8Bit::C, 2)),
            0xd2 => Some(CbInstruction::Set(Register8Bit::D, 2)),
            0xd3 => Some(CbInstruction::Set(Register8Bit::E, 2)),
            0xd4 => Some(CbInstruction::Set(Register8Bit::H, 2)),
            0xd5 => Some(CbInstruction::Set(Register8Bit::L, 2)),
            0xd7 => Some(CbInstruction::Set(Register8Bit::A, 2)),
            // 3 bit
            0xd8 => Some(CbInstruction::Set(Register8Bit::B, 3)),
            0xd9 => Some(CbInstruction::Set(Register8Bit::C, 3)),
            0xda => Some(CbInstruction::Set(Register8Bit::D, 3)),
            0xdb => Some(CbInstruction::Set(Register8Bit::E, 3)),
            0xdc => Some(CbInstruction::Set(Register8Bit::H, 3)),
            0xdd => Some(CbInstruction::Set(Register8Bit::L, 3)),
            0xdf => Some(CbInstruction::Set(Register8Bit::A, 3)),
            // 4 bit
            0xe0 => Some(CbInstruction::Set(Register8Bit::B, 4)),
            0xe1 => Some(CbInstruction::Set(Register8Bit::C, 4)),
            0xe2 => Some(CbInstruction::Set(Register8Bit::D, 4)),
            0xe3 => Some(CbInstruction::Set(Register8Bit::E, 4)),
            0xe4 => Some(CbInstruction::Set(Register8Bit::H, 4)),
            0xe5 => Some(CbInstruction::Set(Register8Bit::L, 4)),
            0xe7 => Some(CbInstruction::Set(Register8Bit::A, 4)),
            // 5 bit
            0xe8 => Some(CbInstruction::Set(Register8Bit::B, 5)),
            0xe9 => Some(CbInstruction::Set(Register8Bit::C, 5)),
            0xea => Some(CbInstruction::Set(Register8Bit::D, 5)),
            0xeb => Some(CbInstruction::Set(Register8Bit::E, 5)),
            0xec => Some(CbInstruction::Set(Register8Bit::H, 5)),
            0xed => Some(CbInstruction::Set(Register8Bit::L, 5)),
            0xef => Some(CbInstruction::Set(Register8Bit::A, 5)),
            // 6 bit
            0xf0 => Some(CbInstruction::Set(Register8Bit::B, 6)),
            0xf1 => Some(CbInstruction::Set(Register8Bit::C, 6)),
            0xf2 => Some(CbInstruction::Set(Register8Bit::D, 6)),
            0xf3 => Some(CbInstruction::Set(Register8Bit::E, 6)),
            0xf4 => Some(CbInstruction::Set(Register8Bit::H, 6)),
            0xf5 => Some(CbInstruction::Set(Register8Bit::L, 6)),
            0xf7 => Some(CbInstruction::Set(Register8Bit::A, 6)),
            // 7 bit
            0xf8 => Some(CbInstruction::Set(Register8Bit::B, 7)),
            0xf9 => Some(CbInstruction::Set(Register8Bit::C, 7)),
            0xfa => Some(CbInstruction::Set(Register8Bit::D, 7)),
            0xfb => Some(CbInstruction::Set(Register8Bit::E, 7)),
            0xfc => Some(CbInstruction::Set(Register8Bit::H, 7)),
            0xfd => Some(CbInstruction::Set(Register8Bit::L, 7)),
            0xff => Some(CbInstruction::Set(Register8Bit::A, 7)),
            // SET (HL)
            0xc6 => Some(CbInstruction::SetMem(0)),
            0xce => Some(CbInstruction::SetMem(1)),
            0xd6 => Some(CbInstruction::SetMem(2)),
            0xde => Some(CbInstruction::SetMem(3)),
            0xe6 => Some(CbInstruction::SetMem(4)),
            0xee => Some(CbInstruction::SetMem(5)),
            0xf6 => Some(CbInstruction::SetMem(6)),
            0xfe => Some(CbInstruction::SetMem(7)),
            // SWAP n
            0x30 => Some(CbInstruction::Swap(Register8Bit::B)),
            0x31 => Some(CbInstruction::Swap(Register8Bit::C)),
            0x32 => Some(CbInstruction::Swap(Register8Bit::D)),
            0x33 => Some(CbInstruction::Swap(Register8Bit::E)),
            0x34 => Some(CbInstruction::Swap(Register8Bit::H)),
            0x35 => Some(CbInstruction::Swap(Register8Bit::L)),
            0x37 => Some(CbInstruction::Swap(Register8Bit::A)),
            // SWAP (HL)
            0x36 => Some(CbInstruction::SwapMem()),
            _ => panic!("Invalid instruction"),
        }
    }
    pub fn as_byte(self) -> u8 {
        match self {
            // RLC n
            CbInstruction::Rlc(Register8Bit::B) => 0x00,
            CbInstruction::Rlc(Register8Bit::C) => 0x01,
            CbInstruction::Rlc(Register8Bit::D) => 0x02,
            CbInstruction::Rlc(Register8Bit::E) => 0x03,
            CbInstruction::Rlc(Register8Bit::H) => 0x04,
            CbInstruction::Rlc(Register8Bit::L) => 0x05,
            CbInstruction::Rlc(Register8Bit::A) => 0x07,
            // RLC (HL)
            CbInstruction::RlcMem() => 0x06,
            // RRC n
            CbInstruction::Rrc(Register8Bit::B) => 0x08,
            CbInstruction::Rrc(Register8Bit::C) => 0x09,
            CbInstruction::Rrc(Register8Bit::D) => 0x0a,
            CbInstruction::Rrc(Register8Bit::E) => 0x0b,
            CbInstruction::Rrc(Register8Bit::H) => 0x0c,
            CbInstruction::Rrc(Register8Bit::L) => 0x0d,
            CbInstruction::Rrc(Register8Bit::A) => 0x0f,
            // RRC (HL)
            CbInstruction::RrcMem() => 0x0e,
            // RL n
            CbInstruction::Rl(Register8Bit::B) => 0x10,
            CbInstruction::Rl(Register8Bit::C) => 0x11,
            CbInstruction::Rl(Register8Bit::D) => 0x12,
            CbInstruction::Rl(Register8Bit::E) => 0x13,
            CbInstruction::Rl(Register8Bit::H) => 0x14,
            CbInstruction::Rl(Register8Bit::L) => 0x15,
            CbInstruction::Rl(Register8Bit::A) => 0x17,
            // RL (HL)
            CbInstruction::RlMem() => 0x16,
            // RR n
            CbInstruction::Rr(Register8Bit::B) => 0x18,
            CbInstruction::Rr(Register8Bit::C) => 0x19,
            CbInstruction::Rr(Register8Bit::D) => 0x1a,
            CbInstruction::Rr(Register8Bit::E) => 0x1b,
            CbInstruction::Rr(Register8Bit::H) => 0x1c,
            CbInstruction::Rr(Register8Bit::L) => 0x1d,
            CbInstruction::Rr(Register8Bit::A) => 0x1f,
            // RR (HL)
            CbInstruction::RrMem() => 0x1e,
            // SLA n
            CbInstruction::Sla(Register8Bit::B) => 0x20,
            CbInstruction::Sla(Register8Bit::C) => 0x21,
            CbInstruction::Sla(Register8Bit::D) => 0x22,
            CbInstruction::Sla(Register8Bit::E) => 0x23,
            CbInstruction::Sla(Register8Bit::H) => 0x24,
            CbInstruction::Sla(Register8Bit::L) => 0x25,
            CbInstruction::Sla(Register8Bit::A) => 0x27,
            // SLA (HL)
            CbInstruction::SlaMem() => 0x26,
            // SRA n
            CbInstruction::Sra(Register8Bit::B) => 0x28,
            CbInstruction::Sra(Register8Bit::C) => 0x29,
            CbInstruction::Sra(Register8Bit::D) => 0x2a,
            CbInstruction::Sra(Register8Bit::E) => 0x2b,
            CbInstruction::Sra(Register8Bit::H) => 0x2c,
            CbInstruction::Sra(Register8Bit::L) => 0x2d,
            CbInstruction::Sra(Register8Bit::A) => 0x2f,
            // SRA (HL)
            CbInstruction::SraMem() => 0x2e,
            // SRL n
            CbInstruction::Srl(Register8Bit::B) => 0x38,
            CbInstruction::Srl(Register8Bit::C) => 0x39,
            CbInstruction::Srl(Register8Bit::D) => 0x3a,
            CbInstruction::Srl(Register8Bit::E) => 0x3b,
            CbInstruction::Srl(Register8Bit::H) => 0x3c,
            CbInstruction::Srl(Register8Bit::L) => 0x3d,
            CbInstruction::Srl(Register8Bit::A) => 0x3f,
            // SRL (HL)
            CbInstruction::SrlMem() => 0x3e,
            // RES b,r
            // 0 bit
            CbInstruction::Res(Register8Bit::B, 0) => 0x80,
            CbInstruction::Res(Register8Bit::C, 0) => 0x81,
            CbInstruction::Res(Register8Bit::D, 0) => 0x82,
            CbInstruction::Res(Register8Bit::E, 0) => 0x83,
            CbInstruction::Res(Register8Bit::H, 0) => 0x84,
            CbInstruction::Res(Register8Bit::L, 0) => 0x85,
            CbInstruction::Res(Register8Bit::A, 0) => 0x87,
            // 1 bit
            CbInstruction::Res(Register8Bit::B, 1) => 0x88,
            CbInstruction::Res(Register8Bit::C, 1) => 0x89,
            CbInstruction::Res(Register8Bit::D, 1) => 0x8a,
            CbInstruction::Res(Register8Bit::E, 1) => 0x8b,
            CbInstruction::Res(Register8Bit::H, 1) => 0x8c,
            CbInstruction::Res(Register8Bit::L, 1) => 0x8d,
            CbInstruction::Res(Register8Bit::A, 1) => 0x8f,
            // 2 bit
            CbInstruction::Res(Register8Bit::B, 2) => 0x90,
            CbInstruction::Res(Register8Bit::C, 2) => 0x91,
            CbInstruction::Res(Register8Bit::D, 2) => 0x92,
            CbInstruction::Res(Register8Bit::E, 2) => 0x93,
            CbInstruction::Res(Register8Bit::H, 2) => 0x94,
            CbInstruction::Res(Register8Bit::L, 2) => 0x95,
            CbInstruction::Res(Register8Bit::A, 2) => 0x97,
            // 3 bit
            CbInstruction::Res(Register8Bit::B, 3) => 0x98,
            CbInstruction::Res(Register8Bit::C, 3) => 0x99,
            CbInstruction::Res(Register8Bit::D, 3) => 0x9a,
            CbInstruction::Res(Register8Bit::E, 3) => 0x9b,
            CbInstruction::Res(Register8Bit::H, 3) => 0x9c,
            CbInstruction::Res(Register8Bit::L, 3) => 0x9d,
            CbInstruction::Res(Register8Bit::A, 3) => 0x9f,
            // 4 bit
            CbInstruction::Res(Register8Bit::B, 4) => 0xa0,
            CbInstruction::Res(Register8Bit::C, 4) => 0xa1,
            CbInstruction::Res(Register8Bit::D, 4) => 0xa2,
            CbInstruction::Res(Register8Bit::E, 4) => 0xa3,
            CbInstruction::Res(Register8Bit::H, 4) => 0xa4,
            CbInstruction::Res(Register8Bit::L, 4) => 0xa5,
            CbInstruction::Res(Register8Bit::A, 4) => 0xa7,
            // 5 bit
            CbInstruction::Res(Register8Bit::B, 5) => 0xa8,
            CbInstruction::Res(Register8Bit::C, 5) => 0xa9,
            CbInstruction::Res(Register8Bit::D, 5) => 0xaa,
            CbInstruction::Res(Register8Bit::E, 5) => 0xab,
            CbInstruction::Res(Register8Bit::H, 5) => 0xac,
            CbInstruction::Res(Register8Bit::L, 5) => 0xad,
            CbInstruction::Res(Register8Bit::A, 5) => 0xaf,
            // 6 bit
            CbInstruction::Res(Register8Bit::B, 6) => 0xb0,
            CbInstruction::Res(Register8Bit::C, 6) => 0xb1,
            CbInstruction::Res(Register8Bit::D, 6) => 0xb2,
            CbInstruction::Res(Register8Bit::E, 6) => 0xb3,
            CbInstruction::Res(Register8Bit::H, 6) => 0xb4,
            CbInstruction::Res(Register8Bit::L, 6) => 0xb5,
            CbInstruction::Res(Register8Bit::A, 6) => 0xb7,
            // 7 bit
            CbInstruction::Res(Register8Bit::B, 7) => 0xb8,
            CbInstruction::Res(Register8Bit::C, 7) => 0xb9,
            CbInstruction::Res(Register8Bit::D, 7) => 0xba,
            CbInstruction::Res(Register8Bit::E, 7) => 0xbb,
            CbInstruction::Res(Register8Bit::H, 7) => 0xbc,
            CbInstruction::Res(Register8Bit::L, 7) => 0xbd,
            CbInstruction::Res(Register8Bit::A, 7) => 0xbf,
            // RES b,(HL)
            CbInstruction::ResMem(0) => 0x86,
            CbInstruction::ResMem(1) => 0x8e,
            CbInstruction::ResMem(2) => 0x96,
            CbInstruction::ResMem(3) => 0x9e,
            CbInstruction::ResMem(4) => 0xa6,
            CbInstruction::ResMem(5) => 0xae,
            CbInstruction::ResMem(6) => 0xb6,
            CbInstruction::ResMem(7) => 0xbe,
            // BIT b,r
            // 0 bit
            CbInstruction::Bit(Register8Bit::B, 0) => 0x40,
            CbInstruction::Bit(Register8Bit::C, 0) => 0x41,
            CbInstruction::Bit(Register8Bit::D, 0) => 0x42,
            CbInstruction::Bit(Register8Bit::E, 0) => 0x43,
            CbInstruction::Bit(Register8Bit::H, 0) => 0x44,
            CbInstruction::Bit(Register8Bit::L, 0) => 0x45,
            CbInstruction::Bit(Register8Bit::A, 0) => 0x47,
            // 1 bit
            CbInstruction::Bit(Register8Bit::B, 1) => 0x48,
            CbInstruction::Bit(Register8Bit::C, 1) => 0x49,
            CbInstruction::Bit(Register8Bit::D, 1) => 0x4a,
            CbInstruction::Bit(Register8Bit::E, 1) => 0x4b,
            CbInstruction::Bit(Register8Bit::H, 1) => 0x4c,
            CbInstruction::Bit(Register8Bit::L, 1) => 0x4d,
            CbInstruction::Bit(Register8Bit::A, 1) => 0x4f,
            // 2 bit
            CbInstruction::Bit(Register8Bit::B, 2) => 0x50,
            CbInstruction::Bit(Register8Bit::C, 2) => 0x51,
            CbInstruction::Bit(Register8Bit::D, 2) => 0x52,
            CbInstruction::Bit(Register8Bit::E, 2) => 0x53,
            CbInstruction::Bit(Register8Bit::H, 2) => 0x54,
            CbInstruction::Bit(Register8Bit::L, 2) => 0x55,
            CbInstruction::Bit(Register8Bit::A, 2) => 0x57,
            // 3 bit
            CbInstruction::Bit(Register8Bit::B, 3) => 0x58,
            CbInstruction::Bit(Register8Bit::C, 3) => 0x59,
            CbInstruction::Bit(Register8Bit::D, 3) => 0x5a,
            CbInstruction::Bit(Register8Bit::E, 3) => 0x5b,
            CbInstruction::Bit(Register8Bit::H, 3) => 0x5c,
            CbInstruction::Bit(Register8Bit::L, 3) => 0x5d,
            CbInstruction::Bit(Register8Bit::A, 3) => 0x5f,
            // 4 bit
            CbInstruction::Bit(Register8Bit::B, 4) => 0x60,
            CbInstruction::Bit(Register8Bit::C, 4) => 0x61,
            CbInstruction::Bit(Register8Bit::D, 4) => 0x62,
            CbInstruction::Bit(Register8Bit::E, 4) => 0x63,
            CbInstruction::Bit(Register8Bit::H, 4) => 0x64,
            CbInstruction::Bit(Register8Bit::L, 4) => 0x65,
            CbInstruction::Bit(Register8Bit::A, 4) => 0x67,
            // 5 bit
            CbInstruction::Bit(Register8Bit::B, 5) => 0x68,
            CbInstruction::Bit(Register8Bit::C, 5) => 0x69,
            CbInstruction::Bit(Register8Bit::D, 5) => 0x6a,
            CbInstruction::Bit(Register8Bit::E, 5) => 0x6b,
            CbInstruction::Bit(Register8Bit::H, 5) => 0x6c,
            CbInstruction::Bit(Register8Bit::L, 5) => 0x6d,
            CbInstruction::Bit(Register8Bit::A, 5) => 0x6f,
            // 6 bit
            CbInstruction::Bit(Register8Bit::B, 6) => 0x70,
            CbInstruction::Bit(Register8Bit::C, 6) => 0x71,
            CbInstruction::Bit(Register8Bit::D, 6) => 0x72,
            CbInstruction::Bit(Register8Bit::E, 6) => 0x73,
            CbInstruction::Bit(Register8Bit::H, 6) => 0x74,
            CbInstruction::Bit(Register8Bit::L, 6) => 0x75,
            CbInstruction::Bit(Register8Bit::A, 6) => 0x77,
            // 7 bit
            CbInstruction::Bit(Register8Bit::B, 7) => 0x78,
            CbInstruction::Bit(Register8Bit::C, 7) => 0x79,
            CbInstruction::Bit(Register8Bit::D, 7) => 0x7a,
            CbInstruction::Bit(Register8Bit::E, 7) => 0x7b,
            CbInstruction::Bit(Register8Bit::H, 7) => 0x7c,
            CbInstruction::Bit(Register8Bit::L, 7) => 0x7d,
            CbInstruction::Bit(Register8Bit::A, 7) => 0x7f,
            // BIT b,(HL)
            CbInstruction::BitMem(0) => 0x46,
            CbInstruction::BitMem(1) => 0x4e,
            CbInstruction::BitMem(2) => 0x56,
            CbInstruction::BitMem(3) => 0x5e,
            CbInstruction::BitMem(4) => 0x66,
            CbInstruction::BitMem(5) => 0x6e,
            CbInstruction::BitMem(6) => 0x76,
            CbInstruction::BitMem(7) => 0x7e,
            // SET b,n
            // 0 bit
            CbInstruction::Set(Register8Bit::B, 0) => 0xc0,
            CbInstruction::Set(Register8Bit::C, 0) => 0xc1,
            CbInstruction::Set(Register8Bit::D, 0) => 0xc2,
            CbInstruction::Set(Register8Bit::E, 0) => 0xc3,
            CbInstruction::Set(Register8Bit::H, 0) => 0xc4,
            CbInstruction::Set(Register8Bit::L, 0) => 0xc5,
            CbInstruction::Set(Register8Bit::A, 0) => 0xc7,
            // 1 bit
            CbInstruction::Set(Register8Bit::B, 1) => 0xc8,
            CbInstruction::Set(Register8Bit::C, 1) => 0xc9,
            CbInstruction::Set(Register8Bit::D, 1) => 0xca,
            CbInstruction::Set(Register8Bit::E, 1) => 0xcb,
            CbInstruction::Set(Register8Bit::H, 1) => 0xcc,
            CbInstruction::Set(Register8Bit::L, 1) => 0xcd,
            CbInstruction::Set(Register8Bit::A, 1) => 0xcf,
            // 2 bit
            CbInstruction::Set(Register8Bit::B, 2) => 0xd0,
            CbInstruction::Set(Register8Bit::C, 2) => 0xd1,
            CbInstruction::Set(Register8Bit::D, 2) => 0xd2,
            CbInstruction::Set(Register8Bit::E, 2) => 0xd3,
            CbInstruction::Set(Register8Bit::H, 2) => 0xd4,
            CbInstruction::Set(Register8Bit::L, 2) => 0xd5,
            CbInstruction::Set(Register8Bit::A, 2) => 0xd7,
            // 3 bit
            CbInstruction::Set(Register8Bit::B, 3) => 0xd8,
            CbInstruction::Set(Register8Bit::C, 3) => 0xd9,
            CbInstruction::Set(Register8Bit::D, 3) => 0xda,
            CbInstruction::Set(Register8Bit::E, 3) => 0xdb,
            CbInstruction::Set(Register8Bit::H, 3) => 0xdc,
            CbInstruction::Set(Register8Bit::L, 3) => 0xdd,
            CbInstruction::Set(Register8Bit::A, 3) => 0xdf,
            // 4 bit
            CbInstruction::Set(Register8Bit::B, 4) => 0xe0,
            CbInstruction::Set(Register8Bit::C, 4) => 0xe1,
            CbInstruction::Set(Register8Bit::D, 4) => 0xe2,
            CbInstruction::Set(Register8Bit::E, 4) => 0xe3,
            CbInstruction::Set(Register8Bit::H, 4) => 0xe4,
            CbInstruction::Set(Register8Bit::L, 4) => 0xe5,
            CbInstruction::Set(Register8Bit::A, 4) => 0xe7,
            // 5 bit
            CbInstruction::Set(Register8Bit::B, 5) => 0xe8,
            CbInstruction::Set(Register8Bit::C, 5) => 0xe9,
            CbInstruction::Set(Register8Bit::D, 5) => 0xea,
            CbInstruction::Set(Register8Bit::E, 5) => 0xeb,
            CbInstruction::Set(Register8Bit::H, 5) => 0xec,
            CbInstruction::Set(Register8Bit::L, 5) => 0xed,
            CbInstruction::Set(Register8Bit::A, 5) => 0xef,
            // 6 bit
            CbInstruction::Set(Register8Bit::B, 6) => 0xf0,
            CbInstruction::Set(Register8Bit::C, 6) => 0xf1,
            CbInstruction::Set(Register8Bit::D, 6) => 0xf2,
            CbInstruction::Set(Register8Bit::E, 6) => 0xf3,
            CbInstruction::Set(Register8Bit::H, 6) => 0xf4,
            CbInstruction::Set(Register8Bit::L, 6) => 0xf5,
            CbInstruction::Set(Register8Bit::A, 6) => 0xf7,
            // 7 bit
            CbInstruction::Set(Register8Bit::B, 7) => 0xf8,
            CbInstruction::Set(Register8Bit::C, 7) => 0xf9,
            CbInstruction::Set(Register8Bit::D, 7) => 0xfa,
            CbInstruction::Set(Register8Bit::E, 7) => 0xfb,
            CbInstruction::Set(Register8Bit::H, 7) => 0xfc,
            CbInstruction::Set(Register8Bit::L, 7) => 0xfd,
            CbInstruction::Set(Register8Bit::A, 7) => 0xff,
            // SET b,(HL)
            CbInstruction::SetMem(0) => 0xc6,
            CbInstruction::SetMem(1) => 0xce,
            CbInstruction::SetMem(2) => 0xd6,
            CbInstruction::SetMem(3) => 0xde,
            CbInstruction::SetMem(4) => 0xe6,
            CbInstruction::SetMem(5) => 0xee,
            CbInstruction::SetMem(6) => 0xf6,
            CbInstruction::SetMem(7) => 0xfe,
            // SWAP n
            CbInstruction::Swap(Register8Bit::B) => 0x30,
            CbInstruction::Swap(Register8Bit::C) => 0x31,
            CbInstruction::Swap(Register8Bit::D) => 0x32,
            CbInstruction::Swap(Register8Bit::E) => 0x33,
            CbInstruction::Swap(Register8Bit::H) => 0x34,
            CbInstruction::Swap(Register8Bit::L) => 0x35,
            CbInstruction::Swap(Register8Bit::A) => 0x37,
            // SWAP (HL)
            CbInstruction::SwapMem() => 0x36,
            _ => panic!("Invalid instruction"),
        }
    }
}
