use crate::{Memory, cartridge::Cartridge, get_bit, mmu::MMU};

pub(crate) struct OAM {
    ff46_dma: u8,
    pub sprites: [Sprite; 40],
    pub dma_transfer_requested: bool,
}

impl OAM {
    pub fn ff46_dma(&self) -> u8 {
        self.ff46_dma
    }
    pub fn set_ff46_dma(&mut self, value: u8) {
        self.ff46_dma = value;
        self.dma_transfer_requested = true;
    }
}

impl Default for OAM {
    fn default() -> Self {
        Self {
            ff46_dma: 0,
            sprites: [Sprite::default(); 40],
            dma_transfer_requested: false,
        }
    }
}

impl Memory for OAM {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xFE00..=0xFE9F => {
                let sprite_idx = ((addr - 0xFE00) as usize) >> 2;
                match ((addr - 0xFE00) as usize) & 0b11 {
                    0 => self.sprites[sprite_idx].y_pos,
                    1 => self.sprites[sprite_idx].x_pos,
                    2 => self.sprites[sprite_idx].tile_number,
                    3 => self.sprites[sprite_idx].flags,
                    _ => panic!("Sprite read out {:#04x}", addr),
                }
            }
            _ => panic!("OAM read out {:#04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFE00..=0xFE9F => {
                let sprite_idx = ((addr - 0xFE00) as usize) >> 2;
                match ((addr - 0xFE00) as usize) & 0b11 {
                    0 => self.sprites[sprite_idx].y_pos = value,
                    1 => self.sprites[sprite_idx].x_pos = value,
                    2 => self.sprites[sprite_idx].tile_number = value,
                    3 => self.sprites[sprite_idx].flags = value,
                    _ => panic!("Sprite write out {:#04x}", addr),
                }
            }
            _ => panic!("OAM write out {:#04x}", addr),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub(crate) struct Sprite {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_number: u8,
    // Byte3 - Attributes/Flags:
    //     Bit7   OBJ-to-BG Priority (0=OBJ Above BG, 1=OBJ Behind BG color 1-3)
    //             (Used for both BG and Window. BG color 0 is always behind OBJ)
    //     Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
    //     Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
    //     Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
    //     Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
    //     Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
    pub flags: u8,
}

impl Sprite {
    // Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
    pub fn palette(&self) -> bool {
        get_bit::<4>(self.flags)
    }

    // Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
    pub fn x_flip(&self) -> bool {
        get_bit::<5>(self.flags)
    }

    // Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
    pub fn y_flip(&self) -> bool {
        get_bit::<6>(self.flags)
    }

    // Bit7   OBJ-to-BG Priority (0=OBJ Above BG, 1=OBJ Behind BG color 1-3)
    //             (Used for both BG and Window. BG color 0 is always behind OBJ)
    pub fn obj_to_bg_priority(&self) -> bool {
        get_bit::<7>(self.flags)
    }
}

const TRANSFER_DST: u16 = 0xFE00;

#[derive(Default)]
pub(crate) struct OamDmaManager {}

impl OamDmaManager {
    pub fn tick<C: Cartridge>(&mut self, mmu: &mut MMU<C>) {
        if !mmu.oam.dma_transfer_requested {
            return;
        }
        let mut src = (mmu.oam.ff46_dma() as u16) << 8;
        let mut dst = TRANSFER_DST;
        for _ in 0..160 {
            let byte = mmu.read(src);
            mmu.write(dst, byte);
            src += 1;
            dst += 1;
        }

        mmu.oam.dma_transfer_requested = false;
    }
}
