#[derive(Debug, Default)]
pub struct Config2 {
    pub rst: RST,
    pub acc_rst: ACC_RST,
    pub cnvr_mask: CNVR_MASK,
    pub enof_mask: ENOF_MASK,
    pub alert_latch: ALERT_LATCH,
    pub alert_pol: ALERT_POL,
    pub range: RANGE,
}

impl Config2 {
    pub fn to_u16(&self) -> u16 {
        ((self.rst as u16) << 15)
            | ((self.acc_rst as u16) << 8)
            | ((self.cnvr_mask as u16) << 7)
            | ((self.enof_mask as u16) << 6)
            | ((self.alert_latch as u16) << 5)
            | ((self.alert_pol as u16) << 4)
            | (self.range as u16)
    }
}

/// Set this bit to '1' to generate a system reset that is the same as power-on reset.
/// Resets all registers to default values and then self-clears.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RST {
    #[default]
    Normal = 0b0,
    Reset = 0b1,
}

/// Writing a one to these bits resets the energy registers and clears any overflow flags.
/// Bit11 = reserved.
/// Bit10 = reserved.
/// Bit9 = Channel 2 energy reset, overflow clear.
/// Bit8 = Channel 1 energy reset, overflow clear.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ACC_RST {
    #[default]
    AllChannelsActive = 0b0000,
    Channel1Reset = 0b0001,
    Channel2Reset = 0b0010,
    AllChannelsReset = 0b0011,
}

/// Setting this bit high configures the ALERT pin to be asserted when conversions are complete.
/// ALERT remains asserted until the CVRF field in the flags register is read.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum CNVR_MASK {
    #[default]
    Disable = 0b0,
    Enable = 0b1,
}

/// When set to 1, the Alert pin toggles when an energy overflow condition occurs on any of the enabled channels
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ENOF_MASK {
    #[default]
    Disable = 0b0,
    Enable = 0b1,
}

/// When set to 1 the state of the Alert pin latches during fault conditions.
/// To clear the alert the alert flags register must be read and the fault condition removed.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ALERT_LATCH {
    #[default]
    Transparent = 0b0,
    Latched = 0b1,
}

/// When this bit is set to 1, the alert pin toggles from low to high during a fault condition.
/// When set to 0 (default), the alert pin toggles from high to low during faults.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ALERT_POL {
    #[default]
    ActiveLow = 0b0,
    ActiveHigh = 0b1,
}

/// Enables the selection of the shunt full scale input range for each channel.
/// Bit3 = reserved.
/// Bit2 = reserved.
/// Bit1 = Channel 2 range selection.
/// Bit0 = Channel 1 range selection.
/// range selection bit = 0 selects ±81.92mV
/// range selection bit = 1 selects ±20.48mV
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RANGE {
    #[default]
    AllChannels81_92mV = 0b0000,
    Channel1_20_48mV = 0b0001,
    Channel2_20_48mV = 0b0010,
    AllChannels20_48mV = 0b0011,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config2_to_u16() {
        let config = Config2 {
            rst: RST::Normal,
            acc_rst: ACC_RST::AllChannelsActive,
            cnvr_mask: CNVR_MASK::Disable,
            enof_mask: ENOF_MASK::Disable,
            alert_latch: ALERT_LATCH::Transparent,
            alert_pol: ALERT_POL::ActiveLow,
            range: RANGE::AllChannels81_92mV,
        };

        assert_eq!(config.to_u16(), 0b0000_0000_0000_0000);
    }

    #[test]
    fn config2_default_matches_datasheet() {
        let config = Config2::default();
        assert_eq!(config.to_u16(), 0x0000);
    }
}
