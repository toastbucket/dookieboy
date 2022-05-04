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
pub enum Instruction8Bit {
    Noop(),
    Inc(Register8Bit),
    Dec(Register8Bit),
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
    Add(Register8Bit),
    AddImm(),
    AddFromMem(), // always uses HL
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
    JumpAbs(BranchCondition),
    JumpAbsFromReg(), // always uses HL
    JumpRel(BranchCondition),
    Push(Register16Bit),
    Pop(Register16Bit),
    Ret(BranchCondition),
    Rst(RstVec),
    Call(BranchCondition),
    Call16BitInstruction(), // Special dummy instruction for 16 bit instruction calls
}

impl Instruction8Bit {
    pub fn from_byte(byte: u8) -> Option<Instruction8Bit> {
        match byte {
            // NOOP
            0x00 => Some(Instruction8Bit::Noop()),
            // INC r
            0x04 => Some(Instruction8Bit::Inc(Register8Bit::B)),
            0x0c => Some(Instruction8Bit::Inc(Register8Bit::C)),
            0x14 => Some(Instruction8Bit::Inc(Register8Bit::D)),
            0x1c => Some(Instruction8Bit::Inc(Register8Bit::E)),
            0x24 => Some(Instruction8Bit::Inc(Register8Bit::H)),
            0x2c => Some(Instruction8Bit::Inc(Register8Bit::L)),
            0x3c => Some(Instruction8Bit::Inc(Register8Bit::A)),
            // DEC r
            0x05 => Some(Instruction8Bit::Dec(Register8Bit::B)),
            0x0d => Some(Instruction8Bit::Dec(Register8Bit::C)),
            0x15 => Some(Instruction8Bit::Dec(Register8Bit::D)),
            0x1d => Some(Instruction8Bit::Dec(Register8Bit::E)),
            0x25 => Some(Instruction8Bit::Dec(Register8Bit::H)),
            0x2d => Some(Instruction8Bit::Dec(Register8Bit::L)),
            0x3d => Some(Instruction8Bit::Dec(Register8Bit::A)),
            // AND r
            0xa0 => Some(Instruction8Bit::And(Register8Bit::B)),
            0xa1 => Some(Instruction8Bit::And(Register8Bit::C)),
            0xa2 => Some(Instruction8Bit::And(Register8Bit::D)),
            0xa3 => Some(Instruction8Bit::And(Register8Bit::E)),
            0xa4 => Some(Instruction8Bit::And(Register8Bit::H)),
            0xa5 => Some(Instruction8Bit::And(Register8Bit::L)),
            0xa7 => Some(Instruction8Bit::And(Register8Bit::A)),
            // AND (HL)
            0xa6 => Some(Instruction8Bit::AndFromMem()),
            // AND n
            0xe6 => Some(Instruction8Bit::AndImm()),
            // OR A,r
            0xb0 => Some(Instruction8Bit::Or(Register8Bit::B)),
            0xb1 => Some(Instruction8Bit::Or(Register8Bit::C)),
            0xb2 => Some(Instruction8Bit::Or(Register8Bit::D)),
            0xb3 => Some(Instruction8Bit::Or(Register8Bit::E)),
            0xb4 => Some(Instruction8Bit::Or(Register8Bit::H)),
            0xb5 => Some(Instruction8Bit::Or(Register8Bit::L)),
            0xb7 => Some(Instruction8Bit::Or(Register8Bit::A)),
            // OR (HL)
            0xb6 => Some(Instruction8Bit::OrFromMem()),
            // OR n
            0xf6 => Some(Instruction8Bit::OrImm()),
            // XOR A,r
            0xa8 => Some(Instruction8Bit::Xor(Register8Bit::B)),
            0xa9 => Some(Instruction8Bit::Xor(Register8Bit::C)),
            0xaa => Some(Instruction8Bit::Xor(Register8Bit::D)),
            0xab => Some(Instruction8Bit::Xor(Register8Bit::E)),
            0xac => Some(Instruction8Bit::Xor(Register8Bit::H)),
            0xad => Some(Instruction8Bit::Xor(Register8Bit::L)),
            0xaf => Some(Instruction8Bit::Xor(Register8Bit::A)),
            // XOR (HL)
            0xae => Some(Instruction8Bit::XorFromMem()),
            // XOR n
            0xee => Some(Instruction8Bit::XorImm()),
            // CP A,r
            0xb8 => Some(Instruction8Bit::Cp(Register8Bit::B)),
            0xb9 => Some(Instruction8Bit::Cp(Register8Bit::C)),
            0xba => Some(Instruction8Bit::Cp(Register8Bit::D)),
            0xbb => Some(Instruction8Bit::Cp(Register8Bit::E)),
            0xbc => Some(Instruction8Bit::Cp(Register8Bit::H)),
            0xbd => Some(Instruction8Bit::Cp(Register8Bit::L)),
            0xbf => Some(Instruction8Bit::Cp(Register8Bit::A)),
            // CP (HL)
            0xbe => Some(Instruction8Bit::CpFromMem()),
            // CP n
            0xfe => Some(Instruction8Bit::CpImm()),
            // ADD A,r
            0x80 => Some(Instruction8Bit::Add(Register8Bit::B)),
            0x81 => Some(Instruction8Bit::Add(Register8Bit::C)),
            0x82 => Some(Instruction8Bit::Add(Register8Bit::D)),
            0x83 => Some(Instruction8Bit::Add(Register8Bit::E)),
            0x84 => Some(Instruction8Bit::Add(Register8Bit::H)),
            0x85 => Some(Instruction8Bit::Add(Register8Bit::L)),
            0x87 => Some(Instruction8Bit::Add(Register8Bit::A)),
            // ADD A,d8
            0xc6 => Some(Instruction8Bit::AddImm()),
            // ADD A,(HL)
            0x86 => Some(Instruction8Bit::AddFromMem()),
            // ADC A,r
            0x88 => Some(Instruction8Bit::Adc(Register8Bit::B)),
            0x89 => Some(Instruction8Bit::Adc(Register8Bit::C)),
            0x8a => Some(Instruction8Bit::Adc(Register8Bit::D)),
            0x8b => Some(Instruction8Bit::Adc(Register8Bit::E)),
            0x8c => Some(Instruction8Bit::Adc(Register8Bit::H)),
            0x8d => Some(Instruction8Bit::Adc(Register8Bit::L)),
            0x8f => Some(Instruction8Bit::Adc(Register8Bit::A)),
            // ADC A,(HL)
            0x8e => Some(Instruction8Bit::AdcFromMem()),
            // SUB A,r
            0x90 => Some(Instruction8Bit::Sub(Register8Bit::B)),
            0x91 => Some(Instruction8Bit::Sub(Register8Bit::C)),
            0x92 => Some(Instruction8Bit::Sub(Register8Bit::D)),
            0x93 => Some(Instruction8Bit::Sub(Register8Bit::E)),
            0x94 => Some(Instruction8Bit::Sub(Register8Bit::H)),
            0x95 => Some(Instruction8Bit::Sub(Register8Bit::L)),
            0x97 => Some(Instruction8Bit::Sub(Register8Bit::A)),
            // SUB A,d8
            0xd6 => Some(Instruction8Bit::SubImm()),
            // SUB A,(HL)
            0x96 => Some(Instruction8Bit::SubFromMem()),
            // SBC A,r
            0x98 => Some(Instruction8Bit::Sbc(Register8Bit::B)),
            0x99 => Some(Instruction8Bit::Sbc(Register8Bit::C)),
            0x9a => Some(Instruction8Bit::Sbc(Register8Bit::D)),
            0x9b => Some(Instruction8Bit::Sbc(Register8Bit::E)),
            0x9c => Some(Instruction8Bit::Sbc(Register8Bit::H)),
            0x9d => Some(Instruction8Bit::Sbc(Register8Bit::L)),
            0x9f => Some(Instruction8Bit::Sbc(Register8Bit::A)),
            // SBC A,(HL)
            0x9e => Some(Instruction8Bit::SbcFromMem()),
            // LD B, X
            0x40 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::B)),
            0x41 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::C)),
            0x42 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::D)),
            0x43 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::E)),
            0x44 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::H)),
            0x45 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::L)),
            0x47 => Some(Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::A)),
            // LD C, X
            0x48 => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::B)),
            0x49 => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::C)),
            0x4a => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::D)),
            0x4b => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::E)),
            0x4c => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::H)),
            0x4d => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::L)),
            0x4f => Some(Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::A)),
            // LD D, X
            0x50 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::B)),
            0x51 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::C)),
            0x52 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::D)),
            0x53 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::E)),
            0x54 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::H)),
            0x55 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::L)),
            0x57 => Some(Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::A)),
            // LD E, X
            0x58 => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::B)),
            0x59 => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::C)),
            0x5a => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::D)),
            0x5b => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::E)),
            0x5c => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::H)),
            0x5d => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::L)),
            0x5f => Some(Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::A)),
            // LD H, X
            0x60 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::B)),
            0x61 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::C)),
            0x62 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::D)),
            0x63 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::E)),
            0x64 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::H)),
            0x65 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::L)),
            0x67 => Some(Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::A)),
            // LD L, X
            0x68 => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::B)),
            0x69 => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::C)), // nice
            0x6a => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::D)),
            0x6b => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::E)),
            0x6c => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::H)),
            0x6d => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::L)),
            0x6f => Some(Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::A)),
            // LD A, X
            0x78 => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::B)),
            0x79 => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::C)),
            0x7a => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::D)),
            0x7b => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::E)),
            0x7c => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::H)),
            0x7d => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::L)),
            0x7f => Some(Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::A)),
            // LD X, nn
            0x06 => Some(Instruction8Bit::LdImm(Register8Bit::B)),
            0x16 => Some(Instruction8Bit::LdImm(Register8Bit::D)),
            0x26 => Some(Instruction8Bit::LdImm(Register8Bit::H)),
            0x0e => Some(Instruction8Bit::LdImm(Register8Bit::C)),
            0x1e => Some(Instruction8Bit::LdImm(Register8Bit::E)),
            0x2e => Some(Instruction8Bit::LdImm(Register8Bit::L)),
            0x3e => Some(Instruction8Bit::LdImm(Register8Bit::A)),
            // LD (HL), X
            0x70 => Some(Instruction8Bit::LdToMem(Register8Bit::B, Register16Bit::HL)),
            0x71 => Some(Instruction8Bit::LdToMem(Register8Bit::C, Register16Bit::HL)),
            0x72 => Some(Instruction8Bit::LdToMem(Register8Bit::D, Register16Bit::HL)),
            0x73 => Some(Instruction8Bit::LdToMem(Register8Bit::E, Register16Bit::HL)),
            0x74 => Some(Instruction8Bit::LdToMem(Register8Bit::H, Register16Bit::HL)),
            0x75 => Some(Instruction8Bit::LdToMem(Register8Bit::L, Register16Bit::HL)),
            0x77 => Some(Instruction8Bit::LdToMem(Register8Bit::A, Register16Bit::HL)),
            // LD X, (HL)
            0x46 => Some(Instruction8Bit::LdFromMem(Register8Bit::B, Register16Bit::HL)),
            0x4e => Some(Instruction8Bit::LdFromMem(Register8Bit::C, Register16Bit::HL)),
            0x56 => Some(Instruction8Bit::LdFromMem(Register8Bit::D, Register16Bit::HL)),
            0x5e => Some(Instruction8Bit::LdFromMem(Register8Bit::E, Register16Bit::HL)),
            0x66 => Some(Instruction8Bit::LdFromMem(Register8Bit::H, Register16Bit::HL)),
            0x6e => Some(Instruction8Bit::LdFromMem(Register8Bit::L, Register16Bit::HL)),
            0x7e => Some(Instruction8Bit::LdFromMem(Register8Bit::A, Register16Bit::HL)),
            // LD A, (XX)
            0x02 => Some(Instruction8Bit::LdToMem(Register8Bit::A, Register16Bit::BC)),
            0x12 => Some(Instruction8Bit::LdToMem(Register8Bit::A, Register16Bit::DE)),
            0x0a => Some(Instruction8Bit::LdFromMem(Register8Bit::A, Register16Bit::BC)),
            0x1a => Some(Instruction8Bit::LdFromMem(Register8Bit::A, Register16Bit::DE)),
            // LD A, HL(x)crement
            0x22 => Some(Instruction8Bit::LdToMemInc()),
            0x32 => Some(Instruction8Bit::LdToMemDec()),
            0x2a => Some(Instruction8Bit::LdFromMemInc()),
            0x3a => Some(Instruction8Bit::LdFromMemDec()),
            // LD XX, nn
            0x01 => Some(Instruction8Bit::LdRegister16Imm(Register16Bit::BC)),
            0x11 => Some(Instruction8Bit::LdRegister16Imm(Register16Bit::DE)),
            0x21 => Some(Instruction8Bit::LdRegister16Imm(Register16Bit::HL)),
            0x31 => Some(Instruction8Bit::LdRegister16Imm(Register16Bit::SP)),
            // JP
            0xc2 => Some(Instruction8Bit::JumpAbs(BranchCondition::NZ)),
            0xd2 => Some(Instruction8Bit::JumpAbs(BranchCondition::NC)),
            0xc3 => Some(Instruction8Bit::JumpAbs(BranchCondition::NONE)),
            0xca => Some(Instruction8Bit::JumpAbs(BranchCondition::Z)),
            0xda => Some(Instruction8Bit::JumpAbs(BranchCondition::C)),
            // JP (HL)
            0xe9 => Some(Instruction8Bit::JumpAbsFromReg()),
            // JR
            0x20 => Some(Instruction8Bit::JumpRel(BranchCondition::NZ)),
            0x30 => Some(Instruction8Bit::JumpRel(BranchCondition::NC)),
            0x18 => Some(Instruction8Bit::JumpRel(BranchCondition::NONE)),
            0x28 => Some(Instruction8Bit::JumpRel(BranchCondition::Z)),
            0x38 => Some(Instruction8Bit::JumpRel(BranchCondition::C)),
            // PUSH (XX)
            0xc5 => Some(Instruction8Bit::Push(Register16Bit::BC)),
            0xd5 => Some(Instruction8Bit::Push(Register16Bit::DE)),
            0xe5 => Some(Instruction8Bit::Push(Register16Bit::HL)),
            0xf5 => Some(Instruction8Bit::Push(Register16Bit::AF)),
            // POP (XX)
            0xc1 => Some(Instruction8Bit::Pop(Register16Bit::BC)),
            0xd1 => Some(Instruction8Bit::Pop(Register16Bit::DE)),
            0xe1 => Some(Instruction8Bit::Pop(Register16Bit::HL)),
            0xf1 => Some(Instruction8Bit::Pop(Register16Bit::AF)),
            // RET
            0xc0 => Some(Instruction8Bit::Ret(BranchCondition::NZ)),
            0xd0 => Some(Instruction8Bit::Ret(BranchCondition::NC)),
            0xc8 => Some(Instruction8Bit::Ret(BranchCondition::Z)),
            0xd8 => Some(Instruction8Bit::Ret(BranchCondition::C)),
            0xc9 => Some(Instruction8Bit::Ret(BranchCondition::NONE)),
            // RST
            0xc7 => Some(Instruction8Bit::Rst(RstVec::ZERO)),
            0xcf => Some(Instruction8Bit::Rst(RstVec::ONE)),
            0xd7 => Some(Instruction8Bit::Rst(RstVec::TWO)),
            0xdf => Some(Instruction8Bit::Rst(RstVec::THREE)),
            0xe7 => Some(Instruction8Bit::Rst(RstVec::FOUR)),
            0xef => Some(Instruction8Bit::Rst(RstVec::FIVE)),
            0xf7 => Some(Instruction8Bit::Rst(RstVec::SIX)),
            0xff => Some(Instruction8Bit::Rst(RstVec::SEVEN)),
            // CALL
            0xc4 => Some(Instruction8Bit::Call(BranchCondition::NZ)),
            0xd4 => Some(Instruction8Bit::Call(BranchCondition::NC)),
            0xcc => Some(Instruction8Bit::Call(BranchCondition::Z)),
            0xdc => Some(Instruction8Bit::Call(BranchCondition::C)),
            0xcd => Some(Instruction8Bit::Call(BranchCondition::NONE)),
            // Special dummy instruction for 16 bit instruction calls
            0xcb => Some(Instruction8Bit::Call16BitInstruction()),
            _ => None
        }
    }

    pub fn as_byte(self) -> u8 {
        match self {
            // NOOP
            Instruction8Bit::Noop() => 0x00,
            // INC r
            Instruction8Bit::Inc(Register8Bit::B) => 0x04,
            Instruction8Bit::Inc(Register8Bit::C) => 0x0c,
            Instruction8Bit::Inc(Register8Bit::D) => 0x14,
            Instruction8Bit::Inc(Register8Bit::E) => 0x1c,
            Instruction8Bit::Inc(Register8Bit::H) => 0x24,
            Instruction8Bit::Inc(Register8Bit::L) => 0x2c,
            Instruction8Bit::Inc(Register8Bit::A) => 0x3c,
            // DEC r
            Instruction8Bit::Dec(Register8Bit::B) => 0x05,
            Instruction8Bit::Dec(Register8Bit::C) => 0x0d,
            Instruction8Bit::Dec(Register8Bit::D) => 0x15,
            Instruction8Bit::Dec(Register8Bit::E) => 0x1d,
            Instruction8Bit::Dec(Register8Bit::H) => 0x25,
            Instruction8Bit::Dec(Register8Bit::L) => 0x2d,
            Instruction8Bit::Dec(Register8Bit::A) => 0x3d,
            // AND r
            Instruction8Bit::And(Register8Bit::B) => 0xa0,
            Instruction8Bit::And(Register8Bit::C) => 0xa1,
            Instruction8Bit::And(Register8Bit::D) => 0xa2,
            Instruction8Bit::And(Register8Bit::E) => 0xa3,
            Instruction8Bit::And(Register8Bit::H) => 0xa4,
            Instruction8Bit::And(Register8Bit::L) => 0xa5,
            Instruction8Bit::And(Register8Bit::A) => 0xa7,
            // AND (HL)
            Instruction8Bit::AndFromMem() => 0xa6,
            // AND n
            Instruction8Bit::AndImm() => 0xe6,
            // OR r
            Instruction8Bit::Or(Register8Bit::B) => 0xb0,
            Instruction8Bit::Or(Register8Bit::C) => 0xb1,
            Instruction8Bit::Or(Register8Bit::D) => 0xb2,
            Instruction8Bit::Or(Register8Bit::E) => 0xb3,
            Instruction8Bit::Or(Register8Bit::H) => 0xb4,
            Instruction8Bit::Or(Register8Bit::L) => 0xb5,
            Instruction8Bit::Or(Register8Bit::A) => 0xb7,
            // OR (HL)
            Instruction8Bit::OrFromMem() => 0xb6,
            // OR n
            Instruction8Bit::OrImm() => 0xf6,
            // XOR r
            Instruction8Bit::Xor(Register8Bit::B) => 0xa8,
            Instruction8Bit::Xor(Register8Bit::C) => 0xa9,
            Instruction8Bit::Xor(Register8Bit::D) => 0xaa,
            Instruction8Bit::Xor(Register8Bit::E) => 0xab,
            Instruction8Bit::Xor(Register8Bit::H) => 0xac,
            Instruction8Bit::Xor(Register8Bit::L) => 0xad,
            Instruction8Bit::Xor(Register8Bit::A) => 0xaf,
            // XOR (HL)
            Instruction8Bit::XorFromMem() => 0xae,
            // XOR n
            Instruction8Bit::XorImm() => 0xee,
            // CP r
            Instruction8Bit::Cp(Register8Bit::B) => 0xb8,
            Instruction8Bit::Cp(Register8Bit::C) => 0xb9,
            Instruction8Bit::Cp(Register8Bit::D) => 0xba,
            Instruction8Bit::Cp(Register8Bit::E) => 0xbb,
            Instruction8Bit::Cp(Register8Bit::H) => 0xbc,
            Instruction8Bit::Cp(Register8Bit::L) => 0xbd,
            Instruction8Bit::Cp(Register8Bit::A) => 0xbf,
            // CP (HL)
            Instruction8Bit::CpFromMem() => 0xbe,
            // CP n
            Instruction8Bit::CpImm() => 0xfe,
            // ADD A,r
            Instruction8Bit::Add(Register8Bit::B) => 0x80,
            Instruction8Bit::Add(Register8Bit::C) => 0x81,
            Instruction8Bit::Add(Register8Bit::D) => 0x82,
            Instruction8Bit::Add(Register8Bit::E) => 0x83,
            Instruction8Bit::Add(Register8Bit::H) => 0x84,
            Instruction8Bit::Add(Register8Bit::L) => 0x85,
            Instruction8Bit::Add(Register8Bit::A) => 0x87,
            // ADD A,d8
            Instruction8Bit::AddImm() => 0xc6,
            // ADD A,(HL)
            Instruction8Bit::AddFromMem() => 0x86,
            // ADC A,R
            Instruction8Bit::Adc(Register8Bit::B) => 0x88,
            Instruction8Bit::Adc(Register8Bit::C) => 0x89,
            Instruction8Bit::Adc(Register8Bit::D) => 0x8a,
            Instruction8Bit::Adc(Register8Bit::E) => 0x8b,
            Instruction8Bit::Adc(Register8Bit::H) => 0x8c,
            Instruction8Bit::Adc(Register8Bit::L) => 0x8d,
            Instruction8Bit::Adc(Register8Bit::A) => 0x8f,
            // ADC A,(HL)
            Instruction8Bit::AdcFromMem() => 0x8e,
            // SUB A,r
            Instruction8Bit::Sub(Register8Bit::B) => 0x90,
            Instruction8Bit::Sub(Register8Bit::C) => 0x91,
            Instruction8Bit::Sub(Register8Bit::D) => 0x92,
            Instruction8Bit::Sub(Register8Bit::E) => 0x93,
            Instruction8Bit::Sub(Register8Bit::H) => 0x94,
            Instruction8Bit::Sub(Register8Bit::L) => 0x95,
            Instruction8Bit::Sub(Register8Bit::A) => 0x97,
            // SUB A,d8
            Instruction8Bit::SubImm() => 0xd6,
            // SUB A,(HL)
            Instruction8Bit::SubFromMem() => 0x96,
            // SBC A,r
            Instruction8Bit::Sbc(Register8Bit::B) => 0x98,
            Instruction8Bit::Sbc(Register8Bit::C) => 0x99,
            Instruction8Bit::Sbc(Register8Bit::D) => 0x9a,
            Instruction8Bit::Sbc(Register8Bit::E) => 0x9b,
            Instruction8Bit::Sbc(Register8Bit::H) => 0x9c,
            Instruction8Bit::Sbc(Register8Bit::L) => 0x9d,
            Instruction8Bit::Sbc(Register8Bit::A) => 0x9f,
            // SBC A,(HL)
            Instruction8Bit::SbcFromMem() => 0x9e,
            // LD B, Y
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::B) => 0x40,
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::C) => 0x41,
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::D) => 0x42,
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::E) => 0x43,
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::H) => 0x44,
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::L) => 0x45,
            Instruction8Bit::LdRegister(Register8Bit::B, Register8Bit::A) => 0x47,
            // LD C, X
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::B) => 0x48,
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::C) => 0x49,
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::D) => 0x4a,
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::E) => 0x4b,
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::H) => 0x4c,
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::L) => 0x4d,
            Instruction8Bit::LdRegister(Register8Bit::C, Register8Bit::A) => 0x4f,
            // LD D, X
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::B) => 0x50,
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::C) => 0x51,
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::D) => 0x52,
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::E) => 0x53,
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::H) => 0x54,
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::L) => 0x55,
            Instruction8Bit::LdRegister(Register8Bit::D, Register8Bit::A) => 0x57,
            // LD E, X
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::B) => 0x58,
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::C) => 0x59,
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::D) => 0x5a,
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::E) => 0x5b,
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::H) => 0x5c,
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::L) => 0x5d,
            Instruction8Bit::LdRegister(Register8Bit::E, Register8Bit::A) => 0x5f,
            // LD H, X
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::B) => 0x60,
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::C) => 0x61,
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::D) => 0x62,
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::E) => 0x63,
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::H) => 0x64,
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::L) => 0x65,
            Instruction8Bit::LdRegister(Register8Bit::H, Register8Bit::A) => 0x67,
            // LD L, X
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::B) => 0x68,
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::C) => 0x69, // nice 
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::D) => 0x6a, 
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::E) => 0x6b, 
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::H) => 0x6c,
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::L) => 0x6d,
            Instruction8Bit::LdRegister(Register8Bit::L, Register8Bit::A) => 0x6f,
            // LD A, X
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::B) => 0x78,
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::C) => 0x79,
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::D) => 0x7a,
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::E) => 0x7b,
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::H) => 0x7c,
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::L) => 0x7d,
            Instruction8Bit::LdRegister(Register8Bit::A, Register8Bit::A) => 0x7f,
            // LD X, nn
            Instruction8Bit::LdImm(Register8Bit::B) => 0x06,
            Instruction8Bit::LdImm(Register8Bit::D) => 0x16,
            Instruction8Bit::LdImm(Register8Bit::H) => 0x26,
            Instruction8Bit::LdImm(Register8Bit::C) => 0x0e,
            Instruction8Bit::LdImm(Register8Bit::E) => 0x1e,
            Instruction8Bit::LdImm(Register8Bit::L) => 0x2e,
            Instruction8Bit::LdImm(Register8Bit::A) => 0x3e,
            // LD (HL), X
            Instruction8Bit::LdToMem(Register8Bit::B, Register16Bit::HL) => 0x70,
            Instruction8Bit::LdToMem(Register8Bit::C, Register16Bit::HL) => 0x71,
            Instruction8Bit::LdToMem(Register8Bit::D, Register16Bit::HL) => 0x72,
            Instruction8Bit::LdToMem(Register8Bit::E, Register16Bit::HL) => 0x73,
            Instruction8Bit::LdToMem(Register8Bit::H, Register16Bit::HL) => 0x74,
            Instruction8Bit::LdToMem(Register8Bit::L, Register16Bit::HL) => 0x75,
            Instruction8Bit::LdToMem(Register8Bit::A, Register16Bit::HL) => 0x77,
            // LD X, (HL)
            Instruction8Bit::LdFromMem(Register8Bit::B, Register16Bit::HL) => 0x46,
            Instruction8Bit::LdFromMem(Register8Bit::C, Register16Bit::HL) => 0x4e,
            Instruction8Bit::LdFromMem(Register8Bit::D, Register16Bit::HL) => 0x56,
            Instruction8Bit::LdFromMem(Register8Bit::E, Register16Bit::HL) => 0x5e,
            Instruction8Bit::LdFromMem(Register8Bit::H, Register16Bit::HL) => 0x66,
            Instruction8Bit::LdFromMem(Register8Bit::L, Register16Bit::HL) => 0x6e,
            Instruction8Bit::LdFromMem(Register8Bit::A, Register16Bit::HL) => 0x7e,
            // LD A, (XX)
            Instruction8Bit::LdToMem(Register8Bit::A, Register16Bit::BC) => 0x02,
            Instruction8Bit::LdToMem(Register8Bit::A, Register16Bit::DE) => 0x12,
            Instruction8Bit::LdFromMem(Register8Bit::A, Register16Bit::BC) => 0x0a,
            Instruction8Bit::LdFromMem(Register8Bit::A, Register16Bit::DE) => 0x1a,
            // LD A, HL(x)crement
            Instruction8Bit::LdToMemInc() => 0x22,
            Instruction8Bit::LdToMemDec() => 0x32,
            Instruction8Bit::LdFromMemInc() => 0x2a,
            Instruction8Bit::LdFromMemDec() => 0x3a,
            // LD XX, nn
            Instruction8Bit::LdRegister16Imm(Register16Bit::BC) => 0x01,
            Instruction8Bit::LdRegister16Imm(Register16Bit::DE) => 0x11,
            Instruction8Bit::LdRegister16Imm(Register16Bit::HL) => 0x21,
            Instruction8Bit::LdRegister16Imm(Register16Bit::SP) => 0x31,
            // JP
            Instruction8Bit::JumpAbs(BranchCondition::NZ) => 0xc2,
            Instruction8Bit::JumpAbs(BranchCondition::NC) => 0xd2,
            Instruction8Bit::JumpAbs(BranchCondition::NONE) => 0xc3,
            Instruction8Bit::JumpAbs(BranchCondition::Z) => 0xca,
            Instruction8Bit::JumpAbs(BranchCondition::C) => 0xda,
            // JP (HL)
            Instruction8Bit::JumpAbsFromReg() => 0xe9,
            // JR
            Instruction8Bit::JumpRel(BranchCondition::NZ) => 0x20,
            Instruction8Bit::JumpRel(BranchCondition::NC) => 0x30,
            Instruction8Bit::JumpRel(BranchCondition::NONE) =>0x18,
            Instruction8Bit::JumpRel(BranchCondition::Z) => 0x28,
            Instruction8Bit::JumpRel(BranchCondition::C) => 0x38,
            // PUSH (XX)
            Instruction8Bit::Push(Register16Bit::BC) => 0xc5,
            Instruction8Bit::Push(Register16Bit::DE) => 0xd5,
            Instruction8Bit::Push(Register16Bit::HL) => 0xe5,
            Instruction8Bit::Push(Register16Bit::AF) => 0xf5,
            // POP (XX)
            Instruction8Bit::Pop(Register16Bit::BC) => 0xc1,
            Instruction8Bit::Pop(Register16Bit::DE) => 0xd1,
            Instruction8Bit::Pop(Register16Bit::HL) => 0xe1,
            Instruction8Bit::Pop(Register16Bit::AF) => 0xf1,
            // RET
            Instruction8Bit::Ret(BranchCondition::NZ) => 0xc0,
            Instruction8Bit::Ret(BranchCondition::NC) => 0xd0,
            Instruction8Bit::Ret(BranchCondition::Z) => 0xc8,
            Instruction8Bit::Ret(BranchCondition::C) => 0xd8,
            Instruction8Bit::Ret(BranchCondition::NONE) => 0xc9,
            // RST
            Instruction8Bit::Rst(RstVec::ZERO) => 0xc7,
            Instruction8Bit::Rst(RstVec::ONE) => 0xcf,
            Instruction8Bit::Rst(RstVec::TWO) => 0xd7,
            Instruction8Bit::Rst(RstVec::THREE) => 0xdf,
            Instruction8Bit::Rst(RstVec::FOUR) => 0xe7,
            Instruction8Bit::Rst(RstVec::FIVE) => 0xef,
            Instruction8Bit::Rst(RstVec::SIX) => 0xf7,
            Instruction8Bit::Rst(RstVec::SEVEN) => 0xff,
            // CALL
            Instruction8Bit::Call(BranchCondition::NZ) => 0xc4,
            Instruction8Bit::Call(BranchCondition::NC) => 0xd4,
            Instruction8Bit::Call(BranchCondition::Z) => 0xcc,
            Instruction8Bit::Call(BranchCondition::C) => 0xdc,
            Instruction8Bit::Call(BranchCondition::NONE) => 0xcd,
            // Special dummy instruction for 16 bit instruction calls
            Instruction8Bit::Call16BitInstruction() => 0xcb,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction16Bit {
    Res(Register8Bit, u32),
}

impl Instruction16Bit {
    pub fn from_byte(byte: u8) -> Option<Instruction16Bit> {
        match byte {
            // 0 bit
            0x80 => Some(Instruction16Bit::Res(Register8Bit::B, 0)),
            0x81 => Some(Instruction16Bit::Res(Register8Bit::C, 0)),
            0x82 => Some(Instruction16Bit::Res(Register8Bit::D, 0)),
            0x83 => Some(Instruction16Bit::Res(Register8Bit::E, 0)),
            0x84 => Some(Instruction16Bit::Res(Register8Bit::H, 0)),
            0x85 => Some(Instruction16Bit::Res(Register8Bit::L, 0)),
            0x87 => Some(Instruction16Bit::Res(Register8Bit::A, 0)),
            // 1 bit
            0x88 => Some(Instruction16Bit::Res(Register8Bit::B, 1)),
            0x89 => Some(Instruction16Bit::Res(Register8Bit::C, 1)),
            0x8a => Some(Instruction16Bit::Res(Register8Bit::D, 1)),
            0x8b => Some(Instruction16Bit::Res(Register8Bit::E, 1)),
            0x8c => Some(Instruction16Bit::Res(Register8Bit::H, 1)),
            0x8d => Some(Instruction16Bit::Res(Register8Bit::L, 1)),
            0x8f => Some(Instruction16Bit::Res(Register8Bit::A, 1)),
            // 2 bit
            0x90 => Some(Instruction16Bit::Res(Register8Bit::B, 2)),
            0x91 => Some(Instruction16Bit::Res(Register8Bit::C, 2)),
            0x92 => Some(Instruction16Bit::Res(Register8Bit::D, 2)),
            0x93 => Some(Instruction16Bit::Res(Register8Bit::E, 2)),
            0x94 => Some(Instruction16Bit::Res(Register8Bit::H, 2)),
            0x95 => Some(Instruction16Bit::Res(Register8Bit::L, 2)),
            0x97 => Some(Instruction16Bit::Res(Register8Bit::A, 2)),
            // 3 bit
            0x98 => Some(Instruction16Bit::Res(Register8Bit::B, 3)),
            0x99 => Some(Instruction16Bit::Res(Register8Bit::C, 3)),
            0x9a => Some(Instruction16Bit::Res(Register8Bit::D, 3)),
            0x9b => Some(Instruction16Bit::Res(Register8Bit::E, 3)),
            0x9c => Some(Instruction16Bit::Res(Register8Bit::H, 3)),
            0x9d => Some(Instruction16Bit::Res(Register8Bit::L, 3)),
            0x9f => Some(Instruction16Bit::Res(Register8Bit::A, 3)),
            // 4 bit
            0xa0 => Some(Instruction16Bit::Res(Register8Bit::B, 4)),
            0xa1 => Some(Instruction16Bit::Res(Register8Bit::C, 4)),
            0xa2 => Some(Instruction16Bit::Res(Register8Bit::D, 4)),
            0xa3 => Some(Instruction16Bit::Res(Register8Bit::E, 4)),
            0xa4 => Some(Instruction16Bit::Res(Register8Bit::H, 4)),
            0xa5 => Some(Instruction16Bit::Res(Register8Bit::L, 4)),
            0xa7 => Some(Instruction16Bit::Res(Register8Bit::A, 4)),
            // 5 bit
            0xa8 => Some(Instruction16Bit::Res(Register8Bit::B, 5)),
            0xa9 => Some(Instruction16Bit::Res(Register8Bit::C, 5)),
            0xaa => Some(Instruction16Bit::Res(Register8Bit::D, 5)),
            0xab => Some(Instruction16Bit::Res(Register8Bit::E, 5)),
            0xac => Some(Instruction16Bit::Res(Register8Bit::H, 5)),
            0xad => Some(Instruction16Bit::Res(Register8Bit::L, 5)),
            0xaf => Some(Instruction16Bit::Res(Register8Bit::A, 5)),
            // 6 bit
            0xb0 => Some(Instruction16Bit::Res(Register8Bit::B, 6)),
            0xb1 => Some(Instruction16Bit::Res(Register8Bit::C, 6)),
            0xb2 => Some(Instruction16Bit::Res(Register8Bit::D, 6)),
            0xb3 => Some(Instruction16Bit::Res(Register8Bit::E, 6)),
            0xb4 => Some(Instruction16Bit::Res(Register8Bit::H, 6)),
            0xb5 => Some(Instruction16Bit::Res(Register8Bit::L, 6)),
            0xb7 => Some(Instruction16Bit::Res(Register8Bit::A, 6)),
            // 7 bit
            0xb8 => Some(Instruction16Bit::Res(Register8Bit::B, 7)),
            0xb9 => Some(Instruction16Bit::Res(Register8Bit::C, 7)),
            0xba => Some(Instruction16Bit::Res(Register8Bit::D, 7)),
            0xbb => Some(Instruction16Bit::Res(Register8Bit::E, 7)),
            0xbc => Some(Instruction16Bit::Res(Register8Bit::H, 7)),
            0xbd => Some(Instruction16Bit::Res(Register8Bit::L, 7)),
            0xbf => Some(Instruction16Bit::Res(Register8Bit::A, 7)),
            _ => panic!("Invalid instruction"),
        }
    }
    pub fn as_byte(self) -> u8 {
        match self {
            // 0 bit
            Instruction16Bit::Res(Register8Bit::B, 0) => 0x80,
            Instruction16Bit::Res(Register8Bit::C, 0) => 0x81,
            Instruction16Bit::Res(Register8Bit::D, 0) => 0x82,
            Instruction16Bit::Res(Register8Bit::E, 0) => 0x83,
            Instruction16Bit::Res(Register8Bit::H, 0) => 0x84,
            Instruction16Bit::Res(Register8Bit::L, 0) => 0x85,
            Instruction16Bit::Res(Register8Bit::A, 0) => 0x87,
            // 1 bit
            Instruction16Bit::Res(Register8Bit::B, 1) => 0x88,
            Instruction16Bit::Res(Register8Bit::C, 1) => 0x89,
            Instruction16Bit::Res(Register8Bit::D, 1) => 0x8a,
            Instruction16Bit::Res(Register8Bit::E, 1) => 0x8b,
            Instruction16Bit::Res(Register8Bit::H, 1) => 0x8c,
            Instruction16Bit::Res(Register8Bit::L, 1) => 0x8d,
            Instruction16Bit::Res(Register8Bit::A, 1) => 0x8f,
            // 2 bit
            Instruction16Bit::Res(Register8Bit::B, 2) => 0x90,
            Instruction16Bit::Res(Register8Bit::C, 2) => 0x91,
            Instruction16Bit::Res(Register8Bit::D, 2) => 0x92,
            Instruction16Bit::Res(Register8Bit::E, 2) => 0x93,
            Instruction16Bit::Res(Register8Bit::H, 2) => 0x94,
            Instruction16Bit::Res(Register8Bit::L, 2) => 0x95,
            Instruction16Bit::Res(Register8Bit::A, 2) => 0x97,
            // 3 bit
            Instruction16Bit::Res(Register8Bit::B, 3) => 0x98,
            Instruction16Bit::Res(Register8Bit::C, 3) => 0x99,
            Instruction16Bit::Res(Register8Bit::D, 3) => 0x9a,
            Instruction16Bit::Res(Register8Bit::E, 3) => 0x9b,
            Instruction16Bit::Res(Register8Bit::H, 3) => 0x9c,
            Instruction16Bit::Res(Register8Bit::L, 3) => 0x9d,
            Instruction16Bit::Res(Register8Bit::A, 3) => 0x9f,
            // 4 bit
            Instruction16Bit::Res(Register8Bit::B, 4) => 0xa0,
            Instruction16Bit::Res(Register8Bit::C, 4) => 0xa1,
            Instruction16Bit::Res(Register8Bit::D, 4) => 0xa2,
            Instruction16Bit::Res(Register8Bit::E, 4) => 0xa3,
            Instruction16Bit::Res(Register8Bit::H, 4) => 0xa4,
            Instruction16Bit::Res(Register8Bit::L, 4) => 0xa5,
            Instruction16Bit::Res(Register8Bit::A, 4) => 0xa7,
            // 5 bit
            Instruction16Bit::Res(Register8Bit::B, 5) => 0xa8,
            Instruction16Bit::Res(Register8Bit::C, 5) => 0xa9,
            Instruction16Bit::Res(Register8Bit::D, 5) => 0xaa,
            Instruction16Bit::Res(Register8Bit::E, 5) => 0xab,
            Instruction16Bit::Res(Register8Bit::H, 5) => 0xac,
            Instruction16Bit::Res(Register8Bit::L, 5) => 0xad,
            Instruction16Bit::Res(Register8Bit::A, 5) => 0xaf,
            // 6 bit
            Instruction16Bit::Res(Register8Bit::B, 6) => 0xb0,
            Instruction16Bit::Res(Register8Bit::C, 6) => 0xb1,
            Instruction16Bit::Res(Register8Bit::D, 6) => 0xb2,
            Instruction16Bit::Res(Register8Bit::E, 6) => 0xb3,
            Instruction16Bit::Res(Register8Bit::H, 6) => 0xb4,
            Instruction16Bit::Res(Register8Bit::L, 6) => 0xb5,
            Instruction16Bit::Res(Register8Bit::A, 6) => 0xb7,
            // 7 bit
            Instruction16Bit::Res(Register8Bit::B, 7) => 0xb8,
            Instruction16Bit::Res(Register8Bit::C, 7) => 0xb9,
            Instruction16Bit::Res(Register8Bit::D, 7) => 0xba,
            Instruction16Bit::Res(Register8Bit::E, 7) => 0xbb,
            Instruction16Bit::Res(Register8Bit::H, 7) => 0xbc,
            Instruction16Bit::Res(Register8Bit::L, 7) => 0xbd,
            Instruction16Bit::Res(Register8Bit::A, 7) => 0xbf,

            _ => panic!("Invalid instruction"),
        }
    }
}
