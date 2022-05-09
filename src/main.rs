mod cartridge;
mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod mmu;
mod shell;

use std::env;
use std::io;

use crate::gameboy::Gameboy;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn print_usage() {
    println!("usage: dookieboy rom_path");
    println!("  rom_path: absolute or relative path to ROM file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let rom = String::from(&args[1]);

    let mut gameboy = Gameboy::init(WIDTH, HEIGHT).unwrap();
    match gameboy.run(rom) {
        Ok(v) => {},
        Err(e) => {
            println!("dookieboy encountered big error: {}\n", e);
            print_usage();
            std::process::exit(1);
        },
    }
}
