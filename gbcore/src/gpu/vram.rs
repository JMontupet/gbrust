use crate::Memory;

pub(crate) struct VRAM {
    // FF4F — VBK (CGB Mode only): VRAM bank
    pub ff4f_vbk: u8,

    // Tiles data: 2 banks
    tile_bk: [[Tile; 128 * 3]; 2],

    // 2 TileMaps shared between 2 banks. For each TileMap:
    // - bank 1 => tile idx
    // - bank 2 => tile attrs
    tile_map: [TileMap; 2],
}

impl Default for VRAM {
    fn default() -> Self {
        Self {
            ff4f_vbk: 0,
            tile_bk: [[Tile::default(); 128 * 3]; 2],
            tile_map: [TileMap::default(); 2],
        }
    }
}

impl Memory for VRAM {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x97FF => {
                self.tile_bk[((self.ff4f_vbk & 1) == 1) as usize][((addr - 0x8000) >> 4) as usize].0
                    [((addr - 0x8000) & 15) as usize]
            }
            0x9800..=0x9BFF => {
                self.tile_map[0].cells[(addr - 0x9800) as usize].0
                    [((self.ff4f_vbk & 1) == 1) as usize]
            }
            0x9C00..=0x9FFF => {
                self.tile_map[1].cells[(addr - 0x9C00) as usize].0
                    [((self.ff4f_vbk & 1) == 1) as usize]
            }
            _ => panic!("VRAM read out {:#04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x97FF => {
                self.tile_bk[((self.ff4f_vbk & 1) == 1) as usize][((addr - 0x8000) >> 4) as usize]
                    .0[((addr - 0x8000) & 15) as usize] = value
            }
            0x9800..=0x9BFF => {
                self.tile_map[0].cells[(addr - 0x9800) as usize].0
                    [((self.ff4f_vbk & 1) == 1) as usize] = value
            }
            0x9C00..=0x9FFF => {
                self.tile_map[1].cells[(addr - 0x9C00) as usize].0
                    [((self.ff4f_vbk & 1) == 1) as usize] = value
            }
            _ => panic!("VRAM write out {:#04x}", addr),
        }
    }
}

impl VRAM {
    pub fn get_tilemap_cell(&self, tile_map: bool, tile_x: u8, tile_y: u8) -> &TileMapCell {
        return &self.tile_map[tile_map as usize].cells[(tile_y as usize) * 32 + (tile_x as usize)];
    }

    pub fn get_tile_data(&self, tile_data_table: bool, bank: bool, idx: u8) -> &Tile {
        match tile_data_table {
            // unsigned start from 8000
            true => &self.tile_bk[bank as usize][idx as usize],
            // signed centered to 9000
            false => {
                &self.tile_bk[bank as usize][256usize.wrapping_add_signed((idx as i8) as isize)]
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
pub(crate) struct Tile(pub [u8; 8 * 2]);

#[derive(Default, Clone, Copy)]
pub(crate) struct TileMapCell([u8; 2]);

impl TileMapCell {
    // Byte 0 : IDX
    pub fn idx(&self) -> u8 {
        self.0[0]
    }

    // Byte 1 : Attributes
    // Bit 0-2  Background Palette number  (BGP0-7)
    pub fn cgb_palette_number(&self) -> u8 {
        self.0[1] & 0b111
    }
    // Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)
    pub fn bank(&self) -> bool {
        (self.0[1] & (1 << 3)) != 0
    }
    // Bit 5    Horizontal Flip            (0=Normal, 1=Mirror horizontally)
    // Bit 6    Vertical Flip              (0=Normal, 1=Mirror vertically)
    // Bit 7    BG-to-OAM Priority         (0=Use OAM priority bit, 1=BG Priority)
    // Bit 4    Not used
}

#[derive(Clone, Copy)]
struct TileMap {
    cells: [TileMapCell; 32 * 32],
}

impl Default for TileMap {
    fn default() -> Self {
        Self {
            cells: [TileMapCell::default(); 32 * 32],
        }
    }
}
