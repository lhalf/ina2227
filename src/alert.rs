#[derive(Debug, Default)]
pub struct Alert {
    pub channel: CHANNEL,
    pub alert_mask: ALERT_MASK,
}

impl Alert {
    pub fn to_u16(&self) -> u16 {
        ((self.channel as u16) << 3) | (self.alert_mask as u16)
    }
}

/// Selects which channel this alert is assigned to.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum CHANNEL {
    #[default]
    Channel1 = 0b00,
    Channel2 = 0b01,
}

/// Sets the active alert for the assigned channel.
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ALERT_MASK {
    #[default]
    Disabled = 0b000,
    ShuntVoltageOverLimit = 0b001,
    ShuntVoltageUnderLimit = 0b010,
    BusVoltageOverLimit = 0b011,
    BusVoltageUnderLimit = 0b100,
    PowerOverLimit = 0b101,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unusual_byte_groupings)]
    use super::*;

    #[test]
    fn alert_to_u16() {
        let alert = Alert {
            channel: CHANNEL::Channel2,
            alert_mask: ALERT_MASK::PowerOverLimit,
        };

        assert_eq!(alert.to_u16(), 0b00000_01_101);
    }

    #[test]
    fn alert_default_matches_datasheet() {
        let alert = Alert::default();
        assert_eq!(alert.to_u16(), 0x0000);
    }
}
