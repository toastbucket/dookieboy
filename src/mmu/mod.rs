// src/mmu.rs

use crate::cartridge::Cartridge;
use crate::joypad::Joypad;
use crate::memory::Memory;

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
    pub joypad: Joypad,

}

impl Memory for Mmu {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.cartridge.mem_read_byte(addr),
            0xff00 => self.joypad.mem_read_byte(addr),
            _ => panic!("not valid address"),
        }
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.mem_write_byte(addr, val),
            0xff00 => self.joypad.mem_write_byte(addr, val),
            _ => panic!("not valid address"),
        }
    }
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            cartridge: Cartridge::new(),
            joypad: Joypad::new(),
        }
    }
}
