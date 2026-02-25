use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c;
use ina2227::INA2227;

const DEFAULT_ADDRESS: u8 = 0x40;

#[test]
fn manufacturer_id_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x7E], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.manufacturer_id().unwrap_err());

    i2c.done();
}

#[test]
fn device_id_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x7F], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.device_id().unwrap_err());

    i2c.done();
}

#[test]
fn manufacturer_id_invalid() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x7E],
        vec![0x00, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert!(!ina2227.is_manufacturer_id_ok());

    i2c.done();
}

#[test]
fn device_id_invalid() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x7F],
        vec![0x00, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert!(!ina2227.is_device_id_ok());

    i2c.done();
}
