extern crate eeprom24x;
use eeprom24x::{Eeprom24x, SlaveAddr, ic, Error};
extern crate embedded_hal_mock as hal;

fn setup<'a>() -> Eeprom24x<hal::I2cMock<'a>, ic::IC24x256> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&[0xAB, 0xCD, 0xEF]);
    Eeprom24x::new_24x256(dev, SlaveAddr::default())
}

fn check_sent_data(eeprom: Eeprom24x<hal::I2cMock, ic::IC24x256>, data: &[u8]) {
    let dev = eeprom.destroy();
    assert_eq!(dev.get_last_address(), Some(0x50));
    assert_eq!(dev.get_write_data(), &data[..]);
}

#[test]
fn can_create_24x32() {
    let dev = hal::I2cMock::new();
    Eeprom24x::new_24x32(dev, SlaveAddr::default());
}

#[test]
fn can_create_24x64() {
    let dev = hal::I2cMock::new();
    Eeprom24x::new_24x64(dev, SlaveAddr::default());
}

#[test]
fn can_create_24x128() {
    let dev = hal::I2cMock::new();
    Eeprom24x::new_24x128(dev, SlaveAddr::default());
}

#[test]
fn can_create_24x256() {
    let dev = hal::I2cMock::new();
    Eeprom24x::new_24x256(dev, SlaveAddr::default());
}

#[test]
fn can_create_24x512() {
    let dev = hal::I2cMock::new();
    Eeprom24x::new_24x512(dev, SlaveAddr::default());
}

#[test]
fn sends_correct_parameters_for_byte_read() {
    let mut eeprom = setup();
    let address = [0x12, 0x34];
    eeprom.read_byte(&address).unwrap();
    check_sent_data(eeprom, &address);
}

#[test]
fn can_read_byte() {
    let mut eeprom = setup();
    let data = eeprom.read_byte(&[0, 0]).unwrap();
    assert_eq!(data, 0xAB);
}

#[test]
fn can_read_array() {
    let mut eeprom = setup();
    let mut data = [0; 3];
    eeprom.read_data(&[0, 0], &mut data).unwrap();
    assert_eq!(data, [0xAB, 0xCD, 0xEF]);
}

#[test]
fn sends_correct_parameters_for_byte_write() {
    let mut eeprom = setup();
    let address = [0x12, 0x34];
    let data = 0xCD;
    eeprom.write_byte(&address, data).unwrap();
    check_sent_data(eeprom, &[address[0], address[1], data]);
}

#[test]
fn write_empty_data_does_nothing() {
    let mut eeprom = setup();
    eeprom.write_page(&[0, 0], &[]).unwrap();
    let dev = eeprom.destroy();
    assert_eq!(dev.get_last_address(), None);
    assert_eq!(dev.get_write_data(), &[]);
}

#[test]
fn cannot_write_too_big_page() {
    let mut eeprom = setup();
    match eeprom.write_page(&[0, 0], &[0; 65]) {
        Err(Error::TooMuchData) => {},
        _ => panic!("Error::TooMuchData not returned.")
    }
}

#[test]
fn sends_correct_parameters_for_page_write() {
    let mut eeprom = setup();
    let address = [0x12, 0x34];
    let data = [0xCD];
    eeprom.write_page(&address, &data).unwrap();
    check_sent_data(eeprom, &[address[0], address[1], data[0]]);
}

#[test]
fn can_write_page() {
    // cannot write a full page with 64 bytes because the write buffer
    // in embedded-hal-mock is limited to 64 bytes and the address
    // is prepended to the data.
    let mut eeprom = setup();
    let address = [0x12, 0x34];
    let data = [0xCD; 62];
    eeprom.write_page(&address, &data).unwrap();

    let mut payload = [0xCD; 64];
    payload[0] = address[0];
    payload[1] = address[1];
    check_sent_data(eeprom, &payload);
}

#[test]
fn can_read_current_address() {
    let mut eeprom = setup();
    let data = eeprom.read_current_address().unwrap();
    assert_eq!(0xAB, data);
}
