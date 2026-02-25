#![cfg_attr(not(test), no_std)]

mod register;

use crate::register::Register;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

pub const MANUFACTURE_ID: u16 = 0x5449;
pub const DEVICE_ID: u16 = 0x2350;

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
    pub async fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    #[inline(always)]
    pub async fn manufacturer_id(&mut self) -> Result<u16, E> {
        self.read_u16(Register::ManufacturerID).await
    }

    #[inline(always)]
    pub async fn is_manufacturer_id_ok(&mut self) -> bool {
        self.manufacturer_id()
            .await
            .is_ok_and(|id| id == MANUFACTURE_ID)
    }

    #[inline(always)]
    pub async fn device_id(&mut self) -> Result<u16, E> {
        self.read_u16(Register::DeviceID).await
    }

    #[inline(always)]
    pub async fn is_device_id_ok(&mut self) -> bool {
        self.device_id().await.is_ok_and(|id| id == DEVICE_ID)
    }

    #[inline(always)]
    async fn read_u16(&mut self, register: Register) -> Result<u16, E> {
        let mut buffer = [0u8; 2];

        self.i2c
            .write_read(self.address, &[register as u8], &mut buffer)
            .await?;

        Ok(u16::from_be_bytes(buffer))
    }
}
