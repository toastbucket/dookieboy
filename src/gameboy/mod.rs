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
use std::io;
use std::rc::Rc;
use std::time::Duration;
use std::thread::sleep;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use crate::cpu::Cpu;
use crate::intc::Interrupt;
use crate::int_src::InterruptSource;
use crate::mmu::Mmu;
use crate::joypad::Button;
use crate::memory::Memory;
use crate::renderer::Renderer;

pub struct Gameboy {
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
    renderer: Renderer,

    sdl_context: Option<Sdl>,
    canvas: Option<WindowCanvas>,
    width: u32,
    height: u32,
}

impl Memory for Gameboy {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        self.mmu.borrow_mut().mem_read_byte(addr)
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        self.mmu.borrow_mut().mem_write_byte(addr, val);
    }
}

impl Gameboy {
    pub fn new(width: u32, height: u32) -> Gameboy {
        let mmu = Rc::new(RefCell::new(Mmu::new()));

        Gameboy {
            cpu: Cpu::new(Rc::clone(&mmu)),
            mmu: Rc::clone(&mmu),
            renderer: Renderer::new(false),
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
        /*
        Ok(Gameboy {
            cpu: Cpu::new(Rc::clone(&mmu)),
            mmu: Rc::clone(&mmu),
            renderer: Renderer::new(false),
            sdl_context: Some(context),
            canvas: Some(sdl_canvas),
        })
        */
    }


    pub fn run(&mut self) -> Result<(), String> {

        // load gameboy tile
        self.renderer.mem_write_byte(0x8000, 0x3c);
        self.renderer.mem_write_byte(0x8001, 0x7e);
        self.renderer.mem_write_byte(0x8002, 0x42);
        self.renderer.mem_write_byte(0x8003, 0x42);
        self.renderer.mem_write_byte(0x8004, 0x42);
        self.renderer.mem_write_byte(0x8005, 0x42);
        self.renderer.mem_write_byte(0x8006, 0x42);
        self.renderer.mem_write_byte(0x8007, 0x42);
        self.renderer.mem_write_byte(0x8008, 0x7e);
        self.renderer.mem_write_byte(0x8009, 0x5e);
        self.renderer.mem_write_byte(0x800a, 0x7e);
        self.renderer.mem_write_byte(0x800b, 0x0a);
        self.renderer.mem_write_byte(0x800c, 0x7c);
        self.renderer.mem_write_byte(0x800d, 0x56);
        self.renderer.mem_write_byte(0x800e, 0x38);
        self.renderer.mem_write_byte(0x800f, 0x7c);

        // load black tile
        for i in 0x8010..(0x8010+64) {
            self.renderer.mem_write_byte(i, 0x00)
        }

        // load tile_map
        for i in 0x9800..(0x9800+1024) {
            self.renderer.mem_write_byte(i, if i % 2 == 0 { 0 } else { 1 });
            println!("wrote {} into {:#04x}", self.renderer.mem_read_byte(i), i);
        }

        self.canvas.expect("NO CAN DO NO CANVAS!!!").set_draw_color(Color::WHITE);
        self.canvas.expect("NO CAN DO NO CANVAS!!!").clear();
        self.canvas.expect("NO CAN DO NO CANVAS!!!").present();

        'running: loop {
            for y in 0..144 {
                for x in 0..160 {
                    let color = self.renderer.get_rect_color(x, y);
                    let rect = Rect::new((x as i32)*4, (y as i32)*4, 4, 4);
                    self.canvas.expect("NO CAN DO NO CANVAS!!!").set_draw_color(color);
                    self.canvas.expect("NO CAN DO NO CANVAS!!!").fill_rect(rect);
                }
            }
            self.canvas.expect("NO CAN DO NO CANVAS!!!").present();

            self.handle_sdl2_events().unwrap();
            sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    pub fn load_rom(&mut self, path: String) -> Result<(), io::Error> {
        self.mmu.borrow_mut().cartridge.load_rom(path)?;

        Ok(())
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    /*
    pub fn run(&mut self) -> Result<(), io::Error> {

        // TODO: handle stop and halt
        //  If both the interrupt request flag and the corresponding interrupt enable flag are set,
        //  HALT mode is exited, even if the interrupt master enable flag is not set.
        //
        //  If the interrupt master enable flag is set, the contents of the program coounter are
        //  pushed to the stack and control jumps to the starting address of the interrupt.
        loop {
            self.handle_sdl2_events()?;
            self.check_for_interrupts();
            self.handle_interrupts();

            if !self.cpu.halted() {
                self.cpu.step();
            } else {
                // just sleep for 10ms now until we decide what
                // to do
                sleep(Duration::from_millis(10));
            }
        }
    }
    */

    fn check_for_interrupts(&mut self) {
        let mmu = &mut self.mmu.borrow_mut();

        if mmu.joypad.check_and_consume_int_req() {
            mmu.intc.request(Interrupt::JOYPAD);
        }
    }

    fn handle_interrupts(&mut self) {
        for interrupt in Interrupt::iterator() {
            let should_trigger: bool;
            let ime: bool;

            //TODO: find out how to borrow both
            {
                let intc = &mut self.mmu.borrow_mut().intc;
                should_trigger = intc.should_trigger(interrupt);
                ime = intc.get_ime();

                if should_trigger && ime {
                    intc.set_ime(false);
                    intc.clear_request(interrupt);
                }
            }

            if should_trigger && ime {
                self.cpu.trigger_interrupt(interrupt);

                // TODO: emulate halt bug
                // https://gbdev.io/pandocs/halt.html#halt-bug
                if self.cpu.halted() {
                    self.cpu.exit_halt();
                }

                // break from loop, we only handle one interrupt at
                // a time
                break;
            }
        }
    }

    fn handle_sdl2_events(&mut self) -> Result<(), io::Error> {
        if let Some(context) = &self.sdl_context {
            let mut pump = context.event_pump().unwrap();
            let joypad = &mut self.mmu.borrow_mut().joypad;
            for event in pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        repeat: false,
                        ..
                    } => return Err(io::Error::from_raw_os_error(0)),
                    Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                        joypad.update_button(Button::LEFT, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                        joypad.update_button(Button::RIGHT, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                        joypad.update_button(Button::UP, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                        joypad.update_button(Button::DOWN, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::J), repeat: false, .. } => {
                        joypad.update_button(Button::A, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::K), repeat: false, .. } => {
                        joypad.update_button(Button::B, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
                        joypad.update_button(Button::START, true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::RShift), repeat: false, .. } => {
                        joypad.update_button(Button::SELECT, true);
                    },
                    Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                        joypad.update_button(Button::LEFT, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                        joypad.update_button(Button::RIGHT, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                        joypad.update_button(Button::UP, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                        joypad.update_button(Button::DOWN, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::J), repeat: false, .. } => {
                        joypad.update_button(Button::A, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::K), repeat: false, .. } => {
                        joypad.update_button(Button::B, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Return), repeat: false, .. } => {
                        joypad.update_button(Button::START, false);
                    },
                    Event::KeyUp { keycode: Some(Keycode::RShift), repeat: false, .. } => {
                        joypad.update_button(Button::SELECT, false);
                    },
                    _ => continue,
                }
            }
        }
        Ok(())
    }
}

