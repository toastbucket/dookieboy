// Test module for Cpu.
//
// Provides unit tests to test basic functionality
// of the Cpu module.

use super::*;

// Verify incrementing registers
#[test]
fn test_all_increments() {
    let mut cpu = Cpu::new();
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
    assert_eq!(cpu.rf[ArithmeticOperand::B as usize], 1);
    assert_eq!(cpu.rf[ArithmeticOperand::C as usize], 1);
    assert_eq!(cpu.rf[ArithmeticOperand::D as usize], 1);
    assert_eq!(cpu.rf[ArithmeticOperand::E as usize], 1);
    assert_eq!(cpu.rf[ArithmeticOperand::H as usize], 1);
    assert_eq!(cpu.rf[ArithmeticOperand::L as usize], 1);
    assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 1);
}

// Verify overflow when incrementing 0xff
#[test]
fn test_increment_overflow() {
    let mut cpu = Cpu::new();
    const INSTRUCTIONS_LEN: usize = 1;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Inc(ArithmeticOperand::B).as_byte(),
    ];
    cpu.load_test_ram(&test_ram);
    cpu.rf[ArithmeticOperand::B as usize] = 0xff;
    cpu.step();
    assert_eq!(cpu.rf[ArithmeticOperand::B as usize], 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
}

// Verify adding to registers
#[test]
fn test_add() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 1);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 0;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 1);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_add_overflow() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 0xff);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 1;
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
    }
}

// Verify adding to registers with carry
#[test]
fn test_add_carry() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 1);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 0;
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 2);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_add_carry_overflow() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 0xff);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 0;
        cpu.cy = true;
        cpu.step();
        cpu.dump();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
    }
}

// Verify adding immediates
#[test]
fn test_add_immediate() {
    let mut cpu = Cpu::new();
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AddImm().as_byte(),
        0x10,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0x10);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
}

// Verify adding immediates with overflow
#[test]
fn test_add_immediate_overflow() {
    let mut cpu = Cpu::new();
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AddImm().as_byte(),
        0xff,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.rf[ArithmeticOperand::A as usize] = 1;
    cpu.step();
    assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
}

// Verify decrementing registers
#[test]
fn test_all_decrements() {
    let mut cpu = Cpu::new();
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

    cpu.rf.iter_mut().for_each(|x| *x = 1);
    cpu.load_test_ram(&test_ram);
    for i in 0..test_ram.len() {
        cpu.step();
    }
    assert_eq!(cpu.rf[ArithmeticOperand::B as usize], 0);
    assert_eq!(cpu.rf[ArithmeticOperand::C as usize], 0);
    assert_eq!(cpu.rf[ArithmeticOperand::D as usize], 0);
    assert_eq!(cpu.rf[ArithmeticOperand::E as usize], 0);
    assert_eq!(cpu.rf[ArithmeticOperand::H as usize], 0);
    assert_eq!(cpu.rf[ArithmeticOperand::L as usize], 0);
    assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
}

// Verify subtracting registers
#[test]
fn test_sub() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 1);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 1;
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
    }
}

// Verify subtracting registers with overflow
#[test]
fn test_sub_overflow() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 1);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 0;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0xff);
    }
}

// Verify subtracting registers with carry
#[test]
fn test_sub_carry() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 1);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 2;
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_sub_carry_overflow() {
    let mut cpu = Cpu::new();
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
    cpu.rf.iter_mut().for_each(|x| *x = 1);

    for i in 0..test_ram.len() {
        cpu.rf[ArithmeticOperand::A as usize] = 0;
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
        assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0xfe);
    }
}

// Verify subtracting immediates
#[test]
fn test_sub_immediate() {
    let mut cpu = Cpu::new();
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SubImm().as_byte(),
        0x10,
    ];
    cpu.rf[ArithmeticOperand::A as usize] = 0x10;
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
}

// Verify subtracting immediates with overflow
#[test]
fn test_sub_immediate_overflow() {
    let mut cpu = Cpu::new();
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SubImm().as_byte(),
        0x01,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.rf[ArithmeticOperand::A as usize] = 0;
    cpu.step();
    assert_eq!(cpu.rf[ArithmeticOperand::A as usize], 0xff);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
}

