// src/cpu.rs (you know, just in case you don't know what file ur in lmao)

mod instruction;

use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;
use crate::cpu::instruction::Instruction;
use crate::mmu::Mmu;

#[derive(Debug, Copy, Clone)]
pub enum Register8Bit {
    A = 0, B, C, D, E, H, L
}

#[derive(Debug, Copy, Clone)]
pub enum Register16Bit {
    BC = 1, DE = 3, HL = 5
}

const NUM_GP_REGS: usize = 7;

#[cfg(test)]
const TEST_RAM_SIZE: usize = 128;

pub struct Cpu {
    rf: [u8; NUM_GP_REGS],
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
            rf: [0; NUM_GP_REGS],
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
            panic!("Address {:#06x} outside of test rom size {}", addr, TEST_RAM_SIZE);
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
            panic!("Address {:#06x} outside of test rom size {}", addr, TEST_RAM_SIZE);
        }

        self.test_ram[addr as usize] = val;
    }

    #[cfg(not(test))]
    fn write_byte(&mut self, addr: u16, val: u8) {
        self.mmu.borrow_mut().mem_write_byte(addr, val);
    }

    fn get_reg(&self, regop: Register8Bit) -> u8 {
        self.rf[regop as usize]
    }

    fn get_reg_16(&self, regop: Register16Bit) -> u16 {
        let idx = regop as usize;
        ((self.rf[idx] as u16) << 8) | (self.rf[idx + 1] as u16)
    }

    fn set_reg(&mut self, regop: Register8Bit, val: u8) {
        self.rf[regop as usize] = val; 
    }

    fn set_reg_16(&mut self, regop: Register16Bit, val: u16) {
        let idx = regop as usize;
        self.rf[idx] = (val >> 8 & 0xff) as u8;
        self.rf[idx + 1] = (val & 0xff) as u8;
    }

    fn set_all_regs(&mut self, val: u8) {
        self.rf.iter_mut().for_each(|x| *x = val);
    }

    fn and(&mut self, regop: Register8Bit, operand: u8) {
        let result = self.get_reg(regop) & operand;
        self.z = result == 0;
        self.n = false;
        self.h = true;
        self.cy = false;
        self.set_reg(regop, result);
    }

    fn subtract(&mut self, regop: Register8Bit, operand: u8, with_carry: bool) {
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

    fn add(&mut self, regop: Register8Bit, operand: u8, with_carry: bool) {
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

    fn load_register(&mut self, dest: Register8Bit, src: Register8Bit) {
        self.set_reg(dest, self.get_reg(src));
    }

    fn ld_to_mem(&mut self, regop: Register8Bit, addr: u16) {
        self.write_byte(addr, self.get_reg(regop));
    }

    fn ld_from_mem(&mut self, regop: Register8Bit, addr: u16) {
        self.set_reg(regop, self.read_byte(addr));
    }

    // return next PC
    fn execute_instruction(&mut self, instruction: Instruction) -> u16 {
        let pc = self.pc;

        match instruction {
            Instruction::Noop() => {
                pc + 1
            },
            Instruction::Inc(regop) => {
                self.add(regop, 1, false);
                pc + 1
            },
            Instruction::Dec(regop) => {
                self.subtract(regop, 1, false);
                pc + 1
            },
            Instruction::And(regop) => {
                self.and(Register8Bit::A, self.get_reg(regop));
                pc + 1
            },
            Instruction::AndFromMem() => {
                self.and(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)));
                pc + 1
            },
            Instruction::AndImm() => {
                self.and(Register8Bit::A, self.read_byte(pc + 1));
                pc + 2
            },
            Instruction::Add(regop) => {
                self.add(Register8Bit::A, self.get_reg(regop), false);
                pc + 1
            },
            Instruction::Adc(regop) => {
                self.add(Register8Bit::A, self.get_reg(regop), true);
                pc + 1
            },
            Instruction::AddImm() => {
                self.add(Register8Bit::A, self.read_byte(pc + 1), false);
                pc + 2
            },
            Instruction::AddFromMem() => {
                self.add(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), false);
                pc + 1
            },
            Instruction::AdcFromMem() => {
                self.add(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), true);
                pc + 1
            },
            Instruction::Sub(regop) => {
                self.subtract(Register8Bit::A, self.get_reg(regop), false);
                pc + 1
            },
            Instruction::Sbc(regop) => {
                self.subtract(Register8Bit::A, self.get_reg(regop), true);
                pc + 1
            },
            Instruction::SubImm() => {
                self.subtract(Register8Bit::A, self.read_byte(pc + 1), false);
                pc + 2
            },
            Instruction::SubFromMem() => {
                self.subtract(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), false);
                pc + 1
            },
            Instruction::SbcFromMem() => {
                self.subtract(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), true);
                pc + 1
            },
            Instruction::LdRegister(dest, src) => {
                self.load_register(dest, src);
                pc + 1
            },
            Instruction::LdToMem(regop, pair) => {
                self.ld_to_mem(regop, self.get_reg_16(pair));
                pc + 1
            },
            Instruction::LdFromMem(regop, pair) => {
                self.ld_from_mem(regop, self.get_reg_16(pair));
                pc + 1
            },
            Instruction::LdToMemInc() => {
                self.ld_to_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_add(1));
                pc + 1
            },
            Instruction::LdToMemDec() => {
                self.ld_to_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_sub(1));
                pc + 1
            },
            Instruction::LdFromMemInc() => {
                self.ld_from_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_add(1));
                pc + 1
            },
            Instruction::LdFromMemDec() => {
                self.ld_from_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_sub(1));
                pc + 1
            },
            _ => panic!("Invalid instruction"),
        }
    }

    fn dump_to_string(&self) -> String {
        format!("Registers {:#06x?}\n\
                 PC: {:#06x}\n\
                 SP: {:#06x}\n\
                 flags z:{}, n:{}, h:{}, cy:{}",
                self.rf,
                self.pc,
                self.sp,
                self.z, self.n, self.h, self.cy)
    }

    fn dump_the_dookie(&self) {
        println!("{}", self.dump_to_string());
    }


    pub fn step(&mut self) {
        let instruction_byte = self.read_byte(self.pc);

        match Instruction::from_byte(instruction_byte) {
            Some(instruction) => {
                let new_pc = self.execute_instruction(instruction);
                self.pc = new_pc;
            },
            None => {
                panic!("invalid instruction read from ROM at {:#06x}: {:#04x}\n{}",
                       self.pc, instruction_byte, self.dump_to_string());
            },
        }
    }
}

#[cfg(test)]
mod test;

