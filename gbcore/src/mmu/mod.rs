mod interrupt;

use crate::{
    Memory,
    cartridge::Cartridge,
    gpu::{colors::Colors, lcd::LCD, oam::OAM, vram::VRAM},
    hram::HRAM,
    mmu::interrupt::Interrupt,
    unusable::Unusable,
    wram::WRAM,
};

pub(crate) struct MMU<C: Cartridge>
where
    C: Cartridge,
{
    cartridge: C,
    hram: HRAM,
    wram: WRAM,
    unusable: Unusable,

    pub colors: Colors,
    pub oam: OAM,
    pub interrupt: Interrupt,
    pub lcd: LCD,
    pub vram: VRAM,

    io: [u8; 0xFF7F - 0xFF00 + 1], // For other IO
}

impl<C> MMU<C>
where
    C: Cartridge,
{
    pub fn new(
        cartridge: C,
        hram: HRAM,
        wram: WRAM,
        unusable: Unusable,
        vram: VRAM,
        oam: OAM,
    ) -> Self {
        MMU {
            cartridge,
            interrupt: Interrupt::default(),
            hram,
            wram,
            unusable,
            vram,
            oam,
            lcd: LCD::default(),
            colors: Colors::default(),
            io: [0; 0xFF7F - 0xFF00 + 1],
        }
    }
}

impl<'a, C> Memory for MMU<C>
where
    C: Cartridge,
{
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // Boot Room
            0x0000..=0x00FF => self.cartridge.read(addr),
            // 16 KiB ROM bank 00
            0x0100..=0x3FFF => self.cartridge.read(addr),
            // 16 KiB ROM Bank 01–NN
            0x4000..=0x7FFF => self.cartridge.read(addr),
            // 8 KiB Video RAM (VRAM) - Switchable 0-1 in CGB modec
            0x8000..=0x9FFF => self.vram.read(addr),
            // 8 KiB External RAM
            0xA000..=0xBFFF => self.cartridge.read(addr),
            // 4 KiB Work RAM (WRAM)
            0xC000..=0xCFFF => self.wram.read(addr),
            // 4 KiB Work RAM (WRAM) = Switchable bank 1–7 in CGB mode
            0xD000..=0xDFFF => self.wram.read(addr),
            // Echo RAM (mirror of C000–DDFF)
            0xE000..=0xFDFF => self.wram.read(addr - 0x2000),
            // OAM
            0xFE00..=0xFE9F => self.oam.read(addr),
            // Not Usable
            0xFEA0..=0xFEFF => self.unusable.read(addr),
            // Interrupt flag
            0xFF0F => self.interrupt.ff0f_if,
            // LCDC: LCD control
            0xFF40 => self.lcd.ff40_lcdc,
            // STAT: LCD status
            0xFF41 => self.lcd.ff41_stat,
            // SCY: Background viewport Y position
            0xFF42 => self.lcd.ff42_scy,
            // SCX: Background viewport X position
            0xFF43 => self.lcd.ff43_scx,
            // LY - LCDC Y-Coordinate
            0xFF44 => self.lcd.ff44_ly,
            // LYC - LY Compare
            0xFF45 => self.lcd.ff45_lyc,
            // DMA: OAM DMA source address & start
            0xFF46 => self.oam.ff46_dma(),
            // BGP - BG Palette Data (R/W) - Non CGB Mode Only
            0xFF47 => self.colors.ff47_bgp,
            // OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only
            0xFF48 => self.colors.ff48_obp0,
            // OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only
            0xFF49 => self.colors.ff49_obp1,
            // WY - Window Y Position
            0xFF4A => self.lcd.ff4a_wy,
            // WX - Window X Position minus 7
            0xFF4B => self.lcd.ff4b_wx,
            // VBK (CGB Mode only): VRAM bank
            0xFF4F => self.vram.ff4f_vbk,
            // BCPS/BGPI - CGB Mode Only - Background Palette Index
            0xFF68 => self.colors.ff68_bcps_bgpi,
            // BCPD/BGPD - CGB Mode Only - Background Palette Data
            0xFF69 => self.colors.ff69_bcpd_bgpd(),
            // I/O
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            // HRAM
            0xFF80..=0xFFFE => self.hram.read(addr),
            // Interrupt enable
            0xFFFF => self.interrupt.ffff_ie,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // Boot Room
            0x0000..=0x00FF => self.cartridge.write(addr, value),
            // 16 KiB ROM bank 00
            0x0100..=0x3FFF => self.cartridge.write(addr, value),
            // 16 KiB ROM Bank 01–NN
            0x4000..=0x7FFF => self.cartridge.write(addr, value),
            // 8 KiB Video RAM (VRAM) - Switchable 0-1 in CGB modec
            0x8000..=0x9FFF => self.vram.write(addr, value),
            // 8 KiB External RAM
            0xA000..=0xBFFF => self.cartridge.write(addr, value),
            // 4 KiB Work RAM (WRAM)
            0xC000..=0xCFFF => self.wram.write(addr, value),
            // 4 KiB Work RAM (WRAM) = Switchable bank 1–7 in CGB mode
            0xD000..=0xDFFF => self.wram.write(addr, value),
            // Echo RAM (mirror of C000–DDFF)
            0xE000..=0xFDFF => self.wram.write(addr - 0x2000, value),
            // OAM
            0xFE00..=0xFE9F => self.oam.write(addr, value),
            // Not Usable
            0xFEA0..=0xFEFF => self.unusable.write(addr, value),
            // Interrupt flag
            0xFF0F => self.interrupt.ff0f_if = value,
            // LCDC: LCD control
            0xFF40 => self.lcd.ff40_lcdc = value,
            // STAT: LCD status
            0xFF41 => self.lcd.ff41_stat = value,
            // SCY: Background viewport Y position
            0xFF42 => self.lcd.ff42_scy = value,
            // SCX: Background viewport X position
            0xFF43 => self.lcd.ff43_scx = value,
            // LY - LCDC Y-Coordinate
            0xFF44 => self.lcd.ff44_ly = value,
            // LYC - LY Compare
            0xFF45 => self.lcd.ff45_lyc = value,
            // DMA: OAM DMA source address & start
            0xFF46 => self.oam.set_ff46_dma(value),
            // BGP - BG Palette Data (R/W) - Non CGB Mode Only
            0xFF47 => self.colors.ff47_bgp = value,
            // OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only
            0xFF48 => self.colors.ff48_obp0 = value,
            // OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only
            0xFF49 => self.colors.ff49_obp1 = value,
            // WY - Window Y Position
            0xFF4A => self.lcd.ff4a_wy = value,
            // WX - Window X Position minus 7
            0xFF4B => self.lcd.ff4b_wx = value,
            // VBK (CGB Mode only): VRAM bank
            0xFF4F => self.vram.ff4f_vbk = value,
            // BCPS/BGPI - CGB Mode Only - Background Palette Index
            0xFF68 => self.colors.ff68_bcps_bgpi = value,
            // BCPD/BGPD - CGB Mode Only - Background Palette Data
            0xFF69 => self.colors.set_ff69_bcpd_bgpd(value),
            // I/O
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = value,
            // HRAM
            0xFF80..=0xFFFE => self.hram.write(addr, value),
            // Interrupt
            0xFFFF => self.interrupt.ffff_ie = value,
        }
    }
}
