#![cfg_attr(not(test), no_std)]

mod alert;
mod calibration;
mod config;
mod flags;
mod register;

pub use crate::alert::{AlertConfig, AlertLimit};
pub use crate::calibration::Calibration;
pub use crate::config::{Config1, Config2};
pub use crate::flags::Flags;
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
    pub async fn set_config1(&mut self, config1: &Config1) -> Result<(), E> {
        self.write_u16(Register::Config1, config1.to_u16()).await
    }

    #[inline(always)]
    pub async fn set_config2(&mut self, config2: &Config2) -> Result<(), E> {
        self.write_u16(Register::Config2, config2.to_u16()).await
    }

    #[inline(always)]
    pub async fn set_alert1(&mut self, alert: &AlertConfig) -> Result<(), E> {
        self.write_u16(Register::Alert1, alert.to_u16()).await
    }

    #[inline(always)]
    pub async fn set_alert2(&mut self, alert: &AlertConfig) -> Result<(), E> {
        self.write_u16(Register::Alert2, alert.to_u16()).await
    }

    #[inline(always)]
    pub async fn set_limit1(&mut self, limit: &AlertLimit) -> Result<(), E> {
        self.write_u16(Register::Limit1, limit.to_u16()).await
    }

    #[inline(always)]
    pub async fn set_limit2(&mut self, limit: &AlertLimit) -> Result<(), E> {
        self.write_u16(Register::Limit2, limit.to_u16()).await
    }

    #[inline(always)]
    pub async fn set_calibration_channel1(&mut self, calibration: &Calibration) -> Result<(), E> {
        self.write_u16(Register::CalibrationChannel1, calibration.to_u16())
            .await
    }

    #[inline(always)]
    pub async fn set_calibration_channel2(&mut self, calibration: &Calibration) -> Result<(), E> {
        self.write_u16(Register::CalibrationChannel2, calibration.to_u16())
            .await
    }

    #[inline(always)]
    pub async fn shunt_voltage_channel1(&mut self) -> Result<i16, E> {
        self.read_u16(Register::ShuntVoltageChannel1)
            .await
            .map(|v| v as i16)
    }

    #[inline(always)]
    pub async fn shunt_voltage_channel2(&mut self) -> Result<i16, E> {
        self.read_u16(Register::ShuntVoltageChannel2)
            .await
            .map(|v| v as i16)
    }

    #[inline(always)]
    pub async fn bus_voltage_channel1(&mut self) -> Result<u16, E> {
        self.read_u16(Register::BusVoltageChannel1).await
    }

    #[inline(always)]
    pub async fn bus_voltage_channel2(&mut self) -> Result<u16, E> {
        self.read_u16(Register::BusVoltageChannel2).await
    }

    #[inline(always)]
    pub async fn current_channel1(&mut self) -> Result<i16, E> {
        self.read_u16(Register::CurrentChannel1)
            .await
            .map(|v| v as i16)
    }

    #[inline(always)]
    pub async fn current_channel2(&mut self) -> Result<i16, E> {
        self.read_u16(Register::CurrentChannel2)
            .await
            .map(|v| v as i16)
    }

    #[inline(always)]
    pub async fn power_channel1(&mut self) -> Result<u16, E> {
        self.read_u16(Register::PowerChannel1).await
    }

    #[inline(always)]
    pub async fn power_channel2(&mut self) -> Result<u16, E> {
        self.read_u16(Register::PowerChannel2).await
    }

    #[inline(always)]
    pub async fn energy_channel1(&mut self) -> Result<u32, E> {
        self.read_u32(Register::EnergyChannel1).await
    }

    #[inline(always)]
    pub async fn energy_channel2(&mut self) -> Result<u32, E> {
        self.read_u32(Register::EnergyChannel2).await
    }

    #[inline(always)]
    pub async fn flags(&mut self) -> Result<Flags, E> {
        self.read_u16(Register::Flags).await.map(Flags::from_u16)
    }

    #[inline(always)]
    async fn read_u16(&mut self, register: Register) -> Result<u16, E> {
        let mut buffer = [0u8; 2];

        self.i2c
            .write_read(self.address, &[register as u8], &mut buffer)
            .await?;

        Ok(u16::from_be_bytes(buffer))
    }

    #[inline(always)]
    async fn read_u32(&mut self, register: Register) -> Result<u32, E> {
        let mut buffer = [0u8; 4];

        self.i2c
            .write_read(self.address, &[register as u8], &mut buffer)
            .await?;

        Ok(u32::from_be_bytes(buffer))
    }

    #[inline(always)]
    async fn write_u16(&mut self, register: Register, value: u16) -> Result<(), E> {
        self.i2c
            .write(
                self.address,
                &[register as u8, (value >> 8) as u8, value as u8],
            )
            .await
    }
}
