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

pub struct Lcdc {
    // LCDC.0:
    //   Non-CGB: BG and Win becom blank when cleared. LCDC.5
    //            is ignored in this case. Sprites can still
    //            be displayed.
    //   CGB: BG and Win lose priority when cleared. Sprites
    //        will always be displayed on top of BG and Win.
    bg_win_prio: bool,
    // LCDC.1: Sprites displayed when set.
    obj_en: bool,
    // LCDC.2: Sprite size. 0->8x8, 1->8x16 (vertical)
    obj_size: bool,
    // LCDC.3: BG tile map selector. 0->0x9800, 1->0x9c00
    bg_tile_map: bool,
    // LCDC.4: BG and Win addressing mode selector.
    //         0->0x8800 method, 1->0x8000 method
    bg_win_tile_data: bool,
    // LCDC.5: Window enable. LCDC.0 takes precedence.
    win_enable: bool,
    // LCDC.6: Win tile map selector. 0->0x9800, 1->0x9c00
    win_tile_map: bool,
    // LCDC.7: LCD and PPU enable. When disabled, screen is
    //         blanked and CPU has access to VRAM and OAM.
    //         No game should ever clear this outside of VBlank.
    lcd_ppu_en: bool,
}

impl Lcdc {
    pub fn new() -> Lcdc {
        Lcdc {
            bg_win_prio: false,
            obj_en: false,
            obj_size: false,
            bg_tile_map: false,
            bg_win_tile_data: false,
            win_enable: false,
            win_tile_map: false,
            lcd_ppu_en: false,
        }
    }

    pub fn reset(&mut self) {
        self.bg_win_prio = true;
        self.obj_en = false;
        self.obj_size = false;
        self.bg_tile_map = false;
        self.bg_win_tile_data = true;
        self.win_enable = false;
        self.win_tile_map = false;
        self.lcd_ppu_en = true;
    }

    pub fn build_reg(&self) -> u8 {
        (self.lcd_ppu_en as u8) << 0
        | (self.win_tile_map as u8) << 1
        | (self.win_enable as u8) << 2
        | (self.bg_win_tile_data as u8) << 3
        | (self.bg_tile_map as u8) << 4
        | (self.obj_size as u8) << 5
        | (self.obj_en as u8) << 6
        | (self.bg_win_prio as u8) << 7
    }

    pub fn set_reg(&mut self, val: u8) {
        self.lcd_ppu_en = ((val << 0) & 1) == 1;
        self.win_tile_map = ((val << 1) & 1) == 1;
        self.win_enable = ((val << 2) & 1) == 1;
        self.bg_win_tile_data = ((val << 3) & 1) == 1;
        self.bg_tile_map = ((val << 4) & 1) == 1;
        self.obj_size = ((val << 5) & 1) == 1;
        self.obj_en = ((val << 6) & 1) == 1;
        self.bg_win_prio = ((val << 7) & 1) == 1;
    }
}
