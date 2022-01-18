// Test module for Cpu.
//
// Provides unit tests to test basic functionality
// of the Cpu module.

use super::*;

// Verify incrementing registers
#[test]
fn test_all_increments() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 7;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Inc(ArithmeticOperand::B).as_byte(),
        Instruction::Inc(ArithmeticOperand::C).as_byte(),
        Instruction::Inc(ArithmeticOperand::D).as_byte(),
        Instruction::Inc(ArithmeticOperand::E).as_byte(),
        Instruction::Inc(ArithmeticOperand::H).as_byte(),
        Instruction::Inc(ArithmeticOperand::L).as_byte(),
        Instruction::Inc(ArithmeticOperand::A).as_byte(),
    ];
    cpu.load_test_ram(&test_ram);
    for i in 0..test_ram.len() {
        cpu.step();
    }
    assert_eq!(cpu.get_reg(ArithmeticOperand::B), 1);
    assert_eq!(cpu.get_reg(ArithmeticOperand::C), 1);
    assert_eq!(cpu.get_reg(ArithmeticOperand::D), 1);
    assert_eq!(cpu.get_reg(ArithmeticOperand::E), 1);
    assert_eq!(cpu.get_reg(ArithmeticOperand::H), 1);
    assert_eq!(cpu.get_reg(ArithmeticOperand::L), 1);
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 1);
}

// Verify overflow when incrementing 0xff
#[test]
fn test_increment_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 1;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Inc(ArithmeticOperand::B).as_byte(),
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(ArithmeticOperand::B, 0xff);
    cpu.step();
    assert_eq!(cpu.get_reg(ArithmeticOperand::B), 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
}

// Verify adding to registers
#[test]
fn test_add() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Add(ArithmeticOperand::B).as_byte(),
        Instruction::Add(ArithmeticOperand::C).as_byte(),
        Instruction::Add(ArithmeticOperand::D).as_byte(),
        Instruction::Add(ArithmeticOperand::E).as_byte(),
        Instruction::Add(ArithmeticOperand::H).as_byte(),
        Instruction::Add(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 0);
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 1);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_add_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Add(ArithmeticOperand::B).as_byte(),
        Instruction::Add(ArithmeticOperand::C).as_byte(),
        Instruction::Add(ArithmeticOperand::D).as_byte(),
        Instruction::Add(ArithmeticOperand::E).as_byte(),
        Instruction::Add(ArithmeticOperand::H).as_byte(),
        Instruction::Add(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xff);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 1);
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    }
}

// Verify adding to registers with carry
#[test]
fn test_add_carry() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Adc(ArithmeticOperand::B).as_byte(),
        Instruction::Adc(ArithmeticOperand::C).as_byte(),
        Instruction::Adc(ArithmeticOperand::D).as_byte(),
        Instruction::Adc(ArithmeticOperand::E).as_byte(),
        Instruction::Adc(ArithmeticOperand::H).as_byte(),
        Instruction::Adc(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 0);
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 2);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_add_carry_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Adc(ArithmeticOperand::B).as_byte(),
        Instruction::Adc(ArithmeticOperand::C).as_byte(),
        Instruction::Adc(ArithmeticOperand::D).as_byte(),
        Instruction::Adc(ArithmeticOperand::E).as_byte(),
        Instruction::Adc(ArithmeticOperand::H).as_byte(),
        Instruction::Adc(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xff);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 0);
        cpu.cy = true;
        cpu.step();
        cpu.dump();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    }
}

// Verify adding immediates
#[test]
fn test_add_immediate() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AddImm().as_byte(),
        0x10,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0x10);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
}

// Verify adding immediates with overflow
#[test]
fn test_add_immediate_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AddImm().as_byte(),
        0xff,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(ArithmeticOperand::A, 1);
    cpu.step();
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
}

// Verify decrementing registers
#[test]
fn test_all_decrements() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 7;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Dec(ArithmeticOperand::B).as_byte(),
        Instruction::Dec(ArithmeticOperand::C).as_byte(),
        Instruction::Dec(ArithmeticOperand::D).as_byte(),
        Instruction::Dec(ArithmeticOperand::E).as_byte(),
        Instruction::Dec(ArithmeticOperand::H).as_byte(),
        Instruction::Dec(ArithmeticOperand::L).as_byte(),
        Instruction::Dec(ArithmeticOperand::A).as_byte(),
    ];

    cpu.set_all_regs(1);
    cpu.load_test_ram(&test_ram);
    for i in 0..test_ram.len() {
        cpu.step();
    }
    assert_eq!(cpu.get_reg(ArithmeticOperand::B), 0);
    assert_eq!(cpu.get_reg(ArithmeticOperand::C), 0);
    assert_eq!(cpu.get_reg(ArithmeticOperand::D), 0);
    assert_eq!(cpu.get_reg(ArithmeticOperand::E), 0);
    assert_eq!(cpu.get_reg(ArithmeticOperand::H), 0);
    assert_eq!(cpu.get_reg(ArithmeticOperand::L), 0);
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
}

// Verify subtracting registers
#[test]
fn test_sub() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sub(ArithmeticOperand::B).as_byte(),
        Instruction::Sub(ArithmeticOperand::C).as_byte(),
        Instruction::Sub(ArithmeticOperand::D).as_byte(),
        Instruction::Sub(ArithmeticOperand::E).as_byte(),
        Instruction::Sub(ArithmeticOperand::H).as_byte(),
        Instruction::Sub(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 1);
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    }
}

// Verify subtracting registers with overflow
#[test]
fn test_sub_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sub(ArithmeticOperand::B).as_byte(),
        Instruction::Sub(ArithmeticOperand::C).as_byte(),
        Instruction::Sub(ArithmeticOperand::D).as_byte(),
        Instruction::Sub(ArithmeticOperand::E).as_byte(),
        Instruction::Sub(ArithmeticOperand::H).as_byte(),
        Instruction::Sub(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 0);
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0xff);
    }
}

// Verify subtracting registers with carry
#[test]
fn test_sub_carry() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sbc(ArithmeticOperand::B).as_byte(),
        Instruction::Sbc(ArithmeticOperand::C).as_byte(),
        Instruction::Sbc(ArithmeticOperand::D).as_byte(),
        Instruction::Sbc(ArithmeticOperand::E).as_byte(),
        Instruction::Sbc(ArithmeticOperand::H).as_byte(),
        Instruction::Sbc(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 2);
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_sub_carry_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sbc(ArithmeticOperand::B).as_byte(),
        Instruction::Sbc(ArithmeticOperand::C).as_byte(),
        Instruction::Sbc(ArithmeticOperand::D).as_byte(),
        Instruction::Sbc(ArithmeticOperand::E).as_byte(),
        Instruction::Sbc(ArithmeticOperand::H).as_byte(),
        Instruction::Sbc(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 0);
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0xfe);
    }
}

// Verify subtracting immediates
#[test]
fn test_sub_immediate() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SubImm().as_byte(),
        0x10,
    ];
    cpu.set_reg(ArithmeticOperand::A, 0x10);
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
}

// Verify subtracting immediates with overflow
#[test]
fn test_sub_immediate_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SubImm().as_byte(),
        0x01,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(ArithmeticOperand::A, 0);
    cpu.step();
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0xff);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
}

// Verify anding registers
#[test]
fn test_and() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::And(ArithmeticOperand::B).as_byte(),
        Instruction::And(ArithmeticOperand::C).as_byte(),
        Instruction::And(ArithmeticOperand::D).as_byte(),
        Instruction::And(ArithmeticOperand::E).as_byte(),
        Instruction::And(ArithmeticOperand::H).as_byte(),
        Instruction::And(ArithmeticOperand::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xaa);

    for i in 0..test_ram.len() {
        cpu.set_reg(ArithmeticOperand::A, 0x55);
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0);
    }
}

// Verify anding registers and immediates
#[test]
fn test_and_imm() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AndImm().as_byte(),
        0x05,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(ArithmeticOperand::A, 0x07);
    cpu.step();

    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, false);
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0x05);
}

// Verify anding registers and memory
#[test]
fn test_and_hl() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AndHL().as_byte(),
        0x05,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(ArithmeticOperand::A, 0x07);
    cpu.set_reg(ArithmeticOperand::H, 0x00);
    cpu.set_reg(ArithmeticOperand::L, 0x01);
    cpu.step();

    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, false);
    assert_eq!(cpu.get_reg(ArithmeticOperand::A), 0x05);
}
