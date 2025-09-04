use super::Cartridge;
use crate::Memory;
use core::ops::{Index, IndexMut};

pub struct MBC5<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
    ram: RAM,
    rom: ROM,

    ram_bank: usize,
    rom_bank: usize,

    ram_enable: bool,
}

impl<RAM, ROM> MBC5<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
    pub fn new(ram: RAM, rom: ROM) -> Self {
        Self {
            ram,
            rom,
            ram_bank: 0,
            rom_bank: 0,
            ram_enable: false,
        }
    }
}

impl<RAM, ROM> Cartridge for MBC5<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
}

impl<RAM, ROM> Memory for MBC5<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // 0000–3FFF — ROM Bank 00
            0x0000..=0x3FFF => self.rom[addr as usize],
            // 4000–7FFF —  ROM bank 00-1FF
            0x4000..=0x7FFF => self.rom[self.rom_bank * 0x4000 + addr as usize - 0x4000],
            // A000–BFFF — RAM bank 00-0F, if any
            0xA000..=0xBFFF => match self.ram_enable {
                true => self.ram[self.ram_bank * 0x2000 + addr as usize - 0xA000],
                false => 0x00,
            },
            _ => panic!("MBC5 read out of range: {:#04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // 0000–1FFF — RAM Enable
            0x0000..=0x1FFF => self.ram_enable = value & 0x0F == 0x0A,
            // 2000–2FFF — 8 least significant bits of ROM bank number
            0x2000..=0x2FFF => self.rom_bank = (self.rom_bank & !0xFF) | value as usize,
            // 3000-3FFF - 9th bit of ROM bank number
            0x3000..=0x3FFF => {
                self.rom_bank =
                    (self.rom_bank & !(1 << 8)) | ((((value & 0x01) != 0x00) as usize) << 8)
            }
            // 4000-5FFF - RAM bank number
            0x4000..=0x5FFF => self.ram_bank = value as usize & 0x0F,
            // 6000-7FFF - Latch Clock Data
            0x6000..=0x7FFF => {
                // TODO
            }
            // A000–BFFF — RAM bank 00-0F, if any
            0xA000..=0xBFFF => match self.ram_enable {
                true => self.ram[self.ram_bank * 0x2000 + addr as usize - 0xA000] = value,
                false => {}
            },
            _ => panic!("MBC5 write out of range: {:#04x}", addr),
        }
    }
}
