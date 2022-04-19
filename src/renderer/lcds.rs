pub struct Lcds {
    // LCDS.1:0: Mode flag
    //   0: HBlank
    //   1: VBlank
    //   2: Searching ORAM
    //   3: Transfer data
    //
    //   Mode 2  2_____2_____2_____2_____2_____2___________________2____
    //   Mode 3  _33____33____33____33____33____33__________________3___
    //   Mode 0  ___000___000___000___000___000___000________________000
    //   Mode 1  ____________________________________11111111111111_____
    mode: u8,
    // LCDS.2: LYC=LY flag
    lyc_ly: bool,
    // LCDS.3: Mode 0 HBlank STAT
    //   When set, entering Mode 0 triggers STAT interrupt
    hblank_stat: bool,
    // LCDS.4: Mode 1 VBlank STAT
    //   When set, entering Mode 1 triggers STAT interupt
    vblank_stat: bool,
    // LCDS.5: Mode 2 OAM STAT
    //   When set, entering Mode 2 triggers STAT interrupt
    oam_stat: bool,
    //LCDS.6: LYC=LY STAT
    //  When set, STAT is triggered once LYC=LY
    lyc_ly_stat: bool
}

impl Lcds {
    pub fn new() -> Lcds {
        Lcds {
            mode: 0,
            lyc_ly: false,
            hblank_stat: false,
            vblank_stat: false,
            oam_stat: false,
            lyc_ly_stat: false,
        }
    }

    pub fn build_reg(&self) -> u8 {
        (self.mode & 0x3)
        | (self.lyc_ly as u8) << 2
        | (self.hblank_stat as u8) << 3
        | (self.vblank_stat as u8) << 4
        | (self.oam_stat as u8) << 5
        | (self.lyc_ly_stat as u8) << 6
    }

    pub fn set_reg(&mut self, val: u8) {
        self.mode = val & 0x3;
        self.lyc_ly = if ((val << 2) & 1) == 1 { true } else { false };
        self.hblank_stat = if ((val << 3) & 1) == 1 { true } else { false };
        self.vblank_stat = if ((val << 4) & 1) == 1 { true } else { false };
        self.oam_stat = if ((val << 5) & 1) == 1 { true } else { false };
        self.lyc_ly_stat = if ((val << 6) & 1) == 1 { true } else { false };
    }
}
