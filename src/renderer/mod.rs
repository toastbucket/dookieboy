mod lcdc;
mod lcds;
mod tile;

use crate::memory::Memory;
use crate::renderer::lcdc::Lcdc;
use crate::renderer::lcds::Lcds;
use crate::renderer::tile::{ ColorId, Tile };

use sdl2::pixels;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
const BG_WIDTH: usize = 256;
const BG_HEIGHT: usize = 256;
const VRAM_SIZE: usize = 6144;
const TILE_MAP_SIZE: usize = 1024;
const TILE_WIDTH: usize = 8;

pub struct Renderer {
    cgb: bool,
    vram: Vec<u8>,
    tile_map_0: Vec<u8>,
    tile_map_1: Vec<u8>,
    lcdc: Lcdc,
    lcds: Lcds,
    scx: u8,
    scy: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,
}

impl Renderer {
    pub fn new(cgb: bool) -> Renderer {
        Renderer {
            cgb: cgb,
            vram: vec![0; VRAM_SIZE],
            tile_map_0: vec![0; TILE_MAP_SIZE],
            tile_map_1: vec![0; TILE_MAP_SIZE],
            lcdc: Lcdc::new(),
            lcds: Lcds::new(),
            scx: 0,
            scy: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            fb: vec![pixels::Color::BLACK; (SCREEN_WIDTH * SCREEN_HEIGHT)],
        }
    }

    pub fn step(&mut self) {

    }

    pub fn get_rect_color(&self, x: usize, y: usize) -> pixels::Color {
        if (x < 0) || (x > SCREEN_WIDTH) || (y < 0) || (y > SCREEN_HEIGHT) {
            panic!("pixel out of bounds [{}][{}]", x, y);
        }

        let x = (x + (self.scx as usize)) % BG_WIDTH;
        let y = (y + (self.scy as usize)) % BG_HEIGHT;

        let tile_idx = self.get_tile_idx(x, y);
        let tile = self.get_tile(tile_idx);

        let pix_idx = (x % TILE_WIDTH) + ((y % TILE_WIDTH) * TILE_WIDTH);
        match tile.get_pixel(pix_idx) {
            ColorId::ID_0 => pixels::Color::BLACK,
            ColorId::ID_1 => pixels::Color::GREEN,
            ColorId::ID_2 => pixels::Color::GREY,
            ColorId::ID_3 => pixels::Color::WHITE,
        }
    }

    fn get_tile_idx(&self, x: usize, y: usize) -> usize {
        let idx = (x / TILE_WIDTH) + (y / TILE_WIDTH);
        self.tile_map_0[idx] as usize
    }

    fn get_tile(&self, idx: usize) -> Tile {
        let i = idx * 16;
        Tile::from_slice(&self.vram[i..(i+16)])
    }
}

impl Memory for Renderer {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        if (addr >= 0x8000) && (addr <= 0x97ff) {
            let idx = (addr - 0x8000) as usize;
            self.vram[idx]
        } else if (addr >= 0x9800) && (addr <= 0x9bff) {
            let idx = (addr - 0x9800) as usize;
            self.tile_map_0[idx]
        } else if (addr >= 0x9c00) && (addr <= 0x9fff) {
            let idx = (addr - 0x9c00) as usize;
            self.tile_map_1[idx]
        } else if addr == 0xff00 {
            self.lcdc.build_reg()
        } else if addr == 0xff41 {
            self.lcds.build_reg()
        } else if addr == 0xff42 {
            self.scy
        } else if addr == 0xff43 {
            self.scx
        } else if addr == 0xff44 {
            self.ly
        } else if addr == 0xff45 {
            self.lyc
        } else if addr == 0xff4a {
            self.wy
        } else if addr == 0xff4b {
            self.wx
        } else {
            panic!("invalid access to VRAM");
        }
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        if (addr >= 0x8000) && (addr <= 0x97ff) {
            let idx = (addr - 0x8000) as usize;
            self.vram[idx] = val;
        } else if (addr >= 0x9800) && (addr <= 0x9bff) {
            let idx = (addr - 0x9800) as usize;
            self.tile_map_0[idx] = val;
        } else if (addr >= 0x9c00) && (addr <= 0x9fff) {
            let idx = (addr - 0x9c00) as usize;
            self.tile_map_1[idx] = val;
        } else if addr == 0xff00 {
            self.lcdc.set_reg(val);
        } else if addr == 0xff41 {
            self.lcds.set_reg(val);
        } else if addr == 0xff42 {
            self.scy = val;
        } else if addr == 0xff43 {
            self.scx = val;
        } else if addr == 0xff44 {
            self.ly = val;
        } else if addr == 0xff45 {
            self.lyc = val;
        } else if addr == 0xff4a {
            self.wy = val;
        } else if addr == 0xff4b {
            self.wx = val;
        } else {
            panic!("invalid access to VRAM");
        }
    }
}
