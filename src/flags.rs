#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Flags {
    pub limit2_alert: bool,
    pub limit1_alert: bool,
    pub energy_overflow_channel2: bool,
    pub energy_overflow_channel1: bool,
    pub conversion_ready: bool,
    pub math_overflow: bool,
}

impl Flags {
    pub fn from_u16(value: u16) -> Self {
        Self {
            limit2_alert: (value >> 13) & 1 != 0,
            limit1_alert: (value >> 12) & 1 != 0,
            energy_overflow_channel2: (value >> 9) & 1 != 0,
            energy_overflow_channel1: (value >> 8) & 1 != 0,
            conversion_ready: (value >> 7) & 1 != 0,
            math_overflow: (value >> 6) & 1 != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_flags_clear() {
        let flags = Flags::from_u16(0x0000);
        assert!(!flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn limit2_alert_set() {
        let flags = Flags::from_u16(1 << 13);
        assert!(flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn limit1_alert_set() {
        let flags = Flags::from_u16(1 << 12);
        assert!(!flags.limit2_alert);
        assert!(flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn energy_overflow_channel2_set() {
        let flags = Flags::from_u16(1 << 9);
        assert!(!flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn energy_overflow_channel1_set() {
        let flags = Flags::from_u16(1 << 8);
        assert!(!flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn conversion_ready_set() {
        let flags = Flags::from_u16(1 << 7);
        assert!(!flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn math_overflow_set() {
        let flags = Flags::from_u16(1 << 6);
        assert!(!flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(flags.math_overflow);
    }

    #[test]
    fn reserved_bits_ignored() {
        let flags = Flags::from_u16(0b1100_1100_0011_1111);
        assert!(!flags.limit2_alert);
        assert!(!flags.limit1_alert);
        assert!(!flags.energy_overflow_channel2);
        assert!(!flags.energy_overflow_channel1);
        assert!(!flags.conversion_ready);
        assert!(!flags.math_overflow);
    }

    #[test]
    fn all_flags_set() {
        let value = (1 << 13) | (1 << 12) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6);
        let flags = Flags::from_u16(value);
        assert!(flags.limit2_alert);
        assert!(flags.limit1_alert);
        assert!(flags.energy_overflow_channel2);
        assert!(flags.energy_overflow_channel1);
        assert!(flags.conversion_ready);
        assert!(flags.math_overflow);
    }
}
