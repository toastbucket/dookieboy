const PIXELS_PER_TILE: usize = 64;
const BPP: usize = 2;

#[derive (Copy, Clone)]
pub enum ColorId {
    ID_0 = 0,
    ID_1 = 1,
    ID_2 = 2,
    ID_3 = 3,
}

impl ColorId {
    fn from_byte(byte: u8) -> ColorId {
        match byte {
            0 => ColorId::ID_0,
            1 => ColorId::ID_1,
            2 => ColorId::ID_2,
            3 => ColorId::ID_3,
            _ => panic!("invalid color ID"),
        }
    }
}

pub struct Tile {
    pixels: Vec<ColorId>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            pixels: vec![ColorId::ID_0; PIXELS_PER_TILE],
        }
    }

    pub fn from_slice(slice: &[u8]) -> Tile {
        if (slice.len() != (PIXELS_PER_TILE * BPP) / 8) {
            panic!("invalid slice length {}", slice.len());
        }

        let mut pixels = vec![ColorId::ID_0; PIXELS_PER_TILE];

        // TODO: use a map maybe?
        for (bline, pline) in slice.chunks(2).zip(pixels.chunks_mut(8)) {
            pline[0] = ColorId::from_byte((bline[0] >> 7) & 1 | (((bline[1] >> 7) & 1) << 1));
            pline[1] = ColorId::from_byte((bline[0] >> 6) & 1 | (((bline[1] >> 6) & 1) << 1));
            pline[2] = ColorId::from_byte((bline[0] >> 5) & 1 | (((bline[1] >> 5) & 1) << 1));
            pline[3] = ColorId::from_byte((bline[0] >> 4) & 1 | (((bline[1] >> 4) & 1) << 1));
            pline[4] = ColorId::from_byte((bline[0] >> 3) & 1 | (((bline[1] >> 3) & 1) << 1));
            pline[5] = ColorId::from_byte((bline[0] >> 2) & 1 | (((bline[1] >> 2) & 1) << 1));
            pline[6] = ColorId::from_byte((bline[0] >> 1) & 1 | (((bline[1] >> 1) & 1) << 1));
            pline[7] = ColorId::from_byte((bline[0] >> 0) & 1 | (((bline[1] >> 0) & 1) << 1));
        }

        Tile { pixels: pixels }
    }

    pub fn get_pixel(&self, idx: usize) -> ColorId {
        if idx >= 0 && idx < PIXELS_PER_TILE {
            self.pixels[idx]
        } else {
            panic!("pixel index out of bounds: {}", idx);
        }
    }

    pub fn set_pixel(&mut self, idx: usize, color: ColorId) {
        if idx >= 0 && idx < PIXELS_PER_TILE {
            self.pixels[idx] = color;
        } else {
            panic!("pixel index out of bounds: {}", idx);
        }
    }
}
