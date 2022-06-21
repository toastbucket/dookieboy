// ░░░░░░░░░░░█▀▀░░█░░░░░░
// ░░░░░░▄▀▀▀▀░░░░░█▄▄░░░░
// ░░░░░░█░█░░░░░░░░░░▐░░░
// ░░░░░░▐▐░░░░░░░░░▄░▐░░░
// ░░░░░░█░░░░░░░░▄▀▀░▐░░░
// ░░░░▄▀░░░░░░░░▐░▄▄▀░░░░
// ░░▄▀░░░▐░░░░░█▄▀░▐░░░░░
// ░░█░░░▐░░░░░░░░▄░█░░░░░
// ░░░█▄░░▀▄░░░░▄▀▐░█░░░░░
// ░░░█▐▀▀▀░▀▀▀▀░░▐░█░░░░░
// ░░▐█▐▄░░▀░░░░░░▐░█▄▄░░
// ░░░▀▀░▄TSM▄░░░▐▄▄▄▀░░░

use crate::cartridge::Cartridge;
use crate::intc::InterruptController;
use crate::joypad::Joypad;
use crate::memory::Memory;
use crate::serial::Serial;

const WRAM_BASE: usize = 0xc000;
const WRAM_BANK_BASE: usize = 0xd000;
const WRAM_SIZE: usize = 4096;
const HRAM_BASE: usize = 0xff80;
const HRAM_SIZE: usize = 127;
const NUM_WRAM_BANKS: usize = 8;

/*
 * Memory Map
 *
 * 0000..3FFF: 16KiB ROM bank 00,              From cartridge, usually a fixed bank
 * 4000..7FFF: 16KiB ROM Bank 01~NN,           From cartridge, switchable bank via mapper (if any)
 * 8000..9FFF: 8KiB Video RAM (VRAM),          Only bank 0 in Non-CGB mode. Switchable bank 0/1 in CGB mode
 * A000..BFFF: 8KiB External RAM,              From cartridge, switchable bank if any
 * C000..CFFF: 4KiB Work RAM (WRAM) bank 0
 * D000..DFFF: 4KiB Work RAM (WRAM) bank 1~N,  Only bank 1 in Non-CGB mode, Switchable bank 1~7 in CGB mode
 * E000..FDFF: Mirror of C000~DDFF (ECHO RAM), Use prohibited according to Nintendo
 * FE00..FE9F: Sprite attribute table (OAM)
 * FEA0..FEFF: Not Usable,                     Use prohibited according to Nintendo
 * FF00..FF7F: I/O Registers
 * FF80..FFFE: High RAM (HRAM)
 * FFFE..FFFF: Interrupts Enable Register (IE)
 */

pub struct Mmu {
    pub cartridge: Cartridge,
    pub intc: InterruptController,
    pub joypad: Joypad,
    serial: Serial,
    wram: [[u8; WRAM_SIZE]; NUM_WRAM_BANKS],
    svbk: usize,
    hram: [u8; HRAM_SIZE],
}

impl Memory for Mmu {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.cartridge.mem_read_byte(addr),
            0xc000..=0xcfff => {
                let idx = (addr as usize) - WRAM_BASE;
                self.wram[0][idx]
            },
            0xd000..=0xdfff => {
                let idx = (addr as usize) - WRAM_BANK_BASE;
                let bank = if self.svbk == 0 { 1 } else { self.svbk };
                self.wram[bank][idx]
            },
            0xff00 => self.joypad.mem_read_byte(addr),
            0xff01..=0xff02 => self.serial.mem_read_byte(addr),
            0xff70 => self.svbk as u8,
            0xff80..=0xfffe => {
                let idx = (addr as usize) - HRAM_BASE;
                self.hram[idx]
            },
            0xff0f | 0xffff => self.intc.mem_read_byte(addr),
            _ => panic!("read from unmapped address {:#06x}", addr),
        }
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.mem_write_byte(addr, val),
            0xc000..=0xcfff => {
                let idx = (addr as usize) - WRAM_BASE;
                self.wram[0][idx] = val;
            },
            0xd000..=0xdfff => {
                let idx = (addr as usize) - WRAM_BANK_BASE;
                let bank = if self.svbk == 0 { 1 } else { self.svbk };
                self.wram[bank][idx] = val;
            },
            0xff00 => self.joypad.mem_write_byte(addr, val),
            0xff01..=0xff02 => self.serial.mem_write_byte(addr, val),
            0xff70 => self.svbk = (val & 0x7) as usize,
            0xff80..=0xfffe => {
                let idx = (addr as usize) - HRAM_BASE;
                self.hram[idx] = val;
            },
            0xff0f | 0xffff => self.intc.mem_write_byte(addr, val),
            _ => panic!("write to unmapped address {:#06x}", addr),
        }
    }
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            cartridge: Cartridge::new(),
            intc: InterruptController::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),
            wram: [[0; WRAM_SIZE]; NUM_WRAM_BANKS],
            svbk: 0,
            hram: [0; HRAM_SIZE],
        }
    }
}
