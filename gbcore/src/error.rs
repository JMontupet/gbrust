#[derive(Debug)]
pub enum CoreError {
    UnknownRamType(u8),
    UnknownRomType(u8),
    UnknownCartridgeType(u8),
    UnknownOpCode(u8),
    UnknownOpCodeCB(u8),
    UnknownCPUState(u16, u16),
    UnknownGPULY(u8),
}
