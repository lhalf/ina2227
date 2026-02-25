use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c;
use ina2227::{Config1, Config2, INA2227};

const DEFAULT_ADDRESS: u8 = 0x40;

#[test]
fn set_config1_write_failure() {
    let expectations = [
        i2c::Transaction::write(DEFAULT_ADDRESS, vec![0x10, 0xF1, 0x27])
            .with_error(ErrorKind::Other),
    ];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227.set_config1(&Config1::default()).unwrap_err()
    );

    i2c.done();
}

#[test]
fn set_config2_write_failure() {
    let expectations = [
        i2c::Transaction::write(DEFAULT_ADDRESS, vec![0x11, 0x00, 0x00])
            .with_error(ErrorKind::Other),
    ];

    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227.set_config2(&Config2::default()).unwrap_err()
    );

    i2c.done();
}
