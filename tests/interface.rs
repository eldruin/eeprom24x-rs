use eeprom24x::Error;
use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{
    destroy, new_24x00, new_24x01, new_24x02, new_24x04, new_24x08, new_24x128, new_24x16,
    new_24x256, new_24x32, new_24x512, new_24x64, new_24xm01, new_24xm02, new_m24x01, new_m24x02,
    DEV_ADDR,
};

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

macro_rules! can_read_byte_1byte_addr {
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
for_all_ics_with_1b_addr!(can_read_byte_1byte_addr);

macro_rules! can_read_byte_2byte_addr {
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
for_all_ics_with_2b_addr!(can_read_byte_2byte_addr);

macro_rules! can_read_array_1byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(
                DEV_ADDR,
                vec![0xF],
                vec![0xAB, 0xCD, 0xEF],
            )];
            let mut eeprom = $create(&trans);
            let mut data = [0; 3];
            eeprom.read_data(0xF, &mut data).unwrap();
            assert_eq!([0xAB, 0xCD, 0xEF], data);
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(can_read_array_1byte_addr);

macro_rules! can_read_array_2byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(
                DEV_ADDR,
                vec![0xF, 0x34],
                vec![0xAB, 0xCD, 0xEF],
            )];
            let mut eeprom = $create(&trans);
            let mut data = [0; 3];
            eeprom.read_data(0xF34, &mut data).unwrap();
            assert_eq!([0xAB, 0xCD, 0xEF], data);
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(can_read_array_2byte_addr);

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

macro_rules! can_write_byte_1byte_addr {
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
for_all_ics_with_1b_addr!(can_write_byte_1byte_addr);

macro_rules! can_write_byte_2byte_addr {
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
for_all_ics_with_2b_addr!(can_write_byte_2byte_addr);

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

macro_rules! can_write_array_1byte_addr {
    ($name:ident, $create:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![0x34, 0xAB, 0xCD, 0xEF])];
            let mut eeprom = $create(&trans);
            eeprom.write_page(0x34, &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr_and_page_size!(can_write_array_1byte_addr);

macro_rules! can_write_array_2byte_addr {
    ($name:ident, $create:ident, $page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![0xF, 0x34, 0xAB, 0xCD, 0xEF])];
            let mut eeprom = $create(&trans);
            eeprom.write_page(0xF34, &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr_and_page_size!(can_write_array_2byte_addr);

// only available since Rust 1.31: #[allow(clippy::needless_pass_by_value)]
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

macro_rules! cannot_write_so_much_data_that_page_address_would_rollover {
    ($name:ident, $create:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_too_much_data(eeprom.write_page(0x01, &[0xAB; $size]));
            destroy(eeprom);
        }
    };
}
for_all_ics_with_page_size!(cannot_write_so_much_data_that_page_address_would_rollover);

macro_rules! can_write_whole_page_1byte_addr {
    ($name:ident, $create:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut data = vec![$size];
            data.extend_from_slice(&[0xAB; $size]);
            let trans = [I2cTrans::write(DEV_ADDR, data)];
            let mut eeprom = $create(&trans);
            eeprom.write_page($size, &[0xAB; $size]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr_and_page_size!(can_write_whole_page_1byte_addr);

macro_rules! can_write_whole_page_2byte_addr {
    ($name:ident, $create:ident, $size:expr) => {
        #[test]
        fn $name() {
            let mut data = vec![($size >> 8) as u8, $size as u8];
            data.extend_from_slice(&[0xAB; $size]);
            let trans = [I2cTrans::write(DEV_ADDR, data)];
            let mut eeprom = $create(&trans);
            eeprom.write_page($size as u32, &[0xAB; $size]).unwrap();
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr_and_page_size!(can_write_whole_page_2byte_addr);

#[test]
fn can_use_device_address_for_memory_addressing_1byte() {
    let trans = [I2cTrans::write(DEV_ADDR | 0x7, vec![0xBC, 0xAB])];
    let mut eeprom = new_24x16(&trans);
    eeprom.write_byte(0x7BC, 0xAB).unwrap();
    destroy(eeprom);
}

#[test]
fn can_use_device_address_for_memory_addressing_2bytes() {
    let trans = [I2cTrans::write(DEV_ADDR | 0x3, vec![0xBC, 0xDE, 0xAB])];
    let mut eeprom = new_24xm02(&trans);
    eeprom.write_byte(0x3BCDE, 0xAB).unwrap();
    destroy(eeprom);
}
