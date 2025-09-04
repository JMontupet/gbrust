use crate::Memory;

pub struct Unusable {
    mem: [u8; 0xFEFF - 0xFEA0 + 1],
}

impl Default for Unusable {
    fn default() -> Self {
        Self {
            mem: [0; 0xFEFF - 0xFEA0 + 1],
        }
    }
}

impl Memory for Unusable {
    fn read(&mut self, addr: u16) -> u8 {
        self.mem[(addr - 0xFEA0) as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.mem[(addr - 0xFEA0) as usize] = value
    }
}
