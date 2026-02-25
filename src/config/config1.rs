#[derive(Debug, Default)]
pub struct Config1 {
    pub active_channel: ACTIVE_CHANNEL,
    pub avg: AVG,
    pub vbusct: VBUSCT,
    pub vshct: VSHCT,
    pub mode: MODE,
}

impl Config1 {
    pub fn to_u16(&self) -> u16 {
        ((self.active_channel as u16) << 12)
            | ((self.avg as u16) << 9)
            | ((self.vbusct as u16) << 6)
            | ((self.vshct as u16) << 3)
            | (self.mode as u16)
    }
}

/// These 4 bits determine which channels are active. Set this bit to '1' to enable each channel.
/// Disabled channels are skipped in the round robin cycle.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ACTIVE_CHANNEL {
    Channel1 = 0b0010,
    Channel2 = 0b0001,
    AllChannels = 0b0011,
    #[default]
    AllChannelsWithReserved = 0b1111,
}

/// Sets the number of ADC conversion results to be averaged. The read-back registers are updated after averaging is completed.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum AVG {
    #[default]
    _1 = 0b000,
    _4 = 0b001,
    _16 = 0b010,
    _64 = 0b011,
    _128 = 0b100,
    _256 = 0b101,
    _512 = 0b110,
    _1024 = 0b111,
}

/// Sets the conversion time of the VBUS measurement
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum VBUSCT {
    _140us = 0b000,
    _204us = 0b001,
    _332us = 0b010,
    _588us = 0b011,
    #[default]
    _1100us = 0b100,
    _2116us = 0b101,
    _4156us = 0b110,
    _8244us = 0b111,
}

/// Sets the conversion time of the SHUNT measurement
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum VSHCT {
    _140us = 0b000,
    _204us = 0b001,
    _332us = 0b010,
    _588us = 0b011,
    #[default]
    _1100us = 0b100,
    _2116us = 0b101,
    _4156us = 0b110,
    _8244us = 0b111,
}

/// Operating mode: Modes can be selected to operate the device either in Shutdown mode, continuous mode or triggered mode.
/// The mode also allows user to select mux settings to set continuous or triggered mode on bus voltage and/or shunt voltage measurements.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum MODE {
    Shutdown = 0b000,
    ShuntVoltageTriggered = 0b001,
    BusVoltageTriggered = 0b010,
    ShuntBusVoltageTriggered = 0b011,
    Shutdown2 = 0b100,
    ShuntVoltageContinuous = 0b101,
    BusVoltageContinuous = 0b110,
    #[default]
    ShuntBusVoltageContinuous = 0b111,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unusual_byte_groupings)]
    use super::*;

    #[test]
    fn config1_to_u16() {
        let config = Config1 {
            active_channel: ACTIVE_CHANNEL::AllChannels,
            avg: AVG::_64,
            vbusct: VBUSCT::_1100us,
            vshct: VSHCT::_1100us,
            mode: MODE::ShuntBusVoltageContinuous,
        };

        assert_eq!(config.to_u16(), 0b0011_011_100_100_111);
    }

    #[test]
    fn config1_default_matches_datasheet() {
        let config = Config1::default();
        assert_eq!(config.to_u16(), 0xF127);
    }
}
