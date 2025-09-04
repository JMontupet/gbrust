use crate::Memory;

pub struct WRAM {
    wram: [u8; 0xCFFF - 0xC000 + 1],
    sram: [u8; 0xDFFF - 0xD000 + 1],
}

impl Default for WRAM {
    fn default() -> Self {
        Self {
            wram: [0; 0xCFFF - 0xC000 + 1],
            sram: [0; 0xDFFF - 0xD000 + 1],
        }
    }
}

impl Memory for WRAM {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xC000..=0xCFFF => self.wram[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => self.sram[(addr - 0xD000) as usize],
            _ => panic!("WRAM read out {:#04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xC000..=0xCFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.sram[(addr - 0xD000) as usize] = value,
            _ => panic!("WRAM write out {:#04x}", addr),
        }
    }
}
