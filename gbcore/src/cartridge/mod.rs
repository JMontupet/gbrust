mod cartridge;
mod mbc1;
mod mbc5;
mod ram;
mod rom;
mod rom_only;

use crate::CoreError;
use crate::Memory;
use crate::cartridge::mbc1::MBC1;
use crate::cartridge::mbc5::MBC5;
use core::ops::Index;
use core::ops::IndexMut;

pub use self::cartridge::CartridgeType;
pub use self::ram::RamType;
pub use self::rom::RomType;

pub trait RealMemory {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, value: u8);
}

pub trait Cartridge: Memory {}

extern crate alloc;
use alloc::boxed::Box;
use rom_only::RomOnly;

pub struct DynCartridge {
    pub cart_type: CartridgeType,
    pub ram_type: RamType,
    pub rom_type: RomType,
    inner: Box<dyn Cartridge>,
}

impl DynCartridge {
    pub fn new<ROM: Index<usize, Output = u8> + 'static>(rom: ROM) -> Result<Self, CoreError> {
        let cart_type = CartridgeType::try_from(rom[0x147])?;
        let rom_type = RomType::try_from(rom[0x148])?;
        let ram_type = RamType::try_from(rom[0x149])?;
        let ram = DynRam::new(&ram_type)?;

        let inner: Box<dyn Cartridge> = match cart_type {
            CartridgeType::ROMOnly => Box::new(RomOnly::new(rom)),
            CartridgeType::MBC1 => Box::new(MBC1::new(
                ram,
                rom,
                rom_type.memory_size() > 512 || ram_type.memory_size() > 8,
            )),
            CartridgeType::MBC1Ram => todo!(),
            CartridgeType::MBC1RamBattery => Box::new(MBC1::new(
                ram,
                rom,
                rom_type.memory_size() > 512 || ram_type.memory_size() > 8,
            )),
            CartridgeType::MBC2 => todo!(),
            CartridgeType::MBC2Battery => todo!(),
            CartridgeType::RomRam => todo!(),
            CartridgeType::RomRamBattery => todo!(),
            CartridgeType::MMM01 => todo!(),
            CartridgeType::MMM01Ram => todo!(),
            CartridgeType::MMM01RamBattery => todo!(),
            CartridgeType::MBC3TimerBattery => todo!(),
            CartridgeType::MBC3TimerRamBattery => todo!(),
            CartridgeType::MBC3 => todo!(),
            CartridgeType::MBC3Ram => todo!(),
            CartridgeType::MBC3RamBattery => todo!(),
            CartridgeType::MBC5 => todo!(),
            CartridgeType::MBC5Ram => todo!(),
            CartridgeType::MBC5RamBattery => Box::new(MBC5::new(ram, rom)),
            CartridgeType::MBC5Rumble => todo!(),
            CartridgeType::MBC5RumbleRam => todo!(),
            CartridgeType::MBC5RumbleRamBattery => todo!(),
            CartridgeType::MBC6 => todo!(),
            CartridgeType::MBC7SensorRumbleRamBattery => todo!(),
            CartridgeType::PocketCamera => todo!(),
            CartridgeType::BandaiTama5 => todo!(),
            CartridgeType::HuC3 => todo!(),
            CartridgeType::HuC1RamBattery => todo!(),
        };
        Ok(Self {
            inner,
            ram_type,
            rom_type,
            cart_type,
        })
    }
}

impl Cartridge for DynCartridge {}

impl Memory for DynCartridge {
    fn read(&mut self, addr: u16) -> u8 {
        self.inner.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.inner.write(addr, value);
    }
}

struct DynRam {
    inner: Box<dyn IndexMut<usize, Output = u8>>,
}

impl DynRam {
    fn new(t: &RamType) -> Result<Self, CoreError> {
        match t {
            RamType::RamBankNoRam => Ok(Self {
                inner: Box::new([0u8; 0]),
            }),
            RamType::RamBankUnused => todo!(),
            RamType::RamBank8KByte => Ok(Self {
                inner: Box::new([0u8; 0x2000]),
            }),
            RamType::RamBank32KByte => Ok(Self {
                inner: Box::new([0u8; 0x2000 * 4]),
            }),
            RamType::RamBank128KByte => Ok(Self {
                inner: Box::new([0u8; 0x2000 * 16]),
            }),
            RamType::RamBank64KByte => Ok(Self {
                inner: Box::new([0u8; 0x2000 * 8]),
            }),
        }
    }
}

impl Index<usize> for DynRam {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

impl IndexMut<usize> for DynRam {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}
