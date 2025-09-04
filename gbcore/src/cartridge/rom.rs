use crate::CoreError;

#[derive(Debug)]
pub enum RomType {
    RomBank32KByte,
    RomBank64KByte,
    RomBank128KByte,
    RomBank256KByte,
    RomBank512KByte,
    RomBank1MByte,
    RomBank2MByte,
    RomBank4MByte,
    RomBank8MByte,
    RomBank1_1MByte,
    RomBank1_2MByte,
    RomBank1_5MByte,
}

impl RomType {
    pub fn nb_bank(&self) -> usize {
        match self {
            RomType::RomBank32KByte => 2,
            RomType::RomBank64KByte => 4,
            RomType::RomBank128KByte => 8,
            RomType::RomBank256KByte => 16,
            RomType::RomBank512KByte => 32,
            RomType::RomBank1MByte => 64,
            RomType::RomBank2MByte => 128,
            RomType::RomBank4MByte => 256,
            RomType::RomBank8MByte => 512,
            RomType::RomBank1_1MByte => 72,
            RomType::RomBank1_2MByte => 80,
            RomType::RomBank1_5MByte => 96,
        }
    }

    pub fn memory_size(&self) -> usize {
        self.nb_bank() * 16
    }
}

impl TryFrom<u8> for RomType {
    type Error = CoreError;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RomType::RomBank32KByte),
            0x01 => Ok(RomType::RomBank64KByte),
            0x02 => Ok(RomType::RomBank128KByte),
            0x03 => Ok(RomType::RomBank256KByte),
            0x04 => Ok(RomType::RomBank512KByte),
            0x05 => Ok(RomType::RomBank1MByte),
            0x06 => Ok(RomType::RomBank2MByte),
            0x07 => Ok(RomType::RomBank4MByte),
            0x08 => Ok(RomType::RomBank8MByte),
            0x52 => Ok(RomType::RomBank1_1MByte),
            0x53 => Ok(RomType::RomBank1_2MByte),
            0x54 => Ok(RomType::RomBank1_5MByte),
            _ => Err(CoreError::UnknownRomType(value)),
        }
    }
}
