use crate::{get_bit, set_bit};

#[derive(Default)]
pub(crate) struct LCD {
    // FF40 — LCDC: LCD control
    pub ff40_lcdc: u8,
    // FF41 — STAT: LCD status
    pub ff41_stat: u8,
    // FF42 — SCY: Background viewport Y position
    pub ff42_scy: u8,
    // FF43 — SCX: Background viewport X position
    pub ff43_scx: u8,
    // FF44 - LY - LCDC Y-Coordinate
    pub ff44_ly: u8,
    // FF45 - LYC - LY Compare (R/W)
    pub ff45_lyc: u8,
    // FF42 — WY - Window Y Position
    pub ff4a_wy: u8,
    // FF43 — WX - Window X Position minus 7
    pub ff4b_wx: u8,
}

impl LCD {
    // FF40 - LCDC - LCD Control (R/W)
    //   Bit 7 - LCD Display Enable             (0=Off, 1=On)
    //   Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    //   Bit 5 - Window Display Enable          (0=Off, 1=On)
    //   Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
    //   Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
    //   Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
    //   Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
    //   Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)

    // FF40 Bit 7 - LCD Display Enable (0=Off, 1=On)
    pub fn display_enable(&self) -> bool {
        get_bit::<7>(self.ff40_lcdc)
    }
    // FF40 Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    pub fn window_tile_map(&self) -> bool {
        get_bit::<6>(self.ff40_lcdc)
    }
    // FF40 Bit 5 - Window Display Enable (0=Off, 1=On)
    pub fn window_enable(&self) -> bool {
        get_bit::<5>(self.ff40_lcdc)
    }
    // FF40 Bit 4 - BG & Window Tile Data Select (0=8800-97FF, 1=8000-8FFF)
    pub fn tile_data_select(&self) -> bool {
        get_bit::<4>(self.ff40_lcdc)
    }
    // FF40 Bit 3 - BG Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
    pub fn bg_tile_map(&self) -> bool {
        get_bit::<3>(self.ff40_lcdc)
    }
    // FF40 Bit 2 - OBJ (Sprite) Size (0=8x8, 1=8x16)
    pub fn sprite_size(&self) -> bool {
        get_bit::<2>(self.ff40_lcdc)
    }
    // FF40 Bit 1 - OBJ (Sprite) Display Enable (0=Off, 1=On)
    pub fn sprite_enable(&self) -> bool {
        get_bit::<1>(self.ff40_lcdc)
    }
    // FF40 Bit 0 - BG Display (0=Off, 1=On)= 0
    pub fn bg_enable(&self) -> bool {
        get_bit::<0>(self.ff40_lcdc)
    }

    // FF41 - STAT - LCDC Status (R/W)
    //   Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
    //   Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
    //   Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)
    //   Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)
    //   Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)
    //   Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)
    //             0: During H-Blank
    //             1: During V-Blank
    //             2: During Searching OAM-RAM
    //             3: During Transfering Data to LCD Driver

    // FF41 Bit 6 - LYC=LY Coincidence Interrupt (1=Enable)
    pub fn lyc_ly_coincidence_interrupt(&self) -> bool {
        get_bit::<6>(self.ff41_stat)
    }
    // FF41 Bit 5 - Mode 2 OAM Interrupt         (1=Enable)
    pub fn mode_2_oam_interrupt(&self) -> bool {
        get_bit::<5>(self.ff41_stat)
    }
    // FF41 Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable)
    pub fn mode_1_vblank_interrupt(&self) -> bool {
        get_bit::<4>(self.ff41_stat)
    }
    // Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable)
    pub fn mode_0_hblank_interrupt(&self) -> bool {
        get_bit::<3>(self.ff41_stat)
    }
    // Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY)
    pub fn coincidence_flag_interrupt(&self) -> bool {
        get_bit::<2>(self.ff41_stat)
    }
    // Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY)
    pub fn set_coincidence_flag_interrupt(&mut self, value: bool) {
        set_bit::<2>(&mut self.ff41_stat, value);
    }
    // Bit 1-0 - Mode Flag
    pub fn mode_flag(&self) -> Mode {
        (self.ff41_stat & 0b11).into()
    }
    // Bit 1-0 - Mode Flag
    pub fn set_mode_flag(&mut self, mode: Mode) {
        let m: u8 = mode.into();
        self.ff41_stat = (self.ff41_stat & 0b11111100) | m;
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Mode {
    HBlank,
    VBlank,
    SearchOAM,
    ReadOAM,
}

impl Default for Mode {
    fn default() -> Self {
        Self::HBlank
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::SearchOAM => 2,
            Mode::ReadOAM => 3,
        }
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::SearchOAM,
            3 => Mode::ReadOAM,
            _ => Mode::HBlank,
        }
    }
}
