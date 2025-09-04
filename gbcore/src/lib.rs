#![no_std]
#![feature(iter_array_chunks)]
pub mod cartridge;
mod cpu;
mod error;
mod gpu;
mod hram;
mod mmu;
mod system;
mod timers;
mod unusable;
mod wram;

pub use self::error::CoreError;
pub use self::system::System;

pub trait Memory {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub(crate) fn get_bit<const BIT: u32>(n: u8) -> bool {
    n & 1u8.wrapping_shl(BIT) != 0
}

pub(crate) fn set_bit<const BIT: u32>(n: &mut u8, v: bool) {
    *n = (*n & !1u8.wrapping_shl(BIT)) | (v as u8).wrapping_shl(BIT)
}

pub(crate) enum MByte<const ADDR: u16> {}

impl<const ADDR: u16> MByte<ADDR> {
    pub fn read(mem: &mut impl Memory) -> u8 {
        mem.read(ADDR)
    }
    pub fn write(mem: &mut impl Memory, val: u8) {
        mem.write(ADDR, val)
    }
    pub fn bit<const BIT: u8>(mem: &mut impl Memory) -> bool {
        mem.read(ADDR) & (1 << (BIT % 8)) != 0
    }
    pub fn masked<const MASK: u8>(mem: &mut impl Memory) -> u8 {
        mem.read(ADDR) & MASK
    }
}

pub(crate) enum MBit<const ADDR: u16, const BIT: u8> {}

impl<const ADDR: u16, const BIT: u8> MBit<ADDR, BIT> {
    const MASK: u8 = 1 << (BIT % 8);
    pub fn get(mem: &mut impl Memory) -> bool {
        MByte::<ADDR>::read(mem) & Self::MASK != 0
    }
    pub fn set(mem: &mut impl Memory, val: bool) {
        let byte_val = MByte::<ADDR>::read(mem);
        MByte::<ADDR>::write(
            mem,
            match val {
                true => byte_val | Self::MASK,
                false => byte_val & !Self::MASK,
            },
        )
    }
}

const SCREEN_COLORS_DEPTH: usize = 3;
const GB_SCREEN_WIDTH: usize = 160;
const GB_SCREEN_HEIGHT: usize = 144;
const FRAME_BUFFER_SIZE: usize = GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT * SCREEN_COLORS_DEPTH;

pub struct Screen {
    pub frame_buffer: [u8; FRAME_BUFFER_SIZE],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            frame_buffer: [255; FRAME_BUFFER_SIZE],
        }
    }
}

impl Screen {
    fn color_at(&mut self, x: u8, y: u8) -> (u8, u8, u8) {
        let idx = (y as usize * GB_SCREEN_WIDTH + x as usize) * SCREEN_COLORS_DEPTH;
        (
            self.frame_buffer[idx],
            self.frame_buffer[idx + 1],
            self.frame_buffer[idx + 2],
        )
    }

    fn draw_at(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) {
        let idx = (y as usize * GB_SCREEN_WIDTH + x as usize) * SCREEN_COLORS_DEPTH;
        self.frame_buffer[idx] = r;
        self.frame_buffer[idx + 1] = g;
        self.frame_buffer[idx + 2] = b;
    }

    fn line_iterator(&mut self, line: u8) -> impl Iterator<Item = [&mut u8; SCREEN_COLORS_DEPTH]> {
        let idx = line as usize * GB_SCREEN_WIDTH * SCREEN_COLORS_DEPTH;
        let max = GB_SCREEN_WIDTH * SCREEN_COLORS_DEPTH;
        self.frame_buffer[idx..idx + max].iter_mut().array_chunks()
    }
}

pub const KEY_A: u8 = 0b00000001;
pub const KEY_B: u8 = 0b00000010;
pub const KEY_SELECT: u8 = 0b00000100;
pub const KEY_START: u8 = 0b00001000;
pub const KEY_RIGHT: u8 = 0b00010000;
pub const KEY_LEFT: u8 = 0b00100000;
pub const KEY_UP: u8 = 0b01000000;
pub const KEY_DOWN: u8 = 0b10000000;
