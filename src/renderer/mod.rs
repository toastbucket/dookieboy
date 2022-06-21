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
    vram: [u8; VRAM_SIZE],
    tile_map_0: [u8; TILE_MAP_SIZE],
    tile_map_1: [u8; TILE_MAP_SIZE],
    lcdc: Lcdc,
    lcds: Lcds,
    scx: u8,
    scy: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,
    fb: [pixels::Color; (SCREEN_WIDTH * SCREEN_HEIGHT)],
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            vram: [0; VRAM_SIZE],
            tile_map_0: [0; TILE_MAP_SIZE],
            tile_map_1: [0; TILE_MAP_SIZE],
            lcdc: Lcdc::new(),
            lcds: Lcds::new(),
            scx: 0,
            scy: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            fb: [pixels::Color::BLACK; (SCREEN_WIDTH * SCREEN_HEIGHT)],
        }
    }

    pub fn reset(&mut self) {
        self.scy = 0;
        self.scx = 0;
        self.ly = 0;
        self.lyc = 0;
        self.wy = 0;
        self.wx = 0;

        self.lcdc.reset();
        self.lcds.reset();
    }

    fn get_rect_color(&self, x: usize, y: usize) -> pixels::Color {
        if x > SCREEN_WIDTH || y > SCREEN_HEIGHT {
            panic!("pixel out of bounds [{}][{}]", x, y);
        }

        let x = (x + (self.scx as usize)) % BG_WIDTH;
        let y = (y + (self.scy as usize)) % BG_HEIGHT;

        let tile_idx = self.get_tile_idx(x, y);
        let tile = self.get_tile(tile_idx);

        let pix_idx = (x % TILE_WIDTH) + ((y % TILE_WIDTH) * TILE_WIDTH);
        match tile.get_pixel(pix_idx) {
            ColorId::White => pixels::Color::WHITE,
            ColorId::LightGray => pixels::Color::GREEN,
            ColorId::DarkGray => pixels::Color::GREY,
            ColorId::Black => pixels::Color::BLACK,
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
        match addr {
            0x8000..=0x97ff => {
                let idx = (addr - 0x8000) as usize;
                self.vram[idx]
            },
            0x9800..=0x9bff => {
                let idx = (addr - 0x9800) as usize;
                self.tile_map_0[idx]
            },
            0x9c00..=0x9fff => {
                let idx = (addr - 0x9c00) as usize;
                self.tile_map_1[idx]
            },
            0xff40 => self.lcdc.build_reg(),
            0xff41 => self.lcds.build_reg(),
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => panic!("invalid access to VRAM: {:#06x}", addr),
        }
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x97ff => {
                let idx = (addr - 0x8000) as usize;
                self.vram[idx] = val;
            },
            0x9800..=0x9bff => {
                let idx = (addr - 0x9800) as usize;
                self.tile_map_0[idx] = val;
            },
            0x9c00..=0x9fff => {
                let idx = (addr - 0x9c00) as usize;
                self.tile_map_1[idx] = val;
            },
            0xff40 => self.lcdc.set_reg(val),
            0xff41 => self.lcds.set_reg(val),
            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff44 => self.ly = val,
            0xff45 => self.lyc = val,
            0xff4a => self.wy = val,
            0xff4b => self.wx = val,
            _ => panic!("invalid access to VRAM: {:#06x}", addr),
        }
    }
}
