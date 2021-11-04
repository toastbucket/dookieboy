mod cartridge;
mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod mmu;

use crate::gameboy::Gameboy;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let mut gameboy = Gameboy::init(WIDTH, HEIGHT)?;
    gameboy.run()?;

    Ok(())
}
