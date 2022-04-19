use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;
use std::thread::sleep;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::joypad::Joypad;
use crate::memory::Memory;
use crate::renderer::Renderer;

pub struct Gameboy {
    cpu: Cpu,
    mmu: Rc<RefCell<Mmu>>,
    renderer: Renderer,

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
            renderer: Renderer::new(false),
            sdl_context: context,
            canvas: sdl_canvas,
        })
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

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.canvas.present();

        'running: loop {
            for y in 0..144 {
                for x in 0..160 {
                    let color = self.renderer.get_rect_color(x, y);
                    let rect = Rect::new((x as i32)*4, (y as i32)*4, 4, 4);
                    self.canvas.set_draw_color(color);
                    self.canvas.fill_rect(rect);
                }
            }
            self.canvas.present();

            self.handle_sdl2_events()?;
            sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    fn handle_sdl2_events(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => return Err("quit requested".to_string()),
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

