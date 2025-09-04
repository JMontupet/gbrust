pub const COLOR_ZERO: (u8, u8, u8) = (0xED, 0xED, 0xED);

pub struct Colors {
    // FF47 - BGP - BG Palette Data (R/W) - Non CGB Mode Only
    pub ff47_bgp: u8,
    // FF48 - OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only
    pub ff48_obp0: u8,
    // FF49 - OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only
    pub ff49_obp1: u8,

    // FF68 - BCPS/BGPI - CGB Mode Only - Background Palette Index
    pub ff68_bcps_bgpi: u8,
    // FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data
    ff69_bcpd_bgpd: [u8; 64],
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            ff47_bgp: 0,
            ff48_obp0: 0,
            ff49_obp1: 0,
            ff68_bcps_bgpi: 0,
            ff69_bcpd_bgpd: [0; 64],
        }
    }
}

impl Colors {
    // Grayscale

    pub fn bgp_palette(&self) -> Palette {
        Palette::new_mono_palette(self.ff47_bgp)
    }
    pub fn obp0_palette(&self) -> Palette {
        Palette::new_mono_palette(self.ff48_obp0)
    }
    pub fn obp1_palette(&self) -> Palette {
        Palette::new_mono_palette(self.ff49_obp1)
    }

    // Colors

    pub fn cgb_bgp_palette(&self, idx: u8) -> Palette {
        let mut byte_offset = (idx * 8) as usize; // 8 possible palettes
        let mut palette = Palette::default();
        for idx in 0..4 {
            let color_bits = (self.ff69_bcpd_bgpd[byte_offset] as u16)
                | ((self.ff69_bcpd_bgpd[byte_offset + 1] as u16) << 8);
            byte_offset += 2;

            let mut color = (
                (color_bits as u8 & 0x1F) << 3,
                ((color_bits >> 5) as u8 & 0x1F) << 3,
                ((color_bits >> 10) as u8 & 0x1F) << 3,
            );

            if color == (0, 0, 0) {
                color = COLOR_ZERO;
            }
            palette.colors[idx] = color
        }

        palette
    }

    // IO

    pub fn ff69_bcpd_bgpd(&self) -> u8 {
        let byte_idx = self.ff68_bcps_bgpi & 0b111111;
        self.ff69_bcpd_bgpd[byte_idx as usize]
    }

    pub fn set_ff69_bcpd_bgpd(&mut self, value: u8) {
        let mut byte_idx = self.ff68_bcps_bgpi & 0b111111;
        let auto_inc = (self.ff68_bcps_bgpi & 0b10000000) != 0;
        self.ff69_bcpd_bgpd[byte_idx as usize] = value;
        if auto_inc {
            byte_idx = (byte_idx + 1) & 0b111111;
            self.ff68_bcps_bgpi = 0b10000000 | byte_idx;
        }
    }
}

pub struct Palette {
    colors: [(u8, u8, u8); 4],
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            colors: [COLOR_ZERO, COLOR_ZERO, COLOR_ZERO, COLOR_ZERO],
        }
    }
}

impl Palette {
    fn new_mono_palette(raw: u8) -> Self {
        let mut palette = Palette::default();
        for idx in 0..4 {
            palette.colors[idx] = match (raw >> (idx * 2)) & 0x03 {
                0 => COLOR_ZERO,
                1 => (0x99, 0x99, 0x99),
                2 => (0x66, 0x66, 0x66),
                3 => (0x21, 0x21, 0x21),
                _ => panic!(),
            };
        }
        palette
    }

    pub fn color(&self, idx: u8) -> (u8, u8, u8) {
        match idx {
            0 => self.colors[0],
            1 => self.colors[1],
            2 => self.colors[2],
            3 => self.colors[3],
            _ => panic!("Invalid color idx {}", idx),
        }
    }
}
