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

use std::io::{
    stdin,
    stdout,
    Write,
};

use crate::gameboy::Gameboy;
use crate::memory::Memory;

const PROMPT: &str = "dookie>";

#[derive(Debug)]
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
}

pub struct Shell {
    dump_mode: bool,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            dump_mode: false,
        }
    }

    pub fn get_cmd() -> Option<Cmd> {
        Shell::display_prompt();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut raw = input.trim().split_whitespace();
        match raw.next() {
            Some(cmd) => {
                Some(
                    Cmd {
                        cmd: cmd.to_string(),
                        args: raw.map(|s| s.to_string()).collect(),
                    })
            },
            None => {
                None
            },
        }
    }

    pub fn run_cmd(&mut self, gb: &mut Gameboy, cmd: &Option<Cmd>) -> bool {
        match cmd {
            Some(cmd) => {
                match cmd.cmd.as_ref() {
                    "step" => {
                        let mut num_steps = 1;
                        if cmd.args.len() > 0 {
                            num_steps = cmd.args[0].parse::<usize>().unwrap_or(1);
                        }

                        for _ in 0..num_steps {
                            gb.cpu().step();

                            if self.dump_mode {
                                Shell::dump_the_dookie(gb);
                            }
                        }
                    },
                    "dump" => {
                       if cmd.args.len() > 0 {
                           match cmd.args[0].as_ref() {
                               "on" => {
                                   self.dump_mode = true;
                                   println!("enabling dump mode");
                               },
                               "off" => {
                                   self.dump_mode = false;
                                   println!("disabling dump mode");
                               },
                               &_ => {},
                           }
                       } else {
                           Shell::dump_the_dookie(gb);
                       }
                    },
                    "set" => {
                        const INVAL: usize = 0x10000;

                        if cmd.args.len() >= 2 {
                            let addr = usize::from_str_radix(cmd.args[0].as_ref(), 16)
                                .unwrap_or(INVAL);

                            let val = usize::from_str_radix(cmd.args[1].as_ref(), 16)
                                .unwrap_or(INVAL);

                            if addr < INVAL && val < INVAL {
                                gb.mem_write_byte(addr as u16, val as u8);
                            }
                        }
                    },
                    "get" => {
                        const INVAL: usize = 0x10000;

                        if cmd.args.len() >= 1 {
                            let addr = usize::from_str_radix(cmd.args[0].as_ref(), 16)
                                .unwrap_or(INVAL);

                            if addr < INVAL {
                                let val = gb.mem_read_byte(addr as u16);
                                println!("[{:#06x}] = {:#04x}", addr, val);
                            }
                        }
                    },
                    "q" | "e" | "quit" | "exit" => {
                        std::process::exit(0);
                    },
                    _ => {
                        println!("invalid command: {}", cmd.cmd);
                    },
                }

                true
            },
            None => false
        }
    }

    fn display_prompt() {
        print!("{}", PROMPT);
        let _ = stdout().flush();
    }

    fn dump_the_dookie(gb: &mut Gameboy) {
        println!("CPU:\n{}", gb.cpu());
    }
}
