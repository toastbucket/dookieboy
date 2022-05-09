use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::rc::Rc;
use std::time::Duration;
use std::thread::sleep;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use crate::shell::{Shell, Cmd};
use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::joypad::Joypad;
use crate::memory::Memory;

pub struct Gameboy {
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,

    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl Gameboy {
    pub fn init(width: u32, height: u32) -> Result<Gameboy, String> {
        let mmu = Rc::new(RefCell::new(Mmu::new()));

        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let window = video_subsystem
            .window("DookieBoy", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let sdl_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Gameboy {
            cpu: Cpu::new(Rc::clone(&mmu)),
            mmu: Rc::clone(&mmu),
            sdl_context: context,
            canvas: sdl_canvas,
        })
    }

    fn run_cmd(&mut self, cmd: &Option<Cmd>) -> bool {
        match cmd {
            Some(cmd) => {
                match cmd.cmd.as_ref() {
                    "step" => {
                        let mut num_steps = 1;
                        if cmd.args.len() > 0 {
                            num_steps = cmd.args[0].parse::<usize>().unwrap_or(1);
                        }

                        for _ in 0..num_steps {
                            self.cpu.step();
                        }
                    },
                    "dump" => {
                        println!("CPU:\n{}", self.cpu);
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

    pub fn run(&mut self, rom: String, debug: bool) -> Result<(), io::Error> {

        self.mmu.borrow_mut().cartridge.load_rom(rom)?;
        self.cpu.reset();

        if debug {
            let mut last_cmd: Option<Cmd> = None;
            let mut cmd: Option<Cmd> = None;

            'debug: loop {
                cmd = Shell::get_cmd();
                let ret = self.run_cmd(&cmd);
                if !ret {
                    self.run_cmd(&last_cmd);
                } else {
                    last_cmd = cmd.take();
                }
            }
        } else {
            'running: loop {
                self.handle_sdl2_events()?;
                sleep(Duration::from_millis(100));
            }
        }

        Ok(())
    }

    fn handle_sdl2_events(&mut self) -> Result<(), io::Error> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => return Err(io::Error::from_raw_os_error(0)),
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.left = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.right = true;
                },
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.up = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.down = true;
                },
                Event::KeyDown { keycode: Some(Keycode::J), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.a = true;
                },
                Event::KeyDown { keycode: Some(Keycode::K), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.b = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.start = true;
                },
                Event::KeyDown { keycode: Some(Keycode::RShift), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.select = true;
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.left = false;
                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.right = false;
                },
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.up = false;
                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.down = false;
                },
                Event::KeyUp { keycode: Some(Keycode::J), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.a = false;
                },
                Event::KeyUp { keycode: Some(Keycode::K), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.b = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Return), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.start = false;
                },
                Event::KeyUp { keycode: Some(Keycode::RShift), repeat: false, .. } => {
                    self.mmu.borrow_mut().joypad.select = false;
                },
                _ => continue,
            }
        }

        Ok(())
    }
}

