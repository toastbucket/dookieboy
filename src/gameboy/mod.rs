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

    sdl_context: Option<Sdl>,
    canvas: Option<WindowCanvas>,
    width: u32,
    height: u32,
}

impl Gameboy {
    pub fn new(width: u32, height: u32) -> Gameboy {
        let mmu = Rc::new(RefCell::new(Mmu::new()));

        Gameboy {
            cpu: Cpu::new(Rc::clone(&mmu)),
            mmu: Rc::clone(&mmu),
            sdl_context: None,
            canvas: None,
            width: width,
            height: height,
        }
    }

    pub fn cpu(&mut self) -> &mut Cpu {
        &mut self.cpu
    }

    pub fn init_sdl(&mut self) -> Result<(), String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let window = video_subsystem
            .window("DookieBoy", self.width, self.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let sdl_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        self.sdl_context = Some(context);
        self.canvas = Some(sdl_canvas);

        Ok(())
    }

    pub fn load_rom(&mut self, path: String) -> Result<(), io::Error> {
        self.mmu.borrow_mut().cartridge.load_rom(path)?;

        Ok(())
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self) -> Result<(), io::Error> {

        // TODO: handle stop and halt
        //  If both the interrupt request flag and the corresponding interrupt enable flag are set,
        //  HALT mode is exited, even if the interrupt master enable flag is not set.
        //
        //  If the interrupt master enable flag is set, the contents of the program coounter are
        //  pushed to the stack and control jumps to the starting address of the interrupt.
        'running: loop {
            self.handle_sdl2_events()?;
            self.cpu.step();
        }

        Ok(())
    }

    fn handle_sdl2_events(&mut self) -> Result<(), io::Error> {
        if let Some(context) = &self.sdl_context {
            let mut pump = context.event_pump().unwrap();
            for event in pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        repeat: false,
                        ..
                    } => return Err(io::Error::from_raw_os_error(0)),
                    Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                        println!("got a");
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
        }

        Ok(())
    }
}

