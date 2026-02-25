#[repr(u8)]
pub enum Register {
    Limit1 = 0x06,
    Alert1 = 0x07,
    Limit2 = 0x0E,
    Alert2 = 0x0F,
    Config1 = 0x10,
    Config2 = 0x11,
    ManufacturerID = 0x7E,
    DeviceID = 0x7F,
}
