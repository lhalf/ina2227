use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c;
use ina2227::{Flags, INA2227};

const DEFAULT_ADDRESS: u8 = 0x40;

#[test]
fn flags_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x12], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.flags().unwrap_err());

    i2c.done();
}

#[test]
fn flags_all_clear() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x00, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(!flags.limit2_alert);
    assert!(!flags.limit1_alert);
    assert!(!flags.energy_overflow_channel2);
    assert!(!flags.energy_overflow_channel1);
    assert!(!flags.conversion_ready);
    assert!(!flags.math_overflow);

    i2c.done();
}

#[test]
fn flags_conversion_ready() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x00, 0x80],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.conversion_ready);
    assert!(!flags.math_overflow);
    assert!(!flags.limit1_alert);
    assert!(!flags.limit2_alert);
    assert!(!flags.energy_overflow_channel1);
    assert!(!flags.energy_overflow_channel2);

    i2c.done();
}

#[test]
fn flags_math_overflow() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x00, 0x40],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.math_overflow);
    assert!(!flags.conversion_ready);

    i2c.done();
}

#[test]
fn flags_limit1_alert() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x10, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.limit1_alert);
    assert!(!flags.limit2_alert);

    i2c.done();
}

#[test]
fn flags_limit2_alert() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x20, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.limit2_alert);
    assert!(!flags.limit1_alert);

    i2c.done();
}

#[test]
fn flags_energy_overflow_channel1() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x01, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.energy_overflow_channel1);
    assert!(!flags.energy_overflow_channel2);

    i2c.done();
}

#[test]
fn flags_energy_overflow_channel2() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x02, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.energy_overflow_channel2);
    assert!(!flags.energy_overflow_channel1);

    i2c.done();
}

#[test]
fn flags_all_set() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x33, 0xC0],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags = ina2227.flags().unwrap();
    assert!(flags.limit2_alert);
    assert!(flags.limit1_alert);
    assert!(flags.energy_overflow_channel2);
    assert!(flags.energy_overflow_channel1);
    assert!(flags.conversion_ready);
    assert!(flags.math_overflow);

    i2c.done();
}

#[test]
fn flags_returns_correct_type() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x12],
        vec![0x00, 0x80],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    let flags: Flags = ina2227.flags().unwrap();
    assert!(flags.conversion_ready);

    i2c.done();
}
