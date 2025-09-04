use super::Cartridge;
use crate::Memory;
use core::ops::{Index, IndexMut};

pub struct MBC1<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
    banking_mode_select: bool,

    ram: RAM,
    rom: ROM,

    ram_bank: u8,
    rom_bank: u8,

    ram_enable: bool,
    lower_rom_bank: u8,
    rom_bank_reg: u8,               // 5 bits max
    bank_or_upper_rom_bank_reg: u8, // 2 bits
    banked_reg: bool,
}

impl<RAM, ROM> MBC1<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
    pub fn new(ram: RAM, rom: ROM, banking_mode_select: bool) -> Self {
        Self {
            banking_mode_select,
            ram,
            rom,
            ram_bank: 0,
            lower_rom_bank: 0,
            rom_bank: 1,
            ram_enable: false,
            rom_bank_reg: 1,
            bank_or_upper_rom_bank_reg: 0,
            banked_reg: false,
        }
    }

    fn update_ram_bank(&mut self) {
        match self.banked_reg {
            true => self.ram_bank = self.bank_or_upper_rom_bank_reg,
            false => self.ram_bank = 0,
        }
    }
    fn update_rom_bank(&mut self) {
        match self.banked_reg {
            true => {
                self.lower_rom_bank = self.bank_or_upper_rom_bank_reg << 5;
                self.rom_bank = self.rom_bank_reg;
            }
            false => {
                self.lower_rom_bank = 0;
                self.rom_bank = (self.bank_or_upper_rom_bank_reg << 5) + self.rom_bank_reg;
            }
        };
    }
}

impl<RAM, ROM> Cartridge for MBC1<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
}

impl<RAM, ROM> Memory for MBC1<RAM, ROM>
where
    RAM: IndexMut<usize, Output = u8>,
    ROM: Index<usize, Output = u8>,
{
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // 0000–3FFF — ROM Bank X0
            0x0000..=0x3FFF => self.rom[self.lower_rom_bank as usize * 0x4000 + addr as usize],
            // 4000–7FFF — ROM Bank 01-7F
            0x4000..=0x7FFF => self.rom[self.rom_bank as usize * 0x4000 + addr as usize - 0x4000],
            // A000–BFFF — RAM Bank 00–03, if any
            0xA000..=0xBFFF => match self.ram_enable {
                true => self.ram[self.ram_bank as usize * 0x2000 + addr as usize - 0xA000],
                false => 0x00,
            },
            _ => panic!("MBC1 read out of range: {:#04x}", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // 0000–1FFF — RAM Enable
            0x0000..=0x1FFF => self.ram_enable = value & 0x0F == 0x0A,

            // 2000–3FFF — ROM Bank Number (5 lower bits)
            0x2000..=0x3FFF => {
                // Take 5 bits
                let mut value: u8 = value & 0b11111;
                // Remap bank 0 -> 1
                if value == 0 {
                    value = 1;
                }
                self.rom_bank_reg = value;
                self.update_rom_bank()
            }
            // 4000–5FFF — RAM Bank Number — or — Upper Bits of ROM Bank Number (2 upper bits)
            0x4000..=0x5FFF => {
                self.bank_or_upper_rom_bank_reg = value & 0b11;
                self.update_rom_bank();
                self.update_ram_bank();
            }
            // 6000–7FFF — Banking Mode Select
            0x6000..=0x7FFF => {
                if self.banking_mode_select {
                    // self.bank_or_upper_rom_bank_reg = 0; ???
                    self.banked_reg = (value & 0b1) == 1;
                    self.update_rom_bank();
                    self.update_ram_bank();
                }
            }
            // A000–BFFF — RAM Bank 00–03, if any
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram[self.ram_bank as usize * 0x2000 + addr as usize - 0xA000] = value;
                }
            }

            _ => panic!("MBC1 write out of range: {:#04x}", addr),
        }
    }
}
