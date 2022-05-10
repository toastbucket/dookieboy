use std::io::{
    stdin,
    stdout,
    Write,
};

use crate::gameboy::Gameboy;

const PROMPT: &str = "dookie>";

#[derive(Debug)]
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
}

pub struct Shell;

impl Shell {
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

    pub fn run_cmd(gb: &mut Gameboy, cmd: &Option<Cmd>) -> bool {
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
                        }
                    },
                    "dump" => {
                        println!("CPU:\n{}", gb.cpu());
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
        stdout().flush();
    }
}
