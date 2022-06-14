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

use crate::memory::Memory;

pub struct Joypad {
   pub action_select: bool,
   pub direction_select: bool,
   pub up: bool,
   pub down: bool,
   pub left: bool,
   pub right: bool,
   pub a: bool,
   pub b: bool,
   pub start: bool,
   pub select: bool,
}

impl Memory for Joypad {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        if addr == 0xff00 {
            self.build_reg()
        } else {
            0xff
        }
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        if val & (1 << 4) != 0 {
            self.direction_select = true;
        }

        if val & (1<< 5) != 0 {
            self.action_select = true;
        }
    }
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            action_select: false,
            direction_select: false,
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    // All bits are represented inverted in the P1 register
    // i.e. 0b00011110 indicates that action buttons are
    // selected and only the A button is pressed
    fn build_reg(&self) -> u8 {
        let reg;
        if self.action_select {
            reg = (!self.a as u8)
                | (!self.b as u8) << 1
                | (!self.select as u8) << 2
                | (!self.start as u8) << 3;
        } else {
            reg = (!self.right as u8)
                | (!self.left as u8) << 1
                | (!self.up as u8) << 2
                | (!self.down as u8) << 3;
        }
         reg | ((!self.direction_select as u8) << 4) | ((!self.action_select as u8) << 5)
    }
}

#[cfg(test)]
mod test;

