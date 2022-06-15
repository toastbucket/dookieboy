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

use crate::int_src::InterruptSource;
use crate::memory::Memory;

#[derive(Debug, Copy, Clone)]
pub enum Button {
    RIGHT = 0,
    LEFT,
    UP,
    DOWN,
    A,
    B,
    SELECT,
    START,
}

pub struct Joypad {
   action_select: bool,
   direction_select: bool,
   buttons: [bool; 8],
   int_req: bool,
}

impl InterruptSource for Joypad {
    fn check_int_req(&self) -> bool {
        self.int_req
    }

    fn consume_int_req(&mut self) {
        self.int_req = false;
    }
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
            buttons: [false; 8],
            int_req: false,
        }
    }

    pub fn update_button(&mut self, button: Button, state: bool) {
        let idx = button as usize;
        let cur_state = self.buttons[idx];

        // TODO: emulate switch bounce?
        match button {
            Button::A      |
            Button::B      |
            Button::SELECT |
            Button::START => {
                if self.action_select && !cur_state && state {
                    self.int_req = true;
                }
            },
            Button::RIGHT |
            Button::LEFT  |
            Button::UP    |
            Button::DOWN => {
                if self.direction_select && !cur_state && state {
                    self.int_req = true;
                }
            },
            _ => {},
        }

        self.buttons[idx] = state;
    }

    pub fn update_dir_select(&mut self, state: bool) {
        self.direction_select = state;
    }

    pub fn update_act_select(&mut self, state: bool) {
        self.action_select = state;
    }

    // All bits are represented inverted in the P1 register
    // i.e. 0b00011110 indicates that action buttons are
    // selected and only the A button is pressed
    fn build_reg(&self) -> u8 {
        // TODO: fix this logic
        let start: usize = if self.action_select { 4 } else { 0 };

        (!self.buttons[start] as u8)   << 0 |
        (!self.buttons[start+1]as u8)  << 1 |
        (!self.buttons[start+2]as u8)  << 2 |
        (!self.buttons[start+3]as u8)  << 3 |
        (!self.direction_select as u8) << 4 |
        (!self.action_select as u8)    << 5
    }
}

#[cfg(test)]
mod test;

