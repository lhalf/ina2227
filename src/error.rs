#[derive(Debug, PartialEq)]
pub enum Error<E> {
    I2c(E),
    InvalidManufacturerId,
    InvalidDeviceId,
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2c(error)
    }
}
