// src/cpu.rs

use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;
use crate::mmu::Mmu;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Inc(ArithmeticOperand),
    Dec(ArithmeticOperand),
    And(ArithmeticOperand),
    AndHL(),
    AndImm(),
    Add(ArithmeticOperand),
    AddImm(),
    AddHL(),
    Adc(ArithmeticOperand),
    // TODO: AdcHL
    Sub(ArithmeticOperand),
    Sbc(ArithmeticOperand),
    SubImm(),
    SubHL(),
    // TODO: SbcHL
}

#[derive(Debug, Copy, Clone)]
enum ArithmeticOperand {
    A = 0, B, C, D, E, H, L
}
const ARITHMETIC_REGOP_LEN: usize = 7;

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            // INC r
            0x04 => Some(Instruction::Inc(ArithmeticOperand::B)),
            0x0c => Some(Instruction::Inc(ArithmeticOperand::C)),
            0x14 => Some(Instruction::Inc(ArithmeticOperand::D)),
            0x1c => Some(Instruction::Inc(ArithmeticOperand::E)),
            0x24 => Some(Instruction::Inc(ArithmeticOperand::H)),
            0x2c => Some(Instruction::Inc(ArithmeticOperand::L)),
            0x3c => Some(Instruction::Inc(ArithmeticOperand::A)),
            // DEC r
            0x05 => Some(Instruction::Dec(ArithmeticOperand::B)),
            0x0d => Some(Instruction::Dec(ArithmeticOperand::C)),
            0x15 => Some(Instruction::Dec(ArithmeticOperand::D)),
            0x1d => Some(Instruction::Dec(ArithmeticOperand::E)),
            0x25 => Some(Instruction::Dec(ArithmeticOperand::H)),
            0x2d => Some(Instruction::Dec(ArithmeticOperand::L)),
            0x3d => Some(Instruction::Dec(ArithmeticOperand::A)),
            // AND r
            0xa0 => Some(Instruction::And(ArithmeticOperand::B)),
            0xa1 => Some(Instruction::And(ArithmeticOperand::C)),
            0xa2 => Some(Instruction::And(ArithmeticOperand::D)),
            0xa3 => Some(Instruction::And(ArithmeticOperand::E)),
            0xa4 => Some(Instruction::And(ArithmeticOperand::H)),
            0xa5 => Some(Instruction::And(ArithmeticOperand::L)),
            0xa7 => Some(Instruction::And(ArithmeticOperand::A)),
            // AND (HL)
            0xa6 => Some(Instruction::AndHL()),
            // AND n
            0xe6 => Some(Instruction::AndImm()),
            // ADD A,r
            0x80 => Some(Instruction::Add(ArithmeticOperand::B)),
            0x81 => Some(Instruction::Add(ArithmeticOperand::C)),
            0x82 => Some(Instruction::Add(ArithmeticOperand::D)),
            0x83 => Some(Instruction::Add(ArithmeticOperand::E)),
            0x84 => Some(Instruction::Add(ArithmeticOperand::H)),
            0x85 => Some(Instruction::Add(ArithmeticOperand::L)),
            0x87 => Some(Instruction::Add(ArithmeticOperand::A)),
            // ADD A,d8
            0xc6 => Some(Instruction::AddImm()),
            // ADD A,(HL)
            0x86 => Some(Instruction::AddHL()),
            // ADC A,r
            0x88 => Some(Instruction::Adc(ArithmeticOperand::B)),
            0x89 => Some(Instruction::Adc(ArithmeticOperand::C)),
            0x8a => Some(Instruction::Adc(ArithmeticOperand::D)),
            0x8b => Some(Instruction::Adc(ArithmeticOperand::E)),
            0x8c => Some(Instruction::Adc(ArithmeticOperand::H)),
            0x8d => Some(Instruction::Adc(ArithmeticOperand::L)),
            0x8f => Some(Instruction::Adc(ArithmeticOperand::A)),
            // SUB A,r
            0x90 => Some(Instruction::Sub(ArithmeticOperand::B)),
            0x91 => Some(Instruction::Sub(ArithmeticOperand::C)),
            0x92 => Some(Instruction::Sub(ArithmeticOperand::D)),
            0x93 => Some(Instruction::Sub(ArithmeticOperand::E)),
            0x94 => Some(Instruction::Sub(ArithmeticOperand::H)),
            0x95 => Some(Instruction::Sub(ArithmeticOperand::L)),
            0x97 => Some(Instruction::Sub(ArithmeticOperand::A)),
            // SUB A,d8
            0xd6 => Some(Instruction::SubImm()),
            // SUB A,(HL)
            0x96 => Some(Instruction::SubHL()),
            // SBC A,r
            0x98 => Some(Instruction::Sbc(ArithmeticOperand::B)),
            0x99 => Some(Instruction::Sbc(ArithmeticOperand::C)),
            0x9a => Some(Instruction::Sbc(ArithmeticOperand::D)),
            0x9b => Some(Instruction::Sbc(ArithmeticOperand::E)),
            0x9c => Some(Instruction::Sbc(ArithmeticOperand::H)),
            0x9d => Some(Instruction::Sbc(ArithmeticOperand::L)),
            0x9f => Some(Instruction::Sbc(ArithmeticOperand::A)),
            _ => None
        }
    }

    fn as_byte(self) -> u8 {
        match self {
            // INC r
            Instruction::Inc(ArithmeticOperand::B) => 0x04,
            Instruction::Inc(ArithmeticOperand::C) => 0x0c,
            Instruction::Inc(ArithmeticOperand::D) => 0x14,
            Instruction::Inc(ArithmeticOperand::E) => 0x1c,
            Instruction::Inc(ArithmeticOperand::H) => 0x24,
            Instruction::Inc(ArithmeticOperand::L) => 0x2c,
            Instruction::Inc(ArithmeticOperand::A) => 0x3c,
            // DEC r
            Instruction::Dec(ArithmeticOperand::B) => 0x05,
            Instruction::Dec(ArithmeticOperand::C) => 0x0d,
            Instruction::Dec(ArithmeticOperand::D) => 0x15,
            Instruction::Dec(ArithmeticOperand::E) => 0x1d,
            Instruction::Dec(ArithmeticOperand::H) => 0x25,
            Instruction::Dec(ArithmeticOperand::L) => 0x2d,
            Instruction::Dec(ArithmeticOperand::A) => 0x3d,
            // AND r
            Instruction::And(ArithmeticOperand::B) => 0xa0,
            Instruction::And(ArithmeticOperand::C) => 0xa1,
            Instruction::And(ArithmeticOperand::D) => 0xa2,
            Instruction::And(ArithmeticOperand::E) => 0xa3,
            Instruction::And(ArithmeticOperand::H) => 0xa4,
            Instruction::And(ArithmeticOperand::L) => 0xa5,
            Instruction::And(ArithmeticOperand::A) => 0xa7,
            // AND (HL)
            Instruction::AndHL() => 0xa6,
            // AND n
            Instruction::AndImm() => 0xe6,
            // ADD A,r
            Instruction::Add(ArithmeticOperand::B) => 0x80,
            Instruction::Add(ArithmeticOperand::C) => 0x81,
            Instruction::Add(ArithmeticOperand::D) => 0x82,
            Instruction::Add(ArithmeticOperand::E) => 0x83,
            Instruction::Add(ArithmeticOperand::H) => 0x84,
            Instruction::Add(ArithmeticOperand::L) => 0x85,
            Instruction::Add(ArithmeticOperand::A) => 0x87,
            // ADD A,d8
            Instruction::AddImm() => 0xc6,
            // ADD A,(HL)
            Instruction::AddHL() => 0x86,
            // ADC A,R
            Instruction::Adc(ArithmeticOperand::B) => 0x88,
            Instruction::Adc(ArithmeticOperand::C) => 0x89,
            Instruction::Adc(ArithmeticOperand::D) => 0x8a,
            Instruction::Adc(ArithmeticOperand::E) => 0x8b,
            Instruction::Adc(ArithmeticOperand::H) => 0x8c,
            Instruction::Adc(ArithmeticOperand::L) => 0x8d,
            Instruction::Adc(ArithmeticOperand::A) => 0x8f,
            // SUB A,r
            Instruction::Sub(ArithmeticOperand::B) => 0x90,
            Instruction::Sub(ArithmeticOperand::C) => 0x91,
            Instruction::Sub(ArithmeticOperand::D) => 0x92,
            Instruction::Sub(ArithmeticOperand::E) => 0x93,
            Instruction::Sub(ArithmeticOperand::H) => 0x94,
            Instruction::Sub(ArithmeticOperand::L) => 0x95,
            Instruction::Sub(ArithmeticOperand::A) => 0x97,
            // SUB A,d8
            Instruction::SubImm() => 0xd6,
            // SUB A,(HL)
            Instruction::SubHL() => 0x96,
            // SBC A,r
            Instruction::Sbc(ArithmeticOperand::B) => 0x98,
            Instruction::Sbc(ArithmeticOperand::C) => 0x99,
            Instruction::Sbc(ArithmeticOperand::D) => 0x9a,
            Instruction::Sbc(ArithmeticOperand::E) => 0x9b,
            Instruction::Sbc(ArithmeticOperand::H) => 0x9c,
            Instruction::Sbc(ArithmeticOperand::L) => 0x9d,
            Instruction::Sbc(ArithmeticOperand::A) => 0x9f,
            _ => panic!("Invalid instruction"),
        }
    }

    fn size(&self) -> usize {
        match self {
            Instruction::Inc(_) => 1,
            Instruction::Dec(_) => 1,
            Instruction::And(_) => 1,
            Instruction::AndHL() => 1,
            Instruction::AndImm() => 2,
            Instruction::Add(_) => 1,
            Instruction::AddImm() => 2,
            Instruction::AddHL() => 1,
            Instruction::Adc(_) => 1,
            Instruction::Sub(_) => 1,
            Instruction::SubImm() => 2,
            Instruction::SubHL() => 1,
            Instruction::Sbc(_) => 1,
            _ => panic!("Invalid instruction"),
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Instruction::Inc(_) => 1,
            Instruction::Dec(_) => 1,
            Instruction::And(_) => 1,
            Instruction::AndHL() => 2,
            Instruction::AndImm() => 2,
            Instruction::Add(_) => 1,
            Instruction::AddImm() => 2,
            Instruction::AddHL() => 2,
            Instruction::Adc(_) => 1,
            Instruction::Sub(_) => 1,
            Instruction::SubImm() => 2,
            Instruction::SubHL() => 1,
            Instruction::Sbc(_) => 1,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[cfg(test)]
const TEST_RAM_SIZE: usize = 128;

pub struct Cpu {
    rf: [u8; ARITHMETIC_REGOP_LEN],
    sp: u16,
    pc: u16,
    z: bool,
    n: bool,
    h: bool,
    cy: bool,
    mmu: Rc<RefCell<Mmu>>,

    #[cfg(test)]
    test_ram: [u8; TEST_RAM_SIZE],
}

impl Cpu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Cpu {
        Cpu {
            rf: [0; ARITHMETIC_REGOP_LEN],
            sp: 0,
            pc: 0,
            z: false,
            n: false,
            h: false,
            cy: false,
            mmu: mmu,

            #[cfg(test)]
            test_ram: [0; TEST_RAM_SIZE],
        }
    }

    #[cfg(test)]
    fn load_test_ram(&mut self, data: &[u8]) {
        self.test_ram[..data.len()].clone_from_slice(data);
    }

    #[cfg(test)]
    fn read_byte(&self, addr: u16) -> u8 {
        if addr > TEST_RAM_SIZE as u16{
            panic!("Address {:#04x} outside of test rom size {}", addr, TEST_RAM_SIZE);
        }

        self.test_ram[addr as usize]
    }

    #[cfg(not(test))]
    fn read_byte(&self, addr: u16) -> u8 {
        self.mmu.borrow().mem_read_byte(addr)
    }

    #[cfg(test)]
    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr > TEST_RAM_SIZE as u16 {
            panic!("Address {:#04x} outside of test rom size {}", addr, TEST_RAM_SIZE);
        }

        self.test_ram[addr as usize] = val;
    }

    #[cfg(not(test))]
    fn write_byte(&mut self, addr: u16, val: u8) {
        self.mmu.borrow_mut().mem_write_byte(addr, val);
    }

    fn get_hl(&self) -> u16 {
        ((self.get_reg(ArithmeticOperand::H) as u16) << 8) | (self.get_reg(ArithmeticOperand::L) as u16)
    }

    fn get_reg(&self, regop: ArithmeticOperand) -> u8 {
        self.rf[regop as usize]
    }

    fn set_reg(&mut self, regop: ArithmeticOperand, val: u8) {
        self.rf[regop as usize] = val; 
    }

    fn set_all_regs(&mut self, val: u8) {
        self.rf.iter_mut().for_each(|x| *x = val);
    }

    fn and(&mut self, regop: ArithmeticOperand, operand: u8) {
        let result = self.get_reg(regop) & operand;
        self.z = result == 0;
        self.n = false;
        self.h = true;
        self.cy = false;
        self.set_reg(regop, result);
    }

    fn subtract(&mut self, regop: ArithmeticOperand, operand: u8, with_carry: bool) {
        let (result, did_wrap) = if with_carry {
            let (carry_result, carry_did_wrap) = self.get_reg(regop).overflowing_sub(self.cy as u8);
            let (operand_result, operand_did_wrap) = carry_result.overflowing_sub(operand);
            (operand_result, carry_did_wrap | operand_did_wrap)
        } else {
            self.get_reg(regop).overflowing_sub(operand)
        };

        self.z = result == 0;
        self.n = true;
        self.h = ((self.get_reg(regop) as i8) & 0xf)
            .wrapping_sub((operand as i8) & 0xf)
            .wrapping_sub(if with_carry { self.cy as i8 } else { 0 }) < 0;
        self.cy = did_wrap;
        self.set_reg(regop, result);
    }

    fn add(&mut self, regop: ArithmeticOperand, operand: u8, with_carry: bool) {
        let (result, did_wrap) = if with_carry {
            let (carry_result, carry_did_wrap) = self.get_reg(regop).overflowing_add(self.cy as u8);
            let (operand_result, operand_did_wrap) = carry_result.overflowing_add(operand);
            (operand_result, carry_did_wrap | operand_did_wrap)
        } else {
            self.get_reg(regop).overflowing_add(operand)
        };

        self.z = result == 0;
        self.n = false;
        self.h = (self.get_reg(regop) & 0xf)
            .wrapping_add(operand & 0xf)
            .wrapping_add(if with_carry { self.cy as u8 } else { 0 }) > 0xf;
        self.cy = did_wrap;
        self.set_reg(regop, result);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inc(regop) => self.add(regop, 1, false),
            Instruction::Dec(regop) => self.subtract(regop, 1, false),
            Instruction::And(regop) => self.and(ArithmeticOperand::A, self.get_reg(regop)),
            Instruction::AndHL() => self.and(ArithmeticOperand::A, self.read_byte(self.get_hl())),
            Instruction::AndImm() => self.and(ArithmeticOperand::A, self.read_byte(self.pc + 1)),
            Instruction::Add(regop) => self.add(ArithmeticOperand::A, self.get_reg(regop), false),
            Instruction::Adc(regop) => self.add(ArithmeticOperand::A, self.get_reg(regop), true),
            Instruction::AddImm() => self.add(ArithmeticOperand::A, self.read_byte(self.pc + 1), false),
            Instruction::AddHL() => self.add(ArithmeticOperand::A, self.read_byte(self.get_hl()), false),
            Instruction::Sub(regop) => self.subtract(ArithmeticOperand::A, self.get_reg(regop), false),
            Instruction::Sbc(regop) => self.subtract(ArithmeticOperand::A, self.get_reg(regop), true),
            Instruction::SubImm() => self.subtract(ArithmeticOperand::A, self.read_byte(self.pc + 1), false),
            Instruction::SubHL() => self.subtract(ArithmeticOperand::A, self.read_byte(self.get_hl()), false),
            _ => panic!("Invalid instruction"),
        };
    }

    pub fn dump(&self) {
        println!("Registers {:#04x?}", self.rf);
        println!("flags z:{}, n:{}, h:{}, cy:{}", self.z, self.n, self.h, self.cy);
    }

    pub fn step(&mut self) {
        let instruction_byte = self.read_byte(self.pc);
        if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute_instruction(instruction);
            self.pc += instruction.size() as u16;
        }
    }
}

#[cfg(test)]
mod test;

