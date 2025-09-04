use crate::{
    MByte, Memory, Screen,
    cartridge::Cartridge,
    gpu::{colors::COLOR_ZERO, lcd::Mode},
    mmu::MMU,
};

pub mod colors;
pub mod lcd;
pub mod oam;
pub mod vram;

const SEARCH_OAM_LENGTH: usize = 80;
const READ_OAM_LENGTH: usize = 172;
const HBLANK_LENGTH: usize = 204;
const VBLANK_LENGTH: usize = 4560;
const FULL_FRAME: usize =
    (SEARCH_OAM_LENGTH + READ_OAM_LENGTH + SEARCH_OAM_LENGTH) * 144 + VBLANK_LENGTH;

type BgPalette = MByte<0xFF47>; //      BGP  - BG Palette Data
type SpritePalette0 = MByte<0xFF48>; // OBP0 - Object Palette 0 Data
type SpritePalette1 = MByte<0xFF49>; // OBP1 - Object Palette 1 Data

#[derive(Default)]
pub(crate) struct GPU {
    current_mode: Mode, // Can not rely on FF41
    current_mode_length: usize,

    disabled_length: usize,

    screen: Screen,

    color_mode: bool,
}

impl GPU {
    pub fn new(color: bool) -> Self {
        let mut gpu = Self::default();
        gpu.color_mode = color;
        gpu
    }

    pub fn swap_screen(&mut self, screen: &mut Screen) {
        core::mem::swap(&mut self.screen, screen);
    }

    pub fn tick<C: Cartridge>(&mut self, mmu: &mut MMU<C>, ticks: u8) -> State {
        if !mmu.lcd.display_enable() {
            if self.disabled_length >= FULL_FRAME {
                self.disabled_length = 0;
                return State::Frame;
            }
            self.disabled_length += 1;
            return State::Default;
        }

        self.current_mode_length += ticks as usize;
        let state = match self.current_mode {
            Mode::HBlank => self.do_hblank(mmu),
            Mode::VBlank => {
                self.do_vblank(mmu);
                State::Default
            }
            Mode::SearchOAM => {
                self.do_search_oam(mmu);
                State::Default
            }
            Mode::ReadOAM => {
                self.do_read_oam(mmu);
                State::Default
            }
        };

        // Compare ly - lyc
        if mmu.lcd.ff44_ly == mmu.lcd.ff45_lyc {
            mmu.lcd.set_coincidence_flag_interrupt(true);
            if mmu.lcd.lyc_ly_coincidence_interrupt() {
                mmu.interrupt.set_lcd_stat_interrupt_request(true);
            }
        } else {
            mmu.lcd.set_coincidence_flag_interrupt(false)
        }

        state
    }

    fn switch_mode<C: Cartridge>(&mut self, mem: &mut MMU<C>, mode: Mode) {
        match mode {
            Mode::HBlank => {}
            Mode::VBlank => {
                mem.interrupt.set_vblank_interrupt_request(true);
            }
            Mode::SearchOAM => {}
            Mode::ReadOAM => {}
        }
        mem.lcd.set_mode_flag(mode);
        self.current_mode_length = 0;
        self.current_mode = mode;
    }

    fn do_hblank<C: Cartridge>(&mut self, mmu: &mut MMU<C>) -> State {
        match self.current_mode_length {
            // HBLANK continues
            0..=HBLANK_LENGTH => State::Default,
            // End of HBLANK
            _ => {
                mmu.lcd.ff44_ly = mmu.lcd.ff44_ly.wrapping_add(1);
                match mmu.lcd.ff44_ly {
                    // End of line
                    0..=143 => {
                        self.switch_mode(mmu, Mode::SearchOAM);
                        State::Default
                    }
                    // End of frame, start VBLANK
                    144 => {
                        self.switch_mode(mmu, Mode::VBlank);
                        State::Frame
                    }
                    _ => panic!("Bad ly value at the end of hblank"),
                }
            }
        }
    }

    fn do_vblank<C: Cartridge>(&mut self, mmu: &mut MMU<C>) {
        match self.current_mode_length {
            // VBLANK continues
            0..=VBLANK_LENGTH => {
                if self.current_mode_length % 456 == 0 {
                    mmu.lcd.ff44_ly = mmu.lcd.ff44_ly.wrapping_add(1);
                }
            }
            // End of VBLANK
            _ => {
                mmu.lcd.ff44_ly = 0;
                self.switch_mode(mmu, Mode::SearchOAM);
            }
        }
    }
    fn do_search_oam<C: Cartridge>(&mut self, mmu: &mut MMU<C>) {
        match self.current_mode_length {
            // SEARCH OAM continues
            0..=SEARCH_OAM_LENGTH => {}
            // End of SEARCH OAM
            _ => {
                self.switch_mode(mmu, Mode::ReadOAM);
            }
        }
    }
    fn do_read_oam<C: Cartridge>(&mut self, mmu: &mut MMU<C>) -> State {
        match self.current_mode_length {
            // READ OAM continues
            0..=READ_OAM_LENGTH => State::Default,
            // End of READ OAM
            _ => {
                // Draw
                self.draw_line(mmu);
                self.switch_mode(mmu, Mode::HBlank);
                State::Default
            }
        }
    }

    fn draw_line<C: Cartridge>(&mut self, mmu: &mut MMU<C>) {
        let line = mmu.lcd.ff44_ly;

        if mmu.lcd.bg_enable() {
            self.draw_bg_line(mmu, line);
        }
        if mmu.lcd.window_enable() {
            self.draw_window_line(mmu, line);
        }
        if mmu.lcd.sprite_enable() {
            self.draw_sprite_line(mmu, line);
        }
    }

    fn draw_bg_line<C: Cartridge>(&mut self, mmu: &mut MMU<C>, line: u8) {
        let mut palette = mmu.colors.bgp_palette();
        let mut pixel_x: u8 = 0;

        let scroll_x: u8 = mmu.lcd.ff43_scx;
        let scroll_y: u8 = line.wrapping_add(mmu.lcd.ff42_scy);

        let mut in_tile_x: u8 = scroll_x & 0b111; // Mod 8
        let in_tile_y: u8 = scroll_y & 0b111; // Mod 8
        let in_tile_byte_offset: usize = (in_tile_y as usize) * 2;
        let mut to_draw: u8 = 8;

        let mut screen_it = self.screen.line_iterator(line);

        let active_tile_map = mmu.lcd.bg_tile_map();
        let active_tile_data_table = mmu.lcd.tile_data_select();
        while pixel_x < 160 {
            let tile_map_cell = mmu.vram.get_tilemap_cell(
                active_tile_map,
                scroll_x.wrapping_add(pixel_x) >> 3,
                scroll_y >> 3,
            );
            let tile_data = mmu.vram.get_tile_data(
                active_tile_data_table,
                tile_map_cell.bank(),
                tile_map_cell.idx(),
            );

            if self.color_mode {
                palette = mmu
                    .colors
                    .cgb_bgp_palette(tile_map_cell.cgb_palette_number());
            }

            let lo = tile_data.0[in_tile_byte_offset];
            let hi = tile_data.0[in_tile_byte_offset + 1];

            for b in in_tile_x..to_draw {
                let o = 7 - b;
                let color_idx = ((lo >> o) & 0x01) | (((hi >> o) & 0x01) << 1);
                let color = palette.color(color_idx);

                let rgb = screen_it.next().unwrap();
                *rgb[0] = color.0;
                *rgb[1] = color.1;
                *rgb[2] = color.2;

                pixel_x += 1;
            }
            to_draw = 160 - pixel_x;
            if to_draw > 8 {
                to_draw = 8;
            }
            in_tile_x = 0;
        }
    }
    fn draw_window_line<C: Cartridge>(&mut self, mmu: &mut MMU<C>, line: u8) {
        // No need to draw if the window is above the current line.
        if line < mmu.lcd.ff4a_wy {
            return;
        }

        // The window becomes visible when positions are set in range WX=0..166, WY=0..143
        if mmu.lcd.ff4b_wx > 166 || mmu.lcd.ff4a_wy > 143 {
            return;
        }

        let palette = mmu.colors.bgp_palette();
        let mut pixel_x: u8 = 0;

        let scroll_x: u8 = mmu.lcd.ff4b_wx.wrapping_sub(7);
        let scroll_y: u8 = line.wrapping_add(mmu.lcd.ff4a_wy);

        let mut in_tile_x: u8 = scroll_x & 0b111; // Mod 8
        let in_tile_y: u8 = scroll_y & 0b111; // Mod 8
        let in_tile_byte_offset: usize = (in_tile_y as usize) * 2;
        let mut to_draw: u8 = 8;

        let mut screen_it = self.screen.line_iterator(line);

        let active_tile_map = mmu.lcd.window_tile_map();
        let active_tile_data_table = mmu.lcd.tile_data_select();
        while pixel_x < 160 {
            let tile_map_cell = mmu.vram.get_tilemap_cell(
                active_tile_map,
                scroll_x.wrapping_add(pixel_x) >> 3,
                scroll_y >> 3,
            );
            let tile_data = mmu.vram.get_tile_data(
                active_tile_data_table,
                tile_map_cell.bank(),
                tile_map_cell.idx(),
            );

            let lo = tile_data.0[in_tile_byte_offset];
            let hi = tile_data.0[in_tile_byte_offset + 1];

            for b in in_tile_x..to_draw {
                let o = 7 - b;
                let color_idx = ((lo >> o) & 0x01) | (((hi >> o) & 0x01) << 1);
                let color = palette.color(color_idx);

                let rgb = screen_it.next().unwrap();
                *rgb[0] = color.0;
                *rgb[1] = color.1;
                *rgb[2] = color.2;

                pixel_x += 1;
            }
            to_draw = 160 - pixel_x;
            if to_draw > 8 {
                to_draw = 8;
            }
            in_tile_x = 0;
        }
    }

    fn draw_sprite_line<C: Cartridge>(&mut self, mmu: &mut MMU<C>, line: u8) {
        let mode_8x16 = mmu.lcd.sprite_size();
        let max_height = match mode_8x16 {
            true => 16,
            false => 8,
        };

        let line = line as isize;
        for sprite in mmu.oam.sprites.iter() {
            if sprite.y_pos == 0 || sprite.y_pos >= 160 {
                continue; // Offscreen Y
            }

            let top_left_y = (sprite.y_pos as isize) - 16;
            if top_left_y > line || top_left_y + max_height <= line {
                continue; // Not on the line
            }
            let top_left_x = (sprite.x_pos as isize) - 8;

            // TODO: Offscreen sprite affects priority.
            let offscreen = sprite.x_pos == 0 || sprite.x_pos >= 168;
            if !offscreen {
                let mut sprite_line = line - top_left_y;
                let mut tile_number = sprite.tile_number;

                if sprite.y_flip() {
                    if mode_8x16 {
                        sprite_line = (sprite_line - 15) * -1
                    } else {
                        sprite_line = (sprite_line - 7) * -1
                    }
                }

                if mode_8x16 {
                    if sprite_line > 7 {
                        sprite_line %= 8;
                        tile_number |= 0x01
                    }
                }

                let in_tile_byte_offset: usize = (sprite_line as usize) * 2;

                let palette = match sprite.palette() {
                    true => mmu.colors.obp1_palette(),
                    false => mmu.colors.obp0_palette(),
                };
                let obj_to_bg_priority = sprite.obj_to_bg_priority();
                let tile_data = mmu.vram.get_tile_data(true, false, tile_number);

                let mut lo = tile_data.0[in_tile_byte_offset];
                let mut hi = tile_data.0[in_tile_byte_offset + 1];

                if sprite.x_flip() {
                    lo = lo.reverse_bits();
                    hi = hi.reverse_bits();
                }

                for i in 0..8 {
                    let pixel_x = top_left_x + i;
                    if pixel_x >= 160 {
                        break;
                    }
                    if pixel_x < 0 {
                        continue;
                    }
                    let o = 7 - i;
                    let color_idx = ((lo >> o) & 0x01) | (((hi >> o) & 0x01) << 1);
                    // tranparent
                    if color_idx == 0 {
                        continue;
                    }
                    let color = palette.color(color_idx);

                    // BG color 0 is always behind OBJ
                    if self.screen.color_at(pixel_x as u8, line as u8) == COLOR_ZERO {
                        self.screen
                            .draw_at(pixel_x as u8, line as u8, color.0, color.1, color.2);
                        continue;
                    }
                    // sprite behing bg
                    if !obj_to_bg_priority {
                        self.screen
                            .draw_at(pixel_x as u8, line as u8, color.0, color.1, color.2);
                    }
                }
            }
        }
    }
}

pub enum State {
    Default,
    Frame,
}
