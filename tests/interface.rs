extern crate eeprom24x;
use eeprom24x::{Eeprom24x, SlaveAddr, ic, Error};
extern crate embedded_hal_mock as hal;
use hal::i2c::{ Mock as I2cMock, Transaction as I2cTrans };

const DEV_ADDR :u8 = 0b101_0000;

macro_rules! create {
    ($create:ident, $ic:ident) => {
        fn $create(transactions: &[I2cTrans]) -> Eeprom24x<I2cMock, ic::$ic> {
            Eeprom24x::$create(I2cMock::new(&transactions), SlaveAddr::default())
        }
    }
}

fn destroy<T>(eeprom: Eeprom24x<I2cMock, T>) {
    eeprom.destroy().done();
}

create!(new_24x00,  IC24x00);
create!(new_24x01,  IC24x01);
create!(new_24x02,  IC24x02);
create!(new_24x04,  IC24x04);
create!(new_24x08,  IC24x08);
create!(new_24x16,  IC24x16);
create!(new_24x32,  IC24x32);
create!(new_24x64,  IC24x64);
create!(new_24x128, IC24x128);
create!(new_24x256, IC24x256);
create!(new_24x512, IC24x512);


macro_rules! for_all_ics {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00,  new_24x00,  IC24x00);
            $name!(for_24x01,  new_24x01,  IC24x01);
            $name!(for_24x02,  new_24x02,  IC24x02);
            $name!(for_24x04,  new_24x04,  IC24x04);
            $name!(for_24x08,  new_24x08,  IC24x08);
            $name!(for_24x16,  new_24x16,  IC24x16);
            $name!(for_24x32,  new_24x32,  IC24x32);
            $name!(for_24x64,  new_24x64,  IC24x64);
            $name!(for_24x128, new_24x128, IC24x128);
            $name!(for_24x256, new_24x256, IC24x256);
            $name!(for_24x512, new_24x512, IC24x512);
        }
    }
}

macro_rules! for_all_ics_with_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01,  new_24x01,  IC24x01,    8);
            $name!(for_24x02,  new_24x02,  IC24x02,    8);
            $name!(for_24x04,  new_24x04,  IC24x04,   16);
            $name!(for_24x08,  new_24x08,  IC24x08,   16);
            $name!(for_24x16,  new_24x16,  IC24x16,   16);
            $name!(for_24x32,  new_24x32,  IC24x32,   32);
            $name!(for_24x64,  new_24x64,  IC24x64,   32);
            $name!(for_24x128, new_24x128, IC24x128,  64);
            $name!(for_24x256, new_24x256, IC24x256,  64);
            $name!(for_24x512, new_24x512, IC24x512, 128);
        }
    }
}


macro_rules! construction_test {
    ($name:ident, $create:ident, $ic:ident) => {
        #[test]
        fn $name() {
            let eeprom = $create(&[]);
            destroy(eeprom);
        }
    }
}
for_all_ics!(construction_test);

macro_rules! can_read_byte {
    ($name:ident, $create:ident, $ic:ident) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write_read(DEV_ADDR, vec![0x12, 0x34], vec![0xAB]) ];
            let mut eeprom = $create(&trans);
            let data = eeprom.read_byte(&[0x12, 0x34]).unwrap();
            assert_eq!(0xAB, data);
            destroy(eeprom);
        }
    }
}
for_all_ics!(can_read_byte);

macro_rules! can_read_array {
    ($name:ident, $create:ident, $ic:ident) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write_read(DEV_ADDR, vec![0x12, 0x34], vec![0xAB, 0xCD, 0xEF]) ];
            let mut eeprom = $create(&trans);
            let mut data = [0; 3];
            eeprom.read_data(&[0x12, 0x34], &mut data).unwrap();
            assert_eq!([0xAB, 0xCD, 0xEF], data);
            destroy(eeprom);
        }
    }
}
for_all_ics!(can_read_array);

macro_rules! can_read_current_address {
    ($name:ident, $create:ident, $ic:ident) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::read(DEV_ADDR, vec![0xAB]) ];
            let mut eeprom = $create(&trans);
            let data = eeprom.read_current_address().unwrap();
            assert_eq!(0xAB, data);
            destroy(eeprom);
        }
    }
}
for_all_ics!(can_read_current_address);

macro_rules! can_write_byte {
    ($name:ident, $create:ident, $ic:ident) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write(DEV_ADDR, vec![0x12, 0x34, 0xAB]) ];
            let mut eeprom = $create(&trans);
            eeprom.write_byte(&[0x12, 0x34], 0xAB).unwrap();
            destroy(eeprom);
        }
    }
}
for_all_ics!(can_write_byte);

macro_rules! write_empty_data_does_nothing {
    ($name:ident, $create:ident, $ic:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            eeprom.write_page(&[0x12, 0x34], &[]).unwrap();
            destroy(eeprom);
        }
    }
}
for_all_ics_with_page_size!(write_empty_data_does_nothing);

macro_rules! can_write_array {
    ($name:ident, $create:ident, $ic:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write(DEV_ADDR, vec![0x12, 0x34, 0xAB, 0xCD, 0xEF]) ];
            let mut eeprom = $create(&trans);
            eeprom.write_page(&[0x12, 0x34], &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(eeprom);
        }
    }
}
for_all_ics_with_page_size!(can_write_array);

fn assert_too_much_data<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::TooMuchData) => (),
        _ => panic!("Error::TooMuchData not returned.")
    }
}
#[test]
fn check_assert_matches() {
    assert_too_much_data::<(), ()>(Err(Error::TooMuchData));
}

#[test]
#[should_panic]
fn check_assert_fails() {
    assert_too_much_data::<(), ()>(Ok(()));
}

macro_rules! cannot_write_too_big_page {
    ($name:ident, $create:ident, $ic:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_too_much_data(eeprom.write_page(&[0x12, 0x34], &[0xAB; 1+$size]));
            destroy(eeprom);
        }
    }
}
for_all_ics_with_page_size!(cannot_write_too_big_page);

macro_rules! can_write_whole_page {
    ($name:ident, $create:ident, $ic:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut data = vec![0x12, 0x34];
            data.extend_from_slice(&[0xAB; $size]);
            let trans = [ I2cTrans::write(DEV_ADDR, data) ];
            let mut eeprom = $create(&trans);
            eeprom.write_page(&[0x12, 0x34], &[0xAB; $size]).unwrap();
            destroy(eeprom);
        }
    }
}
for_all_ics_with_page_size!(can_write_whole_page);
