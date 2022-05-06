mod cartridge;
mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod mmu;

use std::env;
use std::io;

use crate::gameboy::Gameboy;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct Cli {
    debug: bool,
    rom_path: String,
}

impl Cli {
    fn new() -> Cli {
        Cli {
            debug: false,
            rom_path: String::new(),
        }
    }
}

/* parses arguments and expects last argument to be
 * the rom path
 */
fn parse_args(cli: &mut Cli) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    for arg in env::args() {
        let arg = arg.as_str();
        match arg {
            #[cfg(debug_assertions)]
            "-d" => cli.debug = true,
            &_ => {},
        }
    }

    cli.rom_path = String::from(&args[args.len()-1]);
}

fn print_usage() {
    println!("usage: dookieboy rom_path");
    println!("  rom_path: absolute or relative path to ROM file");
}

fn main() {
    let mut cli = Cli::new();
    parse_args(&mut cli);

    let mut gameboy = Gameboy::init(WIDTH, HEIGHT).unwrap();
    match gameboy.run(cli.rom_path, cli.debug) {
        Ok(v) => {},
        Err(e) => {
            println!("dookieboy encountered big error: {}\n", e);
            print_usage();
            std::process::exit(1);
        },
    }
}
