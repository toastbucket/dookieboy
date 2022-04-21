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

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
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

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
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
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
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
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
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
    cpu.set_reg(Register8Bit::A, 1);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
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
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
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

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
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

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
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
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, false);
        assert_eq!(cpu.cy, false);
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
        cpu.cy = true;
        cpu.step();

        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, true);
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
    cpu.set_reg(Register8Bit::A, 0);
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xff);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
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
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
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

        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.h, true);
        assert_eq!(cpu.cy, false);
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

    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, false);
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

    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, false);
    assert_eq!(cpu.get_reg(Register8Bit::A), 0x05);
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
    cpu.cy = true;
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 2);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
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
    cpu.cy = true;
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
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
    cpu.cy = true;
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0);
    assert_eq!(cpu.z, true);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
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
    cpu.cy = true;
    cpu.step();
    assert_eq!(cpu.get_reg(Register8Bit::A), 0xfe);
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, true);
    assert_eq!(cpu.h, true);
    assert_eq!(cpu.cy, true);
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
    assert_eq!(cpu.z, false);
    assert_eq!(cpu.n, false);
    assert_eq!(cpu.h, false);
    assert_eq!(cpu.cy, false);
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
    cpu.z = false;
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.z = true;
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
    cpu.z = true;
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.z = false;
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
    cpu.cy = false;
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.cy = true;
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
    cpu.cy = true;
    cpu.step();
    assert_eq!(cpu.pc, 0xa5a5);

    cpu.pc = 0;
    cpu.cy = false;
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
    cpu.z = false;
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.z = true;
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
    cpu.z = true;
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.z = false;
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
    cpu.cy = false;
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.cy = true;
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
    cpu.cy = true;
    cpu.step();
    assert_eq!(cpu.pc, 3);

    cpu.pc = 0;
    cpu.cy = false;
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
