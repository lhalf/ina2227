use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c;
use ina2227::INA2227;

const DEFAULT_ADDRESS: u8 = 0x40;

#[test]
fn shunt_voltage_channel1_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x00],
        vec![0x4B, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.shunt_voltage_channel1().unwrap(), 19200i16);

    i2c.done();
}

#[test]
fn shunt_voltage_channel1_negative() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x00],
        vec![0x83, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.shunt_voltage_channel1().unwrap(), -32000i16);

    i2c.done();
}

#[test]
fn shunt_voltage_channel1_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x00], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227.shunt_voltage_channel1().unwrap_err()
    );

    i2c.done();
}

#[test]
fn shunt_voltage_channel2_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x08],
        vec![0x4B, 0x00],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.shunt_voltage_channel2().unwrap(), 19200i16);

    i2c.done();
}

#[test]
fn shunt_voltage_channel2_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x08], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227.shunt_voltage_channel2().unwrap_err()
    );

    i2c.done();
}

#[test]
fn bus_voltage_channel1_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x01],
        vec![0x1D, 0x4C],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.bus_voltage_channel1().unwrap(), 7500u16);

    i2c.done();
}

#[test]
fn bus_voltage_channel1_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x01], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227.bus_voltage_channel1().unwrap_err()
    );

    i2c.done();
}

#[test]
fn bus_voltage_channel2_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x09],
        vec![0x1D, 0x4C],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.bus_voltage_channel2().unwrap(), 7500u16);

    i2c.done();
}

#[test]
fn bus_voltage_channel2_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x09], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(
        ErrorKind::Other,
        ina2227.bus_voltage_channel2().unwrap_err()
    );

    i2c.done();
}

#[test]
fn current_channel1_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x02],
        vec![0x2E, 0xE0],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.current_channel1().unwrap(), 12000i16);

    i2c.done();
}

#[test]
fn current_channel1_negative() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x02],
        vec![0xD1, 0x20],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.current_channel1().unwrap(), -12000i16);

    i2c.done();
}

#[test]
fn current_channel1_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x02], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.current_channel1().unwrap_err());

    i2c.done();
}

#[test]
fn current_channel2_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x0A],
        vec![0x2E, 0xE0],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.current_channel2().unwrap(), 12000i16);

    i2c.done();
}

#[test]
fn current_channel2_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x0A], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.current_channel2().unwrap_err());

    i2c.done();
}

#[test]
fn power_channel1_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x03],
        vec![0x11, 0x94],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.power_channel1().unwrap(), 4500u16);

    i2c.done();
}

#[test]
fn power_channel1_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x03], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.power_channel1().unwrap_err());

    i2c.done();
}

#[test]
fn power_channel2_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x0B],
        vec![0x11, 0x94],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.power_channel2().unwrap(), 4500u16);

    i2c.done();
}

#[test]
fn power_channel2_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x0B], vec![0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.power_channel2().unwrap_err());

    i2c.done();
}

#[test]
fn energy_channel1_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x04],
        vec![0x00, 0xF7, 0x31, 0x40],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.energy_channel1().unwrap(), 16_200_000u32);

    i2c.done();
}

#[test]
fn energy_channel1_max_value() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x04],
        vec![0xFF, 0xFF, 0xFF, 0xFF],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.energy_channel1().unwrap(), u32::MAX);

    i2c.done();
}

#[test]
fn energy_channel1_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x04], vec![0x00, 0x00, 0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.energy_channel1().unwrap_err());

    i2c.done();
}

#[test]
fn energy_channel2_success() {
    let expectations = [i2c::Transaction::write_read(
        DEFAULT_ADDRESS,
        vec![0x0C],
        vec![0x00, 0xF7, 0x31, 0x40],
    )];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ina2227.energy_channel2().unwrap(), 16_200_000u32);

    i2c.done();
}

#[test]
fn energy_channel2_read_failure() {
    let expectations =
        [
            i2c::Transaction::write_read(DEFAULT_ADDRESS, vec![0x0C], vec![0x00, 0x00, 0x00, 0x00])
                .with_error(ErrorKind::Other),
        ];
    let mut i2c = i2c::Mock::new(&expectations);
    let mut ina2227 = INA2227::new(i2c.clone(), DEFAULT_ADDRESS);

    assert_eq!(ErrorKind::Other, ina2227.energy_channel2().unwrap_err());

    i2c.done();
}
