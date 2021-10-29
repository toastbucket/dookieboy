use std::cell::RefCell;
use std::rc::Rc;

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
}
