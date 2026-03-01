#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Calibration {
    pub shunt_calibration: u16,
}

impl Calibration {
    pub fn to_u16(self) -> u16 {
        self.shunt_calibration & 0x7FFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_bit_is_masked() {
        let calibration = Calibration {
            shunt_calibration: 0xFFFF,
        };
        assert_eq!(calibration.to_u16(), 0x7FFF);
    }

    #[test]
    fn zero_value() {
        let calibration = Calibration {
            shunt_calibration: 0,
        };
        assert_eq!(calibration.to_u16(), 0x0000);
    }

    #[test]
    fn datasheet_example_value() {
        let calibration = Calibration {
            shunt_calibration: 1280,
        };
        assert_eq!(calibration.to_u16(), 0x0500);
    }

    #[test]
    fn max_valid_value() {
        let calibration = Calibration {
            shunt_calibration: 0x7FFF,
        };
        assert_eq!(calibration.to_u16(), 0x7FFF);
    }
}
