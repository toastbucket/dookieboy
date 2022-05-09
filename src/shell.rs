use std::io::{
    stdin,
    stdout,
    Write,
};

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

    fn display_prompt() {
        print!("{}", PROMPT);
        stdout().flush();
    }
}
