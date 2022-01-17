use eeprom24x::Error;
mod common;
use crate::common::{
    destroy, new_24x00, new_24x01, new_24x02, new_24x04, new_24x08, new_24x128, new_24x16,
    new_24x256, new_24x32, new_24x512, new_24x64, new_24xm01, new_24xm02, new_m24x01, new_m24x02,
};

// only available since Rust 1.31: #[allow(clippy::needless_pass_by_value)]
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

macro_rules! cannot_write_invalid_addr_1byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_invalid_address(eeprom.write_byte(0xFFF, 0xAB));
            destroy(eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(cannot_write_invalid_addr_1byte_addr);

macro_rules! cannot_write_invalid_addr_2byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let mut eeprom = $create(&[]);
            assert_invalid_address(eeprom.write_byte(0xFFFFF, 0xAB));
            destroy(eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(cannot_write_invalid_addr_2byte_addr);

#[test]
fn cannot_write_to_position_over_capacity_1byte() {
    let mut eeprom = new_24x01(&[]);
    assert_invalid_address(eeprom.write_byte(0xFF, 0xAB));
    destroy(eeprom);
}

#[test]
fn cannot_write_to_position_over_capacity_2bytes() {
    let mut eeprom = new_24x256(&[]);
    assert_invalid_address(eeprom.write_byte(0xFFFF, 0xAB));
    destroy(eeprom);
}
