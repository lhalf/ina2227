#[repr(u8)]
pub enum Register {
    Config1 = 0x10,
    Config2 = 0x11,
    ManufacturerID = 0x7E,
    DeviceID = 0x7F,
}
