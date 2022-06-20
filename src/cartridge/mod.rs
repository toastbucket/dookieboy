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

use std::fs;
use std::io;

use crate::memory::Memory;

const HEADER_START: usize = 0x100;
const HEADER_SIZE: usize = 80;

#[derive(Debug)]
enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0b,
    Mmm01Ram = 0x0c,
    Mmm01RamBattery =0x0d,
    Mbc3TimerBattery = 0x0f,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1a,
    Mbc5RamBattery = 0x1b,
    Mbc5Rumble = 0x1c,
    Mbc5RumbleRam = 0x1d,
    Mbc5RumbleRamBattery = 0x1e,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xfc,
    BandaiTama5 = 0xfd,
    HuC3 = 0xfe,
    HuC1RamBattery = 0xff,
}

#[derive(Debug)]
enum CartridgeMode {
    PgbMode,
    CgbSupported,
    CgbOnly,
}

struct Header {
    title: String,
    cart_type: CartridgeType,
    cart_mode: CartridgeMode,
    rom_size: usize,
    ram_size: usize,
    header_checksum: u8,
    cart_checksum: u16,
}

impl Header {
    fn new() -> Header {
        Header {
            title: String::new(),
            cart_type: CartridgeType::RomOnly,
            cart_mode: CartridgeMode::PgbMode,
            rom_size: 0,
            ram_size: 0,
            header_checksum: 0,
            cart_checksum: 0,
        }
    }

    fn dump(&self) {
        println!("Title: {}", self.title);
        println!("Type: {:?}", self.cart_type);
        println!("Mode: {:?}", self.cart_mode);
        println!("ROM: {}KiB", self.rom_size / 1024);
        println!("RAM: {}KiB", self.ram_size / 1024);
    }
}

pub struct Cartridge {
    header: Header,
    rom: Vec<u8>,
    loaded: bool,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            header: Header::new(),
            rom: Vec::new(),
            loaded: false,
        }
    }

    pub fn load_rom(&mut self, path: String) -> Result<(), io::Error> {
        self.rom = fs::read(path)?;
        if self.rom.len() < (HEADER_START + HEADER_SIZE) {
            panic!("missing header");
        }

        self.parse_header();
        self.loaded = true;

        #[cfg(debug_assertions)]
        self.header.dump();

        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    fn parse_header(&mut self) {
        self.header.header_checksum = self.mem_read_byte(0x14d);
        assert_eq!(self.header.header_checksum, self.calc_header_checksum());

        self.header.cart_checksum = self.mem_read_word_be(0x14e);
        assert_eq!(self.header.cart_checksum, self.calc_cart_checksum());

        assert!(self.compare_nintendo_logo());

        self.header.title = String::from_utf8(self.rom[0x134..0x143].to_vec()).unwrap();

        // default to PGB mode, so don't bother reading bit 7
        if self.mem_is_set(0x143, 6) {
            self.header.cart_mode = CartridgeMode::CgbOnly;
        } else if !(self.mem_is_set(0x143, 3) || self.mem_is_set(0x143, 2)) {
            self.header.cart_mode = CartridgeMode::CgbSupported;
        } else {
            self.header.cart_mode = CartridgeMode::PgbMode;
        }

       self.header.cart_type = match self.mem_read_byte(0x147) {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::Mbc1,
            0x02 => CartridgeType::Mbc1Ram,
            0x03 => CartridgeType::Mbc1RamBattery,
            0x05 => CartridgeType::Mbc2,
            0x06 => CartridgeType::Mbc2Battery,
            0x08 => CartridgeType::RomRam,
            0x09 => CartridgeType::RomRamBattery,
            0x0b => CartridgeType::Mmm01,
            0x0c => CartridgeType::Mmm01Ram,
            0x0d => CartridgeType::Mmm01RamBattery,
            0x0f => CartridgeType::Mbc3TimerBattery,
            0x10 => CartridgeType::Mbc3TimerRamBattery,
            0x11 => CartridgeType::Mbc3,
            0x12 => CartridgeType::Mbc3Ram,
            0x13 => CartridgeType::Mbc3RamBattery,
            0x19 => CartridgeType::Mbc5,
            0x1a => CartridgeType::Mbc5Ram,
            0x1b => CartridgeType::Mbc5RamBattery,
            0x1c => CartridgeType::Mbc5Rumble,
            0x1d => CartridgeType::Mbc5RumbleRam,
            0x1e => CartridgeType::Mbc5RumbleRamBattery,
            0x20 => CartridgeType::Mbc6,
            0x22 => CartridgeType::Mbc7SensorRumbleRamBattery,
            0xfc => CartridgeType::PocketCamera,
            0xfd => CartridgeType::BandaiTama5,
            0xfe => CartridgeType::HuC3,
            0xff => CartridgeType::HuC1RamBattery,
            _ => panic!("invalid cartridge type"),
        };

        let rom_size_code = self.mem_read_byte(0x148);

        /*
         * only support 0x00-0x08
         * https://gbdev.io/pandocs/The_Cartridge_Header.html#0148---rom-size
         *
         * "[0x52-54 are] only listed in unofficial docs. No cartridges or ROM files using these sizes are known.
         * As the other ROM sizes are all powers of 2, these are likely inaccurate. The source for
         * these values is unknown."
         */
        if rom_size_code > 0x08 {
            panic!("invalid ROM size");
        }
        self.header.rom_size = 32768 << rom_size_code;

        self.header.ram_size = match self.mem_read_byte(0x149) {
            0x00 => 0, // no RAM
            0x02 => 8192, // 8KiB
            0x03 => 32768, // 32KiB
            0x04 => 131072, // 128KiB
            0x05 => 65536, // 64KiB
            _ => panic!("invalid RAM size"),
        };

        // When using a MBC2 chip, 0x00 must be specified as the RAM Size, even though the MBC2
        // includes a built-in RAM of 512 x 4 bits.
        if (matches!(self.header.cart_type, CartridgeType::Mbc2)
            || matches!(self.header.cart_type, CartridgeType::Mbc2Battery))
           && self.header.ram_size != 0 {
            panic!("RAM size must be 0 when using MBC2 chip");
        }
    }

    fn compare_nintendo_logo(&self) -> bool {
        let nintendo_logo = vec![
            0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b,
            0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
            0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e,
            0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
            0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
            0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e
        ];

        for i in 0..nintendo_logo.len() {
            if self.mem_read_byte(0x104 + i as u16) != nintendo_logo[i] {
                return false;
            }
        }

        true
    }

    fn calc_header_checksum(&self) -> u8 {
        let mut x: u8 = 0;
        for i in 0x134..=0x14c {
            x = x.wrapping_sub(self.mem_read_byte(i)).wrapping_sub(1);
        }

        x
    }

    fn calc_cart_checksum(&self) -> u16 {
        let mut x: u16 = 0;
        for i in 0..self.rom.len() {
            if (i != 0x14e) && (i != 0x14f) {
                x = x.wrapping_add(self.mem_read_byte(i as u16) as u16);
            }
        }

        x
    }
}

impl Memory for Cartridge {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn mem_write_byte(&mut self, _addr: u16, _val: u8) {
        // do nothing
    }
}

