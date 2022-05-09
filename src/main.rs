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

fn is_gb_rom(filename: &str) -> bool {
    if filename.len() > 3 {
        if &filename[filename.len()-3..filename.len()] == ".gb" {
            return true;
        }
    }
    false
}

fn print_usage() {
    println!("usage: dookieboy [-d] rom_path");
    println!("  rom_path: absolute or relative path to ROM file");
    println!("  -d:       enable debug shell");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();
    if num_args < 2 {
        print_usage();
        std::process::exit(1);
    }

    // argument fields
    let mut debug: bool = false;
    let mut rom: String = String::new();

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if i == (num_args - 1) {
            if !is_gb_rom(arg.as_str()) {
                println!("valid rom path not provided");
                print_usage();
                std::process::exit(1);
            }

            rom = String::from(arg.as_str());
        }

        match arg.as_str() {
            #[cfg(debug_assertions)]
            "-d" => debug = true,
            &_ => {},
        }
    }

    let mut gameboy = Gameboy::init(WIDTH, HEIGHT).unwrap();
    match gameboy.run(rom, debug) {
        Ok(v) => {},
        Err(e) => {
            println!("dookieboy encountered big error: {}\n", e);
            print_usage();
            std::process::exit(1);
        },
    }
}
