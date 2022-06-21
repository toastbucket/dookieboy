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

use self::Interrupt::*;
use crate::memory::Memory;

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    VBLANK = 0,
    STAT = 1,
    TIMER = 2,
    SERIAL = 3,
    JOYPAD = 4,
}

impl Interrupt {
    pub fn vector(&self) -> u16 {
        match self {
            VBLANK => 0x0040,
            STAT => 0x0048,
            TIMER => 0x0050,
            SERIAL => 0x0058,
            JOYPAD => 0x0060,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Interrupt> {
        [VBLANK, STAT, TIMER, SERIAL, JOYPAD].iter().copied()
    }
}

pub struct InterruptController {
    ime: bool,
    enable: u8,
    flag: u8,
}

impl Memory for InterruptController {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff0f => self.flag,
            0xffff => self.enable,
            _ => panic!("write to invalid address: {:#06x}", addr),
        }
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xff0f => self.flag = val,
            0xffff => self.enable = val,
            _ => panic!("write to invalid address: {:#06x}", addr),
        }
    }
}

impl InterruptController {
    pub fn new() -> InterruptController {
        InterruptController {
            ime: false,
            enable: 0,
            flag: 0,
        }
    }

    pub fn reset(&mut self) {
        self.enable = 0;
        self.flag = 0xe1;
        self.ime = true;
    }

    pub fn set_ime(&mut self, val: bool) {
        self.ime = val;
    }

    pub fn get_ime(&self) -> bool {
        self.ime
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        self.flag |= 1 << (interrupt as u8);
    }

    pub fn clear_request(&mut self, interrupt: Interrupt) {
        self.flag &= !(1 << (interrupt as u8));
    }

    pub fn should_trigger(&self, interrupt: Interrupt) -> bool {
        self.is_enabled(interrupt) && self.is_requested(interrupt)
    }

    pub fn is_enabled(&self, interrupt: Interrupt) -> bool {
        ((self.enable >> (interrupt as u8)) & 1) == 1
    }
    pub fn is_requested(&self, interrupt: Interrupt) -> bool {
        (self.flag >> (interrupt as u8)) & 1 == 1
    }
}
