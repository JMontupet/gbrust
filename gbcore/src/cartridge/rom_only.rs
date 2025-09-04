use super::Cartridge;
use crate::Memory;
use core::ops::Index;

pub struct RomOnly<ROM>
where
    ROM: Index<usize, Output = u8>,
{
    rom: ROM,
}

impl<ROM> RomOnly<ROM>
where
    ROM: Index<usize, Output = u8>,
{
    pub fn new(rom: ROM) -> Self {
        Self { rom }
    }
}

impl<ROM> Cartridge for RomOnly<ROM> where ROM: Index<usize, Output = u8> {}

impl<ROM> Memory for RomOnly<ROM>
where
    ROM: Index<usize, Output = u8>,
{
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            _ => panic!("RomOnly read out of range: {:#04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, _: u8) {
        match addr {
            0x0000..=0x7FFF => {}
            _ => panic!("RomOnly write out of range: {:#04x}", addr),
        }
    }
}
