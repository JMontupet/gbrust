use crate::Memory;

pub struct HRAM {
    mem: [u8; 0xFFFE - 0xFF80 + 1],
}

impl Default for HRAM {
    fn default() -> Self {
        Self {
            mem: [0; 0xFFFE - 0xFF80 + 1],
        }
    }
}

impl Memory for HRAM {
    fn read(&mut self, addr: u16) -> u8 {
        self.mem[(addr - 0xFF80) as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.mem[(addr - 0xFF80) as usize] = value
    }
}
