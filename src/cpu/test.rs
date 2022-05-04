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
        Instruction::Inc(Register8Bit::B).as_byte(),
        Instruction::Inc(Register8Bit::C).as_byte(),
        Instruction::Inc(Register8Bit::D).as_byte(),
        Instruction::Inc(Register8Bit::E).as_byte(),
        Instruction::Inc(Register8Bit::H).as_byte(),
        Instruction::Inc(Register8Bit::L).as_byte(),
        Instruction::Inc(Register8Bit::A).as_byte(),
    ];
    cpu.load_test_ram(&test_ram);
    for i in 0..test_ram.len() {
        cpu.step();
    }
    assert_eq!(cpu.get_reg(Register8Bit::B), 1);
    assert_eq!(cpu.get_reg(Register8Bit::C), 1);
    assert_eq!(cpu.get_reg(Register8Bit::D), 1);
    assert_eq!(cpu.get_reg(Register8Bit::E), 1);
    assert_eq!(cpu.get_reg(Register8Bit::H), 1);
    assert_eq!(cpu.get_reg(Register8Bit::L), 1);
    assert_eq!(cpu.get_reg(Register8Bit::A), 1);
}

// Verify overflow when incrementing 0xff
#[test]
fn test_increment_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 1;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Inc(Register8Bit::B).as_byte(),
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::B, 0xff);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::B), 0);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), true);
}

// Verify adding to registers
#[test]
fn test_add() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Add(Register8Bit::B).as_byte(),
        Instruction::Add(Register8Bit::C).as_byte(),
        Instruction::Add(Register8Bit::D).as_byte(),
        Instruction::Add(Register8Bit::E).as_byte(),
        Instruction::Add(Register8Bit::H).as_byte(),
        Instruction::Add(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), false);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 1);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_add_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Add(Register8Bit::B).as_byte(),
        Instruction::Add(Register8Bit::C).as_byte(),
        Instruction::Add(Register8Bit::D).as_byte(),
        Instruction::Add(Register8Bit::E).as_byte(),
        Instruction::Add(Register8Bit::H).as_byte(),
        Instruction::Add(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xff);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 1);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), true);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), true);
        assert_eq!(cpu.get_flag(Flag::C), true);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    }
}

// Verify adding to registers with carry
#[test]
fn test_add_carry() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Adc(Register8Bit::B).as_byte(),
        Instruction::Adc(Register8Bit::C).as_byte(),
        Instruction::Adc(Register8Bit::D).as_byte(),
        Instruction::Adc(Register8Bit::E).as_byte(),
        Instruction::Adc(Register8Bit::H).as_byte(),
        Instruction::Adc(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0);
        cpu.set_flag(Flag::C, true);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), false);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 2);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_add_carry_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Adc(Register8Bit::B).as_byte(),
        Instruction::Adc(Register8Bit::C).as_byte(),
        Instruction::Adc(Register8Bit::D).as_byte(),
        Instruction::Adc(Register8Bit::E).as_byte(),
        Instruction::Adc(Register8Bit::H).as_byte(),
        Instruction::Adc(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xff);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0);
        cpu.set_flag(Flag::C, true);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), true);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), true);
        assert_eq!(cpu.get_flag(Flag::C), true);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0);
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
    assert_eq!(cpu.get_reg(Register8Bit::A), 0x10);
    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
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
    cpu.set_reg(Register8Bit::A, 1);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), true);
}

// Verify adding values from memory
#[test]
fn test_add_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AddFromMem().as_byte(),
        0x01,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 1);
    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify decrementing registers
#[test]
fn test_all_decrements() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 7;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Dec(Register8Bit::B).as_byte(),
        Instruction::Dec(Register8Bit::C).as_byte(),
        Instruction::Dec(Register8Bit::D).as_byte(),
        Instruction::Dec(Register8Bit::E).as_byte(),
        Instruction::Dec(Register8Bit::H).as_byte(),
        Instruction::Dec(Register8Bit::L).as_byte(),
        Instruction::Dec(Register8Bit::A).as_byte(),
    ];

    cpu.set_all_regs(1);
    cpu.load_test_ram(&test_ram);
    for i in 0..test_ram.len() {
        cpu.step();
    }
    assert_eq!(cpu.get_reg(Register8Bit::B), 0);
    assert_eq!(cpu.get_reg(Register8Bit::C), 0);
    assert_eq!(cpu.get_reg(Register8Bit::D), 0);
    assert_eq!(cpu.get_reg(Register8Bit::E), 0);
    assert_eq!(cpu.get_reg(Register8Bit::H), 0);
    assert_eq!(cpu.get_reg(Register8Bit::L), 0);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
}

// Verify subtracting registers
#[test]
fn test_sub() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sub(Register8Bit::B).as_byte(),
        Instruction::Sub(Register8Bit::C).as_byte(),
        Instruction::Sub(Register8Bit::D).as_byte(),
        Instruction::Sub(Register8Bit::E).as_byte(),
        Instruction::Sub(Register8Bit::H).as_byte(),
        Instruction::Sub(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 1);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), true);
        assert_eq!(cpu.get_flag(Flag::N), true);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    }
}

// Verify subtracting registers with overflow
#[test]
fn test_sub_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sub(Register8Bit::B).as_byte(),
        Instruction::Sub(Register8Bit::C).as_byte(),
        Instruction::Sub(Register8Bit::D).as_byte(),
        Instruction::Sub(Register8Bit::E).as_byte(),
        Instruction::Sub(Register8Bit::H).as_byte(),
        Instruction::Sub(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), false);
        assert_eq!(cpu.get_flag(Flag::N), true);
        assert_eq!(cpu.get_flag(Flag::H), true);
        assert_eq!(cpu.get_flag(Flag::C), true);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0xff);
    }
}

// Verify subtracting registers with carry
#[test]
fn test_sub_carry() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sbc(Register8Bit::B).as_byte(),
        Instruction::Sbc(Register8Bit::C).as_byte(),
        Instruction::Sbc(Register8Bit::D).as_byte(),
        Instruction::Sbc(Register8Bit::E).as_byte(),
        Instruction::Sbc(Register8Bit::H).as_byte(),
        Instruction::Sbc(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 2);
        cpu.set_flag(Flag::C, true);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), true);
        assert_eq!(cpu.get_flag(Flag::N), true);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    }
}

// Verify adding to registers with overflow
#[test]
fn test_sub_carry_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Sbc(Register8Bit::B).as_byte(),
        Instruction::Sbc(Register8Bit::C).as_byte(),
        Instruction::Sbc(Register8Bit::D).as_byte(),
        Instruction::Sbc(Register8Bit::E).as_byte(),
        Instruction::Sbc(Register8Bit::H).as_byte(),
        Instruction::Sbc(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0);
        cpu.set_flag(Flag::C, true);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), false);
        assert_eq!(cpu.get_flag(Flag::N), true);
        assert_eq!(cpu.get_flag(Flag::H), true);
        assert_eq!(cpu.get_flag(Flag::C), true);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0xfe);
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
    cpu.set_reg(Register8Bit::A, 0x10);
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
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
    cpu.set_reg(Register8Bit::A, 0);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xff);
    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), true);
}

// Verify subtracting immediates
#[test]
fn test_sub_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SubFromMem().as_byte(),
        0x10,
    ];
    cpu.set_reg(Register8Bit::A, 0x10);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify anding registers
#[test]
fn test_and() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::And(Register8Bit::B).as_byte(),
        Instruction::And(Register8Bit::C).as_byte(),
        Instruction::And(Register8Bit::D).as_byte(),
        Instruction::And(Register8Bit::E).as_byte(),
        Instruction::And(Register8Bit::H).as_byte(),
        Instruction::And(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xaa);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0x55);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), true);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), true);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0);
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
    cpu.set_reg(Register8Bit::A, 0x07);
    cpu.step();

    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0x05);
}

// Verify anding registers and memory
#[test]
fn test_and_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AndFromMem().as_byte(),
        0x05,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0x07);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();

    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0x05);
}

// Verify oring registers
#[test]
fn test_or() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Or(Register8Bit::B).as_byte(),
        Instruction::Or(Register8Bit::C).as_byte(),
        Instruction::Or(Register8Bit::D).as_byte(),
        Instruction::Or(Register8Bit::E).as_byte(),
        Instruction::Or(Register8Bit::H).as_byte(),
        Instruction::Or(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xaa);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0x55);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), false);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0xff);
    }
}

// Verify oring registers and immediates
#[test]
fn test_or_imm() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::OrImm().as_byte(),
        0x55,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0xaa);
    cpu.step();

    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xff);
}

// Verify oring registers and memory
#[test]
fn test_or_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::OrFromMem().as_byte(),
        0x55,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0xaa);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();

    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xff);
}

// Verify xoring registers
#[test]
fn test_xor() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Xor(Register8Bit::B).as_byte(),
        Instruction::Xor(Register8Bit::C).as_byte(),
        Instruction::Xor(Register8Bit::D).as_byte(),
        Instruction::Xor(Register8Bit::E).as_byte(),
        Instruction::Xor(Register8Bit::H).as_byte(),
        Instruction::Xor(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(0xa5);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 0x55);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), false);
        assert_eq!(cpu.get_flag(Flag::N), false);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 0xf0);
    }
}

// Verify xoring registers and immediates
#[test]
fn test_xor_imm() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::XorImm().as_byte(),
        0x55,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0xa5);
    cpu.step();

    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xf0);
}

// Verify xoring registers and memory
#[test]
fn test_xor_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::XorFromMem().as_byte(),
        0x55,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0xa5);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();

    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xf0);
}

// Verify comparing registers
#[test]
fn test_cp() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Cp(Register8Bit::B).as_byte(),
        Instruction::Cp(Register8Bit::C).as_byte(),
        Instruction::Cp(Register8Bit::D).as_byte(),
        Instruction::Cp(Register8Bit::E).as_byte(),
        Instruction::Cp(Register8Bit::H).as_byte(),
        Instruction::Cp(Register8Bit::L).as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_all_regs(1);

    for i in 0..test_ram.len() {
        cpu.set_reg(Register8Bit::A, 1);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Z), true);
        assert_eq!(cpu.get_flag(Flag::N), true);
        assert_eq!(cpu.get_flag(Flag::H), false);
        assert_eq!(cpu.get_flag(Flag::C), false);
        assert_eq!(cpu.get_reg(Register8Bit::A), 1);
    }
}

// Verify comparing immediates
#[test]
fn test_cp_immediate() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::CpImm().as_byte(),
        0x01,
    ];
    cpu.set_reg(Register8Bit::A, 1);
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 1);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify from memory
#[test]
fn test_cp_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::CpFromMem().as_byte(),
        0x01,
    ];
    cpu.set_reg(Register8Bit::A, 1);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 1);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify adding values from memory with carry
#[test]
fn test_adc_mem_carry() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AdcFromMem().as_byte(),
        0x01,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 2);
    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify adding values from memory with carry and overflow
#[test]
fn test_adc_mem_carry_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::AdcFromMem().as_byte(),
        0xff,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), true);
}

// Verify subtracting values from memory with carry
#[test]
fn test_sbc_mem_carry() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SbcFromMem().as_byte(),
        0x01,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 2);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.get_flag(Flag::Z), true);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify subtracting values from memory with carry and overflow
#[test]
fn test_sbc_mem_carry_overflow() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::SbcFromMem().as_byte(),
        0x01,
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xfe);
    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), true);
    assert_eq!(cpu.get_flag(Flag::H), true);
    assert_eq!(cpu.get_flag(Flag::C), true);
}

// Verify loading register B
#[test]
fn test_ld_b_a() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 1;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdRegister(Register8Bit::B, Register8Bit::A).as_byte(),
    ];
    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0x69);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::B), 0x69);
}

// Verify loading register B to memory offset @ hl
#[test]
fn test_ld_to_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdToMem(Register8Bit::B, Register16Bit::HL).as_byte(),
        0x24,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::B, 0x69);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.read_byte(0x01), 0x69);
}

// Verify loading register B from memory offset @ hl
#[test]
fn test_ld_from_mem() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdFromMem(Register8Bit::B, Register16Bit::HL).as_byte(),
        0x69,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::B), 0x69);
}

// Verify ldToMem inc HL
#[test]
fn test_ld_to_mem_inc() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdToMemInc().as_byte(),
        0x24,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0x69);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.read_byte(0x01), 0x69);
    assert_eq!(cpu.get_reg_16(Register16Bit::HL), 0x02);
}

// Verify ldToMem dec HL
#[test]
fn test_ld_to_mem_dec() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdToMemDec().as_byte(),
        0x24,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::A, 0x69);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.read_byte(0x01), 0x69);
    assert_eq!(cpu.get_reg_16(Register16Bit::HL), 0x00);
}

// Verify ldFromMem inc HL
#[test]
fn test_ld_from_mem_inc() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdFromMemInc().as_byte(),
        0x69,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0x69);
    assert_eq!(cpu.get_reg_16(Register16Bit::HL), 0x02);
}

// Verify ldFromMem dec HL
#[test]
fn test_ld_from_mem_dec() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 2;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::LdFromMemDec().as_byte(),
        0x69,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg(Register8Bit::H, 0x00);
    cpu.set_reg(Register8Bit::L, 0x01);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0x69);
    assert_eq!(cpu.get_reg_16(Register16Bit::HL), 0x00);
}

// Verify noop
#[test]
fn test_noop() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 1;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Noop().as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.step();
    for r in cpu.rf {
        assert_eq!(r, 0x00);
    }
    assert_eq!(cpu.get_flag(Flag::Z), false);
    assert_eq!(cpu.get_flag(Flag::N), false);
    assert_eq!(cpu.get_flag(Flag::H), false);
    assert_eq!(cpu.get_flag(Flag::C), false);
}

// Verify jump nz
#[test]
fn test_jp_nz() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpAbs(BranchCondition::NZ).as_byte(),
        0xa5,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_ne!(cpu.pc, 0xa5a5);
}

// Verify jump z
#[test]
fn test_jp_z() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpAbs(BranchCondition::Z).as_byte(),
        0xa5,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_ne!(cpu.pc, 0xa5a5);
}

// Verify jump nc
#[test]
fn test_jp_nc() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpAbs(BranchCondition::NC).as_byte(),
        0xa5,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_ne!(cpu.pc, 0xa5a5);
}

// Verify jump c
#[test]
fn test_jp_c() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpAbs(BranchCondition::C).as_byte(),
        0xa5,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_ne!(cpu.pc, 0xa5a5);
}

// Verify jump
#[test]
fn test_jp() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpAbs(BranchCondition::NONE).as_byte(),
        0xa5,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);
}

// Verify jump HL
#[test]
fn test_jp_hl() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 1;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpAbsFromReg().as_byte(),
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg_16(Register16Bit::HL, 0xa5a5);
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);
}

// Verify jump relative nz
#[test]
fn test_jr_nz() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 4;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpRel(BranchCondition::NZ).as_byte(),
        0x01,
        0xff, // garbage
        0x00,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_ne!(cpu.pc, 2);
}

// Verify jump relative z
#[test]
fn test_jr_z() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 4;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpRel(BranchCondition::Z).as_byte(),
        0x01,
        0xff, // garbage
        0x00,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_ne!(cpu.pc, 2);
}

// Verify jump relative nc
#[test]
fn test_jr_nc() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 4;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpRel(BranchCondition::NC).as_byte(),
        0x01,
        0xff, // garbage
        0x00,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_ne!(cpu.pc, 2);
}

// Verify jump relative c
#[test]
fn test_jr_c() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 4;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpRel(BranchCondition::C).as_byte(),
        0x01,
        0xff, // garbage
        0x00,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_ne!(cpu.pc, 2);
}

// Verify jump relative
#[test]
fn test_jr() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 4;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::JumpRel(BranchCondition::NONE).as_byte(),
        0x01,
        0xff, // garbage
        0x00,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.step();
    assert_eq!(cpu.pc, 3);
}

// Verify push
#[test]
fn test_push() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Push(Register16Bit::BC).as_byte(),
        Instruction::Push(Register16Bit::DE).as_byte(),
        Instruction::Push(Register16Bit::HL).as_byte(),
        Instruction::Push(Register16Bit::AF).as_byte(),
        0x00,
        0x00,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_reg_16(Register16Bit::BC, 0xa55a);
    cpu.set_reg_16(Register16Bit::DE, 0xa55a);
    cpu.set_reg_16(Register16Bit::HL, 0xa55a);
    cpu.set_reg_16(Register16Bit::AF, 0xa55a);

    for i in 0..test_ram.len()-2 {
        cpu.set_sp(INSTRUCTIONS_LEN as u16);
        cpu.write_word((INSTRUCTIONS_LEN - 2) as u16, 0x0000);

        cpu.step();
        assert_eq!(cpu.read_word((INSTRUCTIONS_LEN - 2) as u16), 0xa55a);
    }
}

// Verify pop
#[test]
fn test_pop() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 6;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Pop(Register16Bit::BC).as_byte(),
        Instruction::Pop(Register16Bit::DE).as_byte(),
        Instruction::Pop(Register16Bit::HL).as_byte(),
        Instruction::Pop(Register16Bit::AF).as_byte(),
        0x5a,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);

    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_reg_16(Register16Bit::BC, 0x0000);
    cpu.step();
    assert_eq!(cpu.get_reg_16(Register16Bit::BC), 0xa55a);

    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_reg_16(Register16Bit::DE, 0x0000);
    cpu.step();
    assert_eq!(cpu.get_reg_16(Register16Bit::DE), 0xa55a);

    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_reg_16(Register16Bit::HL, 0x0000);
    cpu.step();
    assert_eq!(cpu.get_reg_16(Register16Bit::HL), 0xa55a);

    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_reg_16(Register16Bit::AF, 0x0000);
    cpu.step();
    assert_eq!(cpu.get_reg_16(Register16Bit::AF), 0xa55a);
}

// Verify ret nz
#[test]
fn test_ret_nz() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Ret(BranchCondition::NZ).as_byte(),
        0x5a,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_eq!(cpu.pc, 1);
}

// Verify ret z
#[test]
fn test_ret_z() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Ret(BranchCondition::Z).as_byte(),
        0x5a,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_eq!(cpu.pc, 1);
}


// Verify ret nc
#[test]
fn test_ret_nc() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Ret(BranchCondition::NC).as_byte(),
        0x5a,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.pc, 1);
}

// Verify ret c
#[test]
fn test_ret_c() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Ret(BranchCondition::C).as_byte(),
        0x5a,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_eq!(cpu.pc, 1);
}

// Verify ret
#[test]
fn test_ret() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 3;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Ret(BranchCondition::NONE).as_byte(),
        0x5a,
        0xa5,
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp((INSTRUCTIONS_LEN - 2) as u16);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);
}

// Verify rst
#[test]
fn test_rst() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 10;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Rst(RstVec::ZERO).as_byte(),
        Instruction::Rst(RstVec::ONE).as_byte(),
        Instruction::Rst(RstVec::TWO).as_byte(),
        Instruction::Rst(RstVec::THREE).as_byte(),
        Instruction::Rst(RstVec::FOUR).as_byte(),
        Instruction::Rst(RstVec::FIVE).as_byte(),
        Instruction::Rst(RstVec::SIX).as_byte(),
        Instruction::Rst(RstVec::SEVEN).as_byte(),
        0xff,
        0xff,
    ];

    let sp_top = INSTRUCTIONS_LEN as u16;

    cpu.load_test_ram(&test_ram);

    cpu.pc = 0;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::ZERO as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0000);

    cpu.pc = 1;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::ONE as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0001);

    cpu.pc = 2;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::TWO as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0002);

    cpu.pc = 3;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::THREE as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0003);

    cpu.pc = 4;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::FOUR as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0004);

    cpu.pc = 5;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::FIVE as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0005);

    cpu.pc = 6;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::SIX as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0006);

    cpu.pc = 7;
    cpu.set_sp(sp_top);
    cpu.step();
    assert_eq!(cpu.pc, RstVec::SEVEN as u16);
    assert_eq!(cpu.read_word(sp_top - 2), 0x0007);
}

// Verify call nz
#[test]
fn test_call_nz() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 5;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Call(BranchCondition::NZ).as_byte(),
        0x5a,
        0xa5,
        0xff, // garbage
        0xff, // garbage
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_eq!(cpu.pc, 3);
}

// Verify call z
#[test]
fn test_call_z() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 5;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Call(BranchCondition::Z).as_byte(),
        0x5a,
        0xa5,
        0xff, // garbage
        0xff, // garbage
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::Z, true);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::Z, false);
    cpu.step();
    assert_eq!(cpu.pc, 3);
}


// Verify call nc
#[test]
fn test_call_nc() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 5;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Call(BranchCondition::NC).as_byte(),
        0x5a,
        0xa5,
        0xff, // garbage
        0xff, // garbage
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.pc, 3);
}

// Verify call c
#[test]
fn test_call_c() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 5;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Call(BranchCondition::C).as_byte(),
        0x5a,
        0xa5,
        0xff, // garbage
        0xff, // garbage
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::C, true);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);

    cpu.pc = 0;
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.set_flag(Flag::C, false);
    cpu.step();
    assert_eq!(cpu.pc, 3);
}

// Verify call
#[test]
fn test_call() {
    let mut cpu = Cpu::new(Rc::new(RefCell::new(Mmu::new())));
    const INSTRUCTIONS_LEN: usize = 5;
    let test_ram: [u8; INSTRUCTIONS_LEN] = [
        Instruction::Call(BranchCondition::NONE).as_byte(),
        0x5a,
        0xa5,
        0xff, // garbage
        0xff, // garbage
    ];

    cpu.load_test_ram(&test_ram);
    cpu.set_sp(INSTRUCTIONS_LEN as u16);
    cpu.step();
    assert_eq!(cpu.pc, 0xa55a);
}
