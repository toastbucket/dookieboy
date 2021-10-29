mod cartridge;
mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod mmu;

use crate::gameboy::Gameboy;

fn main() {
    let gameboy = Gameboy::new();
}
