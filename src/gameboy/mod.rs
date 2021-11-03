use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;
use std::thread::sleep;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::joypad::Joypad;
use crate::memory::Memory;

pub struct Gameboy {
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        let mmu = Rc::new(RefCell::new(Mmu::new()));
        Gameboy {
            cpu: Cpu::new(Rc::clone(&mmu)),
            mmu: Rc::clone(&mmu),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("DookieBoy", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let mut events = sdl_context.event_pump()?;

        'running: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        repeat: false,
                        ..
                    } => break 'running,
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

            sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}

