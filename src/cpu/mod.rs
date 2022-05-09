// src/cpu.rs (you know, just in case you don't know what file ur in lmao)

mod instruction;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;
use crate::cpu::instruction::{ BranchCondition, Instruction };
use crate::mmu::Mmu;

#[derive(Debug, Copy, Clone)]
pub enum Register8Bit {
    A = 0, F, B, C, D, E, H, L,
}

#[derive(Debug, Copy, Clone)]
pub enum Register16Bit {
    AF= 0,
    BC = 2,
    DE = 4,
    HL = 6,
    SP = 8,
}

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Debug, Copy, Clone)]
pub enum RstVec {
    ZERO = 0x0000,
    ONE = 0x0008,
    TWO = 0x0010,
    THREE = 0x0018,
    FOUR = 0x0020,
    FIVE = 0x0028,
    SIX = 0x0030,
    SEVEN = 0x0038,
}

const NUM_GP_REGS: usize = 10;

#[cfg(test)]
const TEST_RAM_SIZE: usize = 128;

#[cfg(test)]
struct TestRam {
    ram: [u8; TEST_RAM_SIZE],
}

#[cfg(test)]
impl TestRam {
    fn new() -> TestRam {
        TestRam {
            ram: [0; TEST_RAM_SIZE],
        }
    }

    fn load_from_slice(&mut self, data: &[u8]) {
        self.ram[..data.len()].clone_from_slice(data);
    }
}

#[cfg(test)]
impl Memory for TestRam {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        if addr > TEST_RAM_SIZE as u16{
            panic!("Address {:#06x} outside of test rom size {}", addr, TEST_RAM_SIZE);
        }

        self.ram[addr as usize]
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        if addr > TEST_RAM_SIZE as u16{
            panic!("Address {:#06x} outside of test rom size {}", addr, TEST_RAM_SIZE);
        }

        self.ram[addr as usize] = val;
    }
}

pub struct Cpu {
    rf: [u8; NUM_GP_REGS],
    pc: u16,
    mmu: Rc<RefCell<Mmu>>,
    cycles: usize,

    #[cfg(test)]
    test_ram: TestRam,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dump_to_string())
    }
}

impl Cpu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Cpu {
        Cpu {
            rf: [0; NUM_GP_REGS],
            pc: 0,
            mmu: mmu,
            cycles: 0,

            #[cfg(test)]
            test_ram: TestRam::new(),
        }
    }

    #[cfg(test)]
    fn load_test_ram(&mut self, data: &[u8]) {
        self.test_ram.load_from_slice(data);
    }

    #[cfg(test)]
    fn read_byte(&self, addr: u16) -> u8 {
        self.test_ram.mem_read_byte(addr)
    }

    #[cfg(test)]
    fn write_byte(&mut self, addr: u16, val: u8) {
        self.test_ram.mem_write_byte(addr, val);
    }

    #[cfg(test)]
    fn read_word(&self, addr: u16) -> u16 {
        self.test_ram.mem_read_word_le(addr)
    }

    #[cfg(test)]
    fn write_word(&mut self, addr: u16, val: u16) {
        self.test_ram.mem_write_word_le(addr, val);
    }

    #[cfg(not(test))]
    fn read_byte(&self, addr: u16) -> u8 {
        self.mmu.borrow().mem_read_byte(addr)
    }

    #[cfg(not(test))]
    fn write_byte(&mut self, addr: u16, val: u8) {
        self.mmu.borrow_mut().mem_write_byte(addr, val);
    }

    #[cfg(not(test))]
    fn read_word(&self, addr: u16) -> u16 {
        self.mmu.borrow_mut().mem_read_word_le(addr)
    }

    #[cfg(not(test))]
    fn write_word(&mut self, addr: u16, val: u16) {
        self.mmu.borrow_mut().mem_write_word_le(addr, val);
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

    fn set_flag(&mut self, flag: Flag, val: bool) {
        let mut flags = self.get_reg(Register8Bit::F);

        if (val) {
            flags |= (1 << (flag as u8));
        } else {
            flags &= !(1 << (flag as u8));
        }
        self.set_reg(Register8Bit::F, flags);
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let bit = (self.get_reg(Register8Bit::F) >> (flag as u8)) & 1;
        bit == 1
    }

    fn set_sp(&mut self, val: u16) {
        self.set_reg_16(Register16Bit::SP, val);
    }

    fn get_sp(&self) -> u16 {
        self.get_reg_16(Register16Bit::SP)
    }

    fn and(&mut self, regop: Register8Bit, operand: u8) {
        let result = self.get_reg(regop) & operand;
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, true);
        self.set_flag(Flag::C, false);
        self.set_reg(regop, result);
    }

    fn or(&mut self, regop: Register8Bit, operand: u8) {
        let result = self.get_reg(regop) | operand;
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
        self.set_reg(regop, result);
    }

    fn xor(&mut self, regop: Register8Bit, operand: u8) {
        let result = self.get_reg(regop) ^ operand;
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
        self.set_reg(regop, result);
    }

    fn cp(&mut self, regop: Register8Bit, operand: u8) {
        // store off old value so we can restore after subtraction. CP
        // instruction does not affect registers, yet our subtraction
        // logic does.
        let old_val = self.get_reg(regop);
        self.subtract(regop, operand, false);
        self.set_reg(regop, old_val);
    }

    /*
     * Handle left rotation and left circular rotation.
     * If RLC (circular), shift left one bit and rotate around
     * bit 7 to bit 0. Carry flag still holds value of bit 7 prior
     * to rotation.
     * If RL, shift left one bit with bit 7 shifting into the carry
     * flag and the carry flag shifting into bit 0.
     */
    fn shift_left(&mut self, regop: Register8Bit, is_rlc: bool) {
        let mut r = self.get_reg(regop);
        let bit7 = (r >> 7) & 1;

        // If RLC, bit 7 goes to C flag and bit 0
        // If RL, bit 7 goes to C flag and C flag
        // goes to bit 0
        let bit0 = if is_rlc {
            bit7
        } else {
            u8::from(self.get_flag(Flag::C))
        };

        r = (r << 1) | bit0;
        self.set_reg(regop, r);

        self.set_flag(Flag::Z, if matches!(regop, Register8Bit::A) { false } else { r == 0 });
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, bit7 == 1);
    }

    /*
     * Handle right rotation and right circular rotation.
     * If RRC (circular), shift right one bit and rotate around
     * bit 0 to bit 7. Carry flag holds value of bit 0 prior
     * to rotation.
     * If RR, shift right one bit with bit 0 shifting into the carry
     * flag and the carry flag shifting into bit 7.
     */
    fn shift_right(&mut self, regop: Register8Bit, is_rrc: bool) {
        let mut r = self.get_reg(regop);
        let bit0 = r & 1;

        // If RRC, bit 0 goes to C flag and bit 7
        // If RR, bit 0 goes to C flag and C flag
        // goes to bit 7
        let bit7 = if is_rrc {
            bit0
        } else {
            u8::from(self.get_flag(Flag::C))
        };

        r = (r >> 1) | (bit7 << 7);
        self.set_reg(regop, r);

        self.set_flag(Flag::Z, if matches!(regop, Register8Bit::A) { false } else { r == 0 });
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, bit0 == 1);
    }

    fn subtract(&mut self, regop: Register8Bit, operand: u8, with_carry: bool) {
        let cy = self.get_flag(Flag::C);
        let (result, did_wrap) = if with_carry {
            let (carry_result, carry_did_wrap) = self.get_reg(regop).overflowing_sub(cy as u8);
            let (operand_result, operand_did_wrap) = carry_result.overflowing_sub(operand);
            (operand_result, carry_did_wrap | operand_did_wrap)
        } else {
            self.get_reg(regop).overflowing_sub(operand)
        };

        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, true);

        let h = ((self.get_reg(regop) as i8) & 0xf)
            .wrapping_sub((operand as i8) & 0xf)
            .wrapping_sub(if with_carry { cy as i8 } else { 0 }) < 0;
        self.set_flag(Flag::H, h);
        self.set_flag(Flag::C, did_wrap);
        self.set_reg(regop, result);
    }

    fn add(&mut self, regop: Register8Bit, operand: u8, with_carry: bool) {
        let cy = self.get_flag(Flag::C);
        let (result, did_wrap) = if with_carry {
            let (carry_result, carry_did_wrap) = self.get_reg(regop).overflowing_add(cy as u8);
            let (operand_result, operand_did_wrap) = carry_result.overflowing_add(operand);
            (operand_result, carry_did_wrap | operand_did_wrap)
        } else {
            self.get_reg(regop).overflowing_add(operand)
        };

        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);

        let h = (self.get_reg(regop) & 0xf)
            .wrapping_add(operand & 0xf)
            .wrapping_add(if with_carry { cy as u8 } else { 0 }) > 0xf;
        self.set_flag(Flag::H, h);
        self.set_flag(Flag::C, did_wrap);
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

    fn push(&mut self, val: u16) {
        self.set_sp(self.get_sp() - 2);
        self.write_word(self.get_sp(), val);
    }

    fn pop(&mut self) -> u16 {
        let val = self.read_word(self.get_sp());
        self.set_sp(self.get_sp() + 2);
        val
    }

    fn should_branch(&self, condition: BranchCondition) -> bool {
        match condition {
            BranchCondition::NZ => self.get_flag(Flag::Z) == false,
            BranchCondition::Z => self.get_flag(Flag::Z) == true,
            BranchCondition::NC => self.get_flag(Flag::C) == false,
            BranchCondition::C => self.get_flag(Flag::C) == true,
            BranchCondition::NONE => true,
        }
    }

    // execute instruction
    // return tuple containing (next_pc, # cycles used)
    fn execute_instruction(&mut self, instruction: Instruction) -> (u16, usize) {
        let pc = self.pc;

        match instruction {
            Instruction::Noop() => {
                (pc + 1, 1)
            },
            Instruction::Inc(regop) => {
                self.add(regop, 1, false);
                (pc + 1, 1)
            },
            Instruction::Dec(regop) => {
                self.subtract(regop, 1, false);
                (pc + 1, 1)
            },
            Instruction::And(regop) => {
                self.and(Register8Bit::A, self.get_reg(regop));
                (pc + 1, 1)
            },
            Instruction::AndFromMem() => {
                self.and(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)));
                (pc + 1, 2)
            },
            Instruction::AndImm() => {
                self.and(Register8Bit::A, self.read_byte(pc + 1));
                (pc + 2, 2)
            },
            Instruction::Or(regop) => {
                self.or(Register8Bit::A, self.get_reg(regop));
                (pc + 1, 1)
            },
            Instruction::OrFromMem() => {
                self.or(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)));
                (pc + 1, 2)
            },
            Instruction::OrImm() => {
                self.or(Register8Bit::A, self.read_byte(pc + 1));
                (pc + 2, 2)
            },
            Instruction::Xor(regop) => {
                self.xor(Register8Bit::A, self.get_reg(regop));
                (pc + 1, 1)
            },
            Instruction::XorFromMem() => {
                self.xor(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)));
                (pc + 1, 2)
            },
            Instruction::XorImm() => {
                self.xor(Register8Bit::A, self.read_byte(pc + 1));
                (pc + 2, 2)
            },
            Instruction::Cp(regop) => {
                self.cp(Register8Bit::A, self.get_reg(regop));
                (pc + 1, 1)
            },
            Instruction::CpFromMem() => {
                self.cp(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)));
                (pc + 1, 2)
            },
            Instruction::CpImm() => {
                self.cp(Register8Bit::A, self.read_byte(pc + 1));
                (pc + 2, 2)
            },
            Instruction::Rla() => {
                self.shift_left(Register8Bit::A, false);
                (pc + 1, 1)
            },
            Instruction::Rlca() => {
                self.shift_left(Register8Bit::A, true);
                (pc + 1, 1)
            },
            Instruction::Rra() => {
                self.shift_right(Register8Bit::A, false);
                (pc + 1, 1)
            },
            Instruction::Rrca() => {
                self.shift_right(Register8Bit::A, true);
                (pc + 1, 1)
            },
            Instruction::Add(regop) => {
                self.add(Register8Bit::A, self.get_reg(regop), false);
                (pc + 1, 1)
            },
            Instruction::Adc(regop) => {
                self.add(Register8Bit::A, self.get_reg(regop), true);
                (pc + 1, 1)
            },
            Instruction::AddImm() => {
                self.add(Register8Bit::A, self.read_byte(pc + 1), false);
                (pc + 2, 2)
            },
            Instruction::AddFromMem() => {
                self.add(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), false);
                (pc + 1, 2)
            },
            Instruction::AdcFromMem() => {
                self.add(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), true);
                (pc + 1, 2)
            },
            Instruction::Sub(regop) => {
                self.subtract(Register8Bit::A, self.get_reg(regop), false);
                (pc + 1, 1)
            },
            Instruction::Sbc(regop) => {
                self.subtract(Register8Bit::A, self.get_reg(regop), true);
                (pc + 1, 1)
            },
            Instruction::SubImm() => {
                self.subtract(Register8Bit::A, self.read_byte(pc + 1), false);
                (pc + 2, 2)
            },
            Instruction::SubFromMem() => {
                self.subtract(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), false);
                (pc + 1, 2)
            },
            Instruction::SbcFromMem() => {
                self.subtract(Register8Bit::A, self.read_byte(self.get_reg_16(Register16Bit::HL)), true);
                (pc + 1, 2)
            },
            Instruction::LdRegister(dest, src) => {
                self.load_register(dest, src);
                (pc + 1, 1)
            },
            Instruction::LdImm(regop) => {
                self.set_reg(regop, self.read_byte(pc + 1));
                (pc + 2, 2)
            },
            Instruction::LdToMem(regop, pair) => {
                self.ld_to_mem(regop, self.get_reg_16(pair));
                (pc + 1, 2)
            },
            Instruction::LdFromMem(regop, pair) => {
                self.ld_from_mem(regop, self.get_reg_16(pair));
                (pc + 1, 2)
            },
            Instruction::LdToMemInc() => {
                self.ld_to_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_add(1));
                (pc + 1, 2)
            },
            Instruction::LdToMemDec() => {
                self.ld_to_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_sub(1));
                (pc + 1, 2)
            },
            Instruction::LdFromMemInc() => {
                self.ld_from_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_add(1));
                (pc + 1, 2)
            },
            Instruction::LdFromMemDec() => {
                self.ld_from_mem(Register8Bit::A, self.get_reg_16(Register16Bit::HL));
                self.set_reg_16(Register16Bit::HL, self.get_reg_16(Register16Bit::HL).wrapping_sub(1));
                (pc + 1, 2)
            },
            Instruction::LdRegister16Imm(pair) => {
                self.set_reg_16(pair, self.read_word(pc + 1));
                (pc + 3, 3)
            },
            Instruction::JumpAbs(condition) => {
                if self.should_branch(condition) {
                    let addr = (self.read_byte(pc + 1) as u16)
                               | ((self.read_byte(pc + 2) as u16) << 8);
                    (addr, 4)
                } else {
                    (pc + 3, 3)
                }
            },
            Instruction::JumpAbsFromReg() => {
                (self.get_reg_16(Register16Bit::HL), 1)
            },
            Instruction::JumpRel(condition) => {
                if self.should_branch(condition) {
                    // cast as offset i8 to preserve sign
                    // cast PC as i32 to ensure unsigned
                    // cast offset as i32 for arithmetic
                    let offset = self.read_byte(pc + 1) as i8;
                    let addr = ((pc as i32) + (offset as i32) + 2) as u16;
                    (addr, 3)
                } else {
                    (pc + 3, 2)
                }
            },
            Instruction::Push(pair) => {
                let val = self.get_reg_16(pair);
                self.push(val);
                (pc + 1, 4)
            },
            Instruction::Pop(pair) => {
                let val = self.pop();
                self.set_reg_16(pair, val);
                (pc + 1, 3)
            },
            Instruction::Ret(condition) => {
                if self.should_branch(condition) {
                    (self.pop(), if matches!(condition, BranchCondition::NONE) { 4 } else { 5 })
                } else {
                    (pc + 1, 2)
                }
            },
            Instruction::Rst(vec) => {
                self.push(pc);
                (vec as u16, 4)
            },
            Instruction::Call(condition) => {
                if self.should_branch(condition) {
                    self.push(pc + 2);
                    (self.read_word(pc + 1), 6)
                } else {
                    (pc + 3, 3)
                }
            },
            _ => panic!("Invalid instruction"),
        }
    }

    fn dump_to_string(&self) -> String {
        format!("AF: {:#04x} {:#04x}\n\
                 BC: {:#04x} {:#04x}\n\
                 DC: {:#04x} {:#04x}\n\
                 HL: {:#04x} {:#04x}\n\
                 PC: {:#06x}\n\
                 SP: {:#06x}\n\
                 flags z:{}, n:{}, h:{}, cy:{}",
                self.get_reg(Register8Bit::A), self.get_reg(Register8Bit::F),
                self.get_reg(Register8Bit::B), self.get_reg(Register8Bit::C),
                self.get_reg(Register8Bit::D), self.get_reg(Register8Bit::E),
                self.get_reg(Register8Bit::H), self.get_reg(Register8Bit::L),
                self.pc,
                self.get_sp(),
                self.get_flag(Flag::Z),
                self.get_flag(Flag::N),
                self.get_flag(Flag::H),
                self.get_flag(Flag::C))
    }

    pub fn step(&mut self) {
        let instruction_byte = self.read_byte(self.pc);

        match Instruction::from_byte(instruction_byte) {
            Some(instruction) => {
                let (new_pc, cycles) = self.execute_instruction(instruction);
                self.pc = new_pc;
                self.cycles += cycles;
            },
            None => {
                panic!("invalid instruction read from ROM at {:#06x}: {:#04x}\n{}",
                       self.pc, instruction_byte, self.dump_to_string());
            },
        }
    }

    /*
     * default DMG powerup state according to
     * https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
     */
    pub fn reset(&mut self) {
        self.set_reg(Register8Bit::A, 0x01);
        self.set_reg(Register8Bit::F, 0x00);
        self.set_reg(Register8Bit::B, 0x00);
        self.set_reg(Register8Bit::C, 0x13);
        self.set_reg(Register8Bit::D, 0x00);
        self.set_reg(Register8Bit::E, 0xd8);
        self.set_reg(Register8Bit::H, 0x01);
        self.set_reg(Register8Bit::L, 0x4d);

        self.set_flag(Flag::Z, true);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);

        self.pc = 0x0100;
        self.set_sp(0xfffe);
    }
}

#[cfg(test)]
mod test;

