use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c;
use ina2227::{Calibration, INA2227};

const DEFAULT_ADDRESS: u8 = 0x40;

#[test]
fn set_calibration_channel1_success() {
    let expectations = [i2c::Transaction::write(
        DEFAULT_ADDRESS,
        vec![0x05, 0x05, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    ina2227
        .set_calibration_channel1(&Calibration {
            shunt_calibration: 1280,
        })
        .unwrap();

    i2c.done();
}

#[test]
fn set_calibration_channel1_zero() {
    let expectations = [i2c::Transaction::write(
        DEFAULT_ADDRESS,
        vec![0x05, 0x00, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    ina2227
        .set_calibration_channel1(&Calibration {
            shunt_calibration: 0,
        })
        .unwrap();

    i2c.done();
}

#[test]
fn set_calibration_channel1_reserved_bit_masked() {
    let expectations = [i2c::Transaction::write(
        DEFAULT_ADDRESS,
        vec![0x05, 0x7F, 0xFF],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    ina2227
        .set_calibration_channel1(&Calibration {
            shunt_calibration: 0xFFFF,
        })
        .unwrap();

    i2c.done();
}

#[test]
fn set_calibration_channel1_write_failure() {
    let expectations = [
        i2c::Transaction::write(DEFAULT_ADDRESS, vec![0x05, 0x05, 0x00])
            .with_error(ErrorKind::Other),
    ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227
            .set_calibration_channel1(&Calibration {
                shunt_calibration: 1280
            })
            .unwrap_err()
    );

    i2c.done();
}

#[test]
fn set_calibration_channel2_success() {
    let expectations = [i2c::Transaction::write(
        DEFAULT_ADDRESS,
        vec![0x0D, 0x05, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    ina2227
        .set_calibration_channel2(&Calibration {
            shunt_calibration: 1280,
        })
        .unwrap();

    i2c.done();
}

#[test]
fn set_calibration_channel2_write_failure() {
    let expectations = [
        i2c::Transaction::write(DEFAULT_ADDRESS, vec![0x0D, 0x05, 0x00])
            .with_error(ErrorKind::Other),
    ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227
            .set_calibration_channel2(&Calibration {
                shunt_calibration: 1280
            })
            .unwrap_err()
    );

    i2c.done();
}
