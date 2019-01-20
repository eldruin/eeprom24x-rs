extern crate eeprom24x;
use eeprom24x::{addr_size, page_size, Eeprom24x, Error, SlaveAddr};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0b101_0000;

macro_rules! create {
    ($create:ident, $AS:ident, $PS:ident) => {
        fn $create(transactions: &[I2cTrans]) -> Eeprom24x<I2cMock, page_size::$PS, addr_size::$AS> {
            Eeprom24x::$create(I2cMock::new(&transactions), SlaveAddr::default())
        }
    };
}

fn destroy<T, V>(eeprom: Eeprom24x<I2cMock, T, V>) {
    eeprom.destroy().done();
}

create!(new_24x00, One, No);
create!(new_24x01, One, B8);
create!(new_24x02, One, B8);
create!(new_24x04, One, B16);
create!(new_24x08, One, B16);
create!(new_24x16, One, B16);
create!(new_24x32, Two, B32);
create!(new_24x64, Two, B32);
create!(new_24x128, Two, B64);
create!(new_24x256, Two, B64);
create!(new_24x512, Two, B128);
create!(new_24xm01, Two, B256);
create!(new_24xm02, Two, B256);

macro_rules! for_all_ics{
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00,  new_24x00);
            $name!(for_24x01,  new_24x01);
            $name!(for_24x02,  new_24x02);
            $name!(for_24x04,  new_24x04);
            $name!(for_24x08,  new_24x08);
            $name!(for_24x16,  new_24x16);
            $name!(for_24x32,  new_24x32);
            $name!(for_24x64,  new_24x64);
            $name!(for_24x128, new_24x128);
            $name!(for_24x256, new_24x256);
            $name!(for_24x512, new_24x512);
            $name!(for_24xm01, new_24xm01);
            $name!(for_24xm02, new_24xm02);
        }
    };
}

macro_rules! for_all_ics_with_1b_addr {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00,  new_24x00);
            $name!(for_24x01,  new_24x01);
            $name!(for_24x02,  new_24x02);
            $name!(for_24x04,  new_24x04);
            $name!(for_24x08,  new_24x08);
            $name!(for_24x16,  new_24x16);
        }
    };
}

macro_rules! for_all_ics_with_2b_addr {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x32,  new_24x32);
            $name!(for_24x64,  new_24x64);
            $name!(for_24x128, new_24x128);
            $name!(for_24x256, new_24x256);
            $name!(for_24x512, new_24x512);
            $name!(for_24xm01, new_24xm01);
            $name!(for_24xm02, new_24xm02);
        }
    };
}

macro_rules! for_all_ics_with_1b_addr_and_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01,  new_24x01,    8);
            $name!(for_24x02,  new_24x02,    8);
            $name!(for_24x04,  new_24x04,   16);
            $name!(for_24x08,  new_24x08,   16);
            $name!(for_24x16,  new_24x16,   16);
        }
    };
}

macro_rules! for_all_ics_with_2b_addr_and_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x32,  new_24x32,   32);
            $name!(for_24x64,  new_24x64,   32);
            $name!(for_24x128, new_24x128,  64);
            $name!(for_24x256, new_24x256,  64);
            $name!(for_24x512, new_24x512, 128);
            $name!(for_24xm01, new_24xm01, 256);
            $name!(for_24xm02, new_24xm02, 256);
        }
    };
}

macro_rules! for_all_ics_with_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01,  new_24x01,    8);
            $name!(for_24x02,  new_24x02,    8);
            $name!(for_24x04,  new_24x04,   16);
            $name!(for_24x08,  new_24x08,   16);
            $name!(for_24x16,  new_24x16,   16);
            $name!(for_24x32,  new_24x32,   32);
            $name!(for_24x64,  new_24x64,   32);
            $name!(for_24x128, new_24x128,  64);
            $name!(for_24x256, new_24x256,  64);
            $name!(for_24x512, new_24x512, 128);
            $name!(for_24xm01, new_24xm01, 256);
            $name!(for_24xm02, new_24xm02, 256);
        }
    };
}

macro_rules! construction_test {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let eeprom = $create(&[]);
            destroy(eeprom);
        }
    };
}
for_all_ics!(construction_test);

macro_rules! can_read_byte_v1 {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(DEV_ADDR, vec![0xF], vec![0xAB])];
            let mut eeprom = $create(&trans);
            let data = eeprom.read_byte(0xF).unwrap();
            assert_eq!(0xAB, data);
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(can_read_byte_v1);

macro_rules! can_read_byte_v2 {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(DEV_ADDR, vec![0xF, 0x34], vec![0xAB])];
            let mut eeprom = $create(&trans);
            let data = eeprom.read_byte(0xF34).unwrap();
            assert_eq!(0xAB, data);
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(can_read_byte_v2);

macro_rules! can_read_array_v1 {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write_read(DEV_ADDR, vec![0xF], vec![0xAB, 0xCD, 0xEF]) ];
            let mut eeprom = $create(&trans);
            let mut data = [0; 3];
            eeprom.read_data(0xF, &mut data).unwrap();
            assert_eq!([0xAB, 0xCD, 0xEF], data);
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(can_read_array_v1);

macro_rules! can_read_array_v2 {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write_read(DEV_ADDR, vec![0xF, 0x34], vec![0xAB, 0xCD, 0xEF]) ];
            let mut eeprom = $create(&trans);
            let mut data = [0; 3];
            eeprom.read_data(0xF34, &mut data).unwrap();
            assert_eq!([0xAB, 0xCD, 0xEF], data);
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(can_read_array_v2);

macro_rules! can_read_current_address {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::read(DEV_ADDR, vec![0xAB])];
            let mut eeprom = $create(&trans);
            let data = eeprom.read_current_address().unwrap();
            assert_eq!(0xAB, data);
            destroy(eeprom);
        }
    };
}
for_all_ics!(can_read_current_address);

macro_rules! can_write_byte_v1{
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![0xF, 0xAB])];
            let mut eeprom = $create(&trans);
            eeprom.write_byte(0xF, 0xAB).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(can_write_byte_v1);

macro_rules! can_write_byte_v2{
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![0xF, 0x34, 0xAB])];
            let mut eeprom = $create(&trans);
            eeprom.write_byte(0xF34, 0xAB).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(can_write_byte_v2);

macro_rules! write_empty_data_does_nothing {
    ($name:ident, $create:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            eeprom.write_page(0xF, &[]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_page_size!(write_empty_data_does_nothing);

macro_rules! can_write_array_v1 {
    ($name:ident, $create:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write(DEV_ADDR, vec![0x34, 0xAB, 0xCD, 0xEF]) ];
            let mut eeprom = $create(&trans);
            eeprom.write_page(0x34, &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr_and_page_size!(can_write_array_v1);

macro_rules! can_write_array_v2 {
    ($name:ident, $create:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [ I2cTrans::write(DEV_ADDR, vec![0xF, 0x34, 0xAB, 0xCD, 0xEF]) ];
            let mut eeprom = $create(&trans);
            eeprom.write_page(0xF34, &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr_and_page_size!(can_write_array_v2);

fn assert_too_much_data<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::TooMuchData) => (),
        _ => panic!("Error::TooMuchData not returned."),
    }
}
#[test]
fn check_data_assert_matches() {
    assert_too_much_data::<(), ()>(Err(Error::TooMuchData));
}

#[test]
#[should_panic]
fn check_data_assert_fails() {
    assert_too_much_data::<(), ()>(Ok(()));
}

macro_rules! cannot_write_too_big_page {
    ($name:ident, $create:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_too_much_data(eeprom.write_page(0x34, &[0xAB; 1 + $size]));
            destroy(eeprom);
        }
    };
}
for_all_ics_with_page_size!(cannot_write_too_big_page);

macro_rules! can_write_whole_page_v1 {
    ($name:ident, $create:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut data = vec![0x34];
            data.extend_from_slice(&[0xAB; $size]);
            let trans = [I2cTrans::write(DEV_ADDR, data)];
            let mut eeprom = $create(&trans);
            eeprom.write_page(0x34, &[0xAB; $size]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr_and_page_size!(can_write_whole_page_v1);

macro_rules! can_write_whole_page_v2 {
    ($name:ident, $create:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut data = vec![0xF, 0x34];
            data.extend_from_slice(&[0xAB; $size]);
            let trans = [I2cTrans::write(DEV_ADDR, data)];
            let mut eeprom = $create(&trans);
            eeprom.write_page(0xF34, &[0xAB; $size]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr_and_page_size!(can_write_whole_page_v2);

fn assert_invalid_address<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidAddr) => (),
        _ => panic!("Error::InvalidAddr not returned."),
    }
}
#[test]
fn check_addr_assert_matches() {
    assert_invalid_address::<(), ()>(Err(Error::InvalidAddr));
}

#[test]
#[should_panic]
fn check_addr_assert_fails() {
    assert_invalid_address::<(), ()>(Ok(()));
}

macro_rules! cannot_write_invalid_addr_v1 {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_invalid_address(eeprom.write_byte(0xFFF, 0xAB));
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(cannot_write_invalid_addr_v1);

macro_rules! cannot_write_invalid_addr_v2 {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_invalid_address(eeprom.write_byte(0xFFFFF, 0xAB));
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(cannot_write_invalid_addr_v2);
