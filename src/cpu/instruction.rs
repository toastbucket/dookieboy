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
    Rra(),
    Rrca(),
    Rla(),
    Rlca(),
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
    ToggleCarryFlag(),
    SetCarryFlag(),
    Invert(),
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
            // DEC r
            0x05 => Some(Instruction::Dec(Register8Bit::B)),
            0x0d => Some(Instruction::Dec(Register8Bit::C)),
            0x15 => Some(Instruction::Dec(Register8Bit::D)),
            0x1d => Some(Instruction::Dec(Register8Bit::E)),
            0x25 => Some(Instruction::Dec(Register8Bit::H)),
            0x2d => Some(Instruction::Dec(Register8Bit::L)),
            0x3d => Some(Instruction::Dec(Register8Bit::A)),
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
            // DEC r
            Instruction::Dec(Register8Bit::B) => 0x05,
            Instruction::Dec(Register8Bit::C) => 0x0d,
            Instruction::Dec(Register8Bit::D) => 0x15,
            Instruction::Dec(Register8Bit::E) => 0x1d,
            Instruction::Dec(Register8Bit::H) => 0x25,
            Instruction::Dec(Register8Bit::L) => 0x2d,
            Instruction::Dec(Register8Bit::A) => 0x3d,
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
            // Special dummy instruction for 16 bit instruction calls
            Instruction::CbInstruction() => 0xcb,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CbInstruction {
    Res(Register8Bit, usize),
    Set(Register8Bit, usize),
}

impl CbInstruction {
    pub fn from_byte(byte: u8) -> Option<CbInstruction> {
        match byte {
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
            _ => panic!("Invalid instruction"),
        }
    }
    pub fn as_byte(self) -> u8 {
        match self {
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
            _ => panic!("Invalid instruction"),
        }
    }
}
