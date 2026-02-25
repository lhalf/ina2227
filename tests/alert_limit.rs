use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c;
use ina2227::{AlertLimit, INA2227};

const DEFAULT_ADDRESS: u8 = 0b0100_0000;

#[test]
fn set_limit1_write_failure() {
    let expectations =
        [
            i2c::Transaction::write(DEFAULT_ADDRESS, vec![0b0000_0110, 0b1111_1111, 0b1111_1111])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227
            .set_limit1(&AlertLimit::ShuntVoltage(-1))
            .unwrap_err()
    );

    i2c.done();
}

#[test]
fn set_limit2_write_failure() {
    let expectations =
        [
            i2c::Transaction::write(DEFAULT_ADDRESS, vec![0b0000_1110, 0b0111_1111, 0b1111_1111])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);

    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227
            .set_limit2(&AlertLimit::BusVoltage(0b1111_1111_1111_1111))
            .unwrap_err()
    );

    i2c.done();
}
