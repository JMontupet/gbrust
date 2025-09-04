use crate::CoreError;

#[derive(Debug, Clone)]
pub enum RamType {
    RamBankNoRam,
    RamBankUnused,
    RamBank8KByte,
    RamBank32KByte,
    RamBank128KByte,
    RamBank64KByte,
}

impl RamType {
    pub fn nb_bank(&self) -> usize {
        match self {
            RamType::RamBankNoRam => 0,
            RamType::RamBankUnused => 0,
            RamType::RamBank8KByte => 1,
            RamType::RamBank32KByte => 4,
            RamType::RamBank128KByte => 16,
            RamType::RamBank64KByte => 68,
        }
    }

    pub fn memory_size(&self) -> usize {
        self.nb_bank() * 8
    }
}

impl TryFrom<u8> for RamType {
    type Error = CoreError;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RamType::RamBankNoRam),
            0x01 => Ok(RamType::RamBankUnused),
            0x02 => Ok(RamType::RamBank8KByte),
            0x03 => Ok(RamType::RamBank32KByte),
            0x04 => Ok(RamType::RamBank128KByte),
            0x05 => Ok(RamType::RamBank64KByte),
            _ => Err(CoreError::UnknownRamType(value)),
        }
    }
}
