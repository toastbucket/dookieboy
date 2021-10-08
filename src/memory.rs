// Abstract interface for simulating memory access

pub trait Memory {
    fn mem_read_byte(&self, addr: u16) -> u8;

    fn mem_read_word_le(&self, addr: u16) -> u16 {
        u16::from(self.mem_read_byte(addr)) | (u16::from(self.mem_read_byte(addr + 1)) << 8)
    }

    fn mem_read_word_be(&self, addr: u16) -> u16 {
        (u16::from(self.mem_read_byte(addr)) << 8) | u16::from(self.mem_read_byte(addr + 1))
    }

    fn mem_write_byte(&mut self, addr: u16, val: u8);

    fn mem_write_word_le(&mut self, addr: u16, val: u16) {
        self.mem_write_byte(addr, (val & 0xff) as u8);
        self.mem_write_byte(addr + 1, ((val >> 8) & 0xff) as u8);
    }

    fn mem_write_word_be(&mut self, addr: u16, val: u16) {
        self.mem_write_byte(addr, ((val >> 8) & 0xff) as u8);
        self.mem_write_byte(addr + 1, (val & 0xff) as u8);
    }

    fn mem_write_byte_field(&mut self, addr: u16, val: u8, mask: u8, shift: u8) {
        let mut byte = self.mem_read_byte(addr);
        byte |= (val << shift) & mask;
        self.mem_write_byte(addr, byte);
    }

    fn mem_is_set(&self, addr: u16, pos: u8) -> bool {
        (self.mem_read_byte(addr) & (1 << pos)) != 0
    }

    fn mem_set_bit(&mut self, addr: u16, pos: u8) {
        let byte = self.mem_read_byte(addr);
        self.mem_write_byte(addr, byte | (1 << pos));
    }

    fn mem_clear_bit(&mut self, addr: u16, pos: u8) {
        let byte = self.mem_read_byte(addr);
        self.mem_write_byte(addr, byte & !(1 << pos));
    }

    fn mem_toggle_bit(&mut self, addr: u16, pos: u8) {
        let byte = self.mem_read_byte(addr);
        self.mem_write_byte(addr, byte ^ (1 << pos));
    }
}
