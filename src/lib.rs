#![cfg_attr(not(test), no_std)]

mod error;
mod register;

use crate::error::Error;
use crate::register::Register;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

const MANUFACTURE_ID: u16 = 0x5449;
const DEVICE_ID: u16 = 0x2350;

#[derive(Debug)]
pub struct INA2227<I2C> {
    i2c: I2C,
    address: u8,
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "INA2227"),
    async(feature = "async", keep_self)
)]
impl<I2C, E> INA2227<I2C>
where
    I2C: I2c<Error = E>,
{
    pub async fn try_new(i2c: I2C, address: u8) -> Result<Self, Error<E>> {
        let mut ina2227 = Self { i2c, address };

        if ina2227.read_u16(Register::ManufacturerID).await? != MANUFACTURE_ID {
            return Err(Error::InvalidManufacturerId);
        }

        if ina2227.read_u16(Register::DeviceID).await? != DEVICE_ID {
            return Err(Error::InvalidDeviceId);
        }

        Ok(ina2227)
    }

    #[inline(always)]
    async fn read_u16(&mut self, register: Register) -> Result<u16, Error<E>> {
        let mut buffer = [0u8; 2];

        self.i2c
            .write_read(self.address, &[register as u8], &mut buffer)
            .await?;

        Ok(u16::from_be_bytes(buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c;

    const DEFAULT_ADDRESS: u8 = 0x40;

    #[test]
    fn failure_to_read_manufacture_id_causes_error() {
        let expectations = [i2c::Transaction::write_read(
            DEFAULT_ADDRESS,
            vec![Register::ManufacturerID as u8],
            vec![0x00, 0x00],
        )
        .with_error(ErrorKind::Other)];
        let mut i2c = i2c::Mock::new(&expectations);

        assert_eq!(
            Error::I2c(ErrorKind::Other),
            INA2227::try_new(i2c.clone(), DEFAULT_ADDRESS).unwrap_err()
        );

        i2c.done();
    }
}
