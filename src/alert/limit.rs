#[derive(Debug, Copy, Clone)]
pub enum AlertLimit {
    ShuntVoltage(i16),
    BusVoltage(u16),
    Power(u16),
}

impl AlertLimit {
    pub fn to_u16(&self) -> u16 {
        match self {
            AlertLimit::ShuntVoltage(voltage) => *voltage as u16,
            AlertLimit::BusVoltage(voltage) => voltage & 0b0111_1111_1111_1111,
            AlertLimit::Power(power) => *power,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shunt_voltage_positive() {
        assert_eq!(
            AlertLimit::ShuntVoltage(0b0111_1111_1111_1111).to_u16(),
            0b0111_1111_1111_1111
        );
    }

    #[test]
    fn shunt_voltage_negative() {
        assert_eq!(AlertLimit::ShuntVoltage(-1).to_u16(), 0b1111_1111_1111_1111);
    }

    #[test]
    fn bus_voltage_masks_msb() {
        assert_eq!(
            AlertLimit::BusVoltage(0b1111_1111_1111_1111).to_u16(),
            0b0111_1111_1111_1111
        );
    }

    #[test]
    fn bus_voltage_normal() {
        assert_eq!(
            AlertLimit::BusVoltage(0b0001_0010_0011_0100).to_u16(),
            0b0001_0010_0011_0100
        );
    }

    #[test]
    fn power_full_range() {
        assert_eq!(
            AlertLimit::Power(0b1111_1111_1111_1111).to_u16(),
            0b1111_1111_1111_1111
        );
    }

    #[test]
    fn reset_value() {
        assert_eq!(AlertLimit::ShuntVoltage(0).to_u16(), 0b0000_0000_0000_0000);
    }
}
