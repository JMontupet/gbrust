use crate::CoreError;

#[derive(Debug)]
pub enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    MBC2,
    MBC2Battery,
    RomRam,
    RomRamBattery,
    MMM01,
    MMM01Ram,
    MMM01RamBattery,
    MBC3TimerBattery,
    MBC3TimerRamBattery,
    MBC3,
    MBC3Ram,
    MBC3RamBattery,
    MBC5,
    MBC5Ram,
    MBC5RamBattery,
    MBC5Rumble,
    MBC5RumbleRam,
    MBC5RumbleRamBattery,
    MBC6,
    MBC7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl TryFrom<u8> for CartridgeType {
    type Error = CoreError;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CartridgeType::ROMOnly),
            0x01 => Ok(CartridgeType::MBC1),
            0x02 => Ok(CartridgeType::MBC1Ram),
            0x03 => Ok(CartridgeType::MBC1RamBattery),
            0x05 => Ok(CartridgeType::MBC2),
            0x06 => Ok(CartridgeType::MBC2Battery),
            0x08 => Ok(CartridgeType::RomRam),
            0x09 => Ok(CartridgeType::RomRamBattery),
            0x0B => Ok(CartridgeType::MMM01),
            0x0C => Ok(CartridgeType::MMM01Ram),
            0x0D => Ok(CartridgeType::MMM01RamBattery),
            0x0F => Ok(CartridgeType::MBC3TimerBattery),
            0x10 => Ok(CartridgeType::MBC3TimerRamBattery),
            0x11 => Ok(CartridgeType::MBC3),
            0x12 => Ok(CartridgeType::MBC3Ram),
            0x13 => Ok(CartridgeType::MBC3RamBattery),
            0x19 => Ok(CartridgeType::MBC5),
            0x1A => Ok(CartridgeType::MBC5Ram),
            0x1B => Ok(CartridgeType::MBC5RamBattery),
            0x1C => Ok(CartridgeType::MBC5Rumble),
            0x1D => Ok(CartridgeType::MBC5RumbleRam),
            0x1E => Ok(CartridgeType::MBC5RumbleRamBattery),
            0x20 => Ok(CartridgeType::MBC6),
            0x22 => Ok(CartridgeType::MBC7SensorRumbleRamBattery),
            0xFC => Ok(CartridgeType::PocketCamera),
            0xFD => Ok(CartridgeType::BandaiTama5),
            0xFE => Ok(CartridgeType::HuC3),
            0xFF => Ok(CartridgeType::HuC1RamBattery),
            _ => Err(CoreError::UnknownCartridgeType(value)),
        }
    }
}
