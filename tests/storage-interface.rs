use eeprom24x::{Eeprom24x, Error, Storage};
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use embedded_storage::{ReadStorage, Storage as _};
mod common;
use crate::common::{
    destroy, new_24x00, new_24x01, new_24x02, new_24x04, new_24x08, new_24x128, new_24x16,
    new_24x256, new_24x32, new_24x512, new_24x64, new_24xm01, new_24xm02, new_m24x01, new_m24x02,
    DEV_ADDR,
};

struct MockCountDown;
impl embedded_hal::timer::CountDown for MockCountDown {
    type Time = core::time::Duration;
    fn start<T>(&mut self, _count: T)
    where
        T: Into<core::time::Duration>,
    {
        // no-op, just mock
    }
    fn wait(&mut self) -> nb::Result<(), void::Void> {
        Ok(()) // always time-out immediately, just used for busy-waiting
    }
}

fn storage_new<PS, AS>(
    eeprom: Eeprom24x<I2cMock, PS, AS>,
) -> Storage<I2cMock, PS, AS, MockCountDown> {
    Storage::new(eeprom, MockCountDown)
}

macro_rules! can_query_capacity {
    ($name:ident, $create:ident, $capacity:expr) => {
        #[test]
        fn $name() {
            let storage = storage_new($create(&[]));
            let capacity = storage.capacity();
            assert_eq!(capacity, $capacity);
            destroy(storage.eeprom);
        }
    };
}
for_all_ics_with_capacity!(can_query_capacity);

macro_rules! can_read_byte_1byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(DEV_ADDR, vec![0xF], vec![0xAB])];
            let mut storage = storage_new($create(&trans));
            let mut data = [0u8; 1];
            storage.read(0xF, &mut data).unwrap();
            assert_eq!(0xAB, data[0]);
            destroy(storage.eeprom);
        }
    };
}
for_all_ics_with_1b_addr!(can_read_byte_1byte_addr);

macro_rules! can_read_byte_2byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(DEV_ADDR, vec![0xF, 0x34], vec![0xAB])];
            let mut storage = storage_new($create(&trans));
            let mut data = [0u8; 1];
            storage.read(0xF34, &mut data).unwrap();
            assert_eq!(0xAB, data[0]);
            destroy(storage.eeprom);
        }
    };
}
for_all_ics_with_2b_addr!(can_read_byte_2byte_addr);

macro_rules! can_write_array_1byte_addr {
    ($name:ident, $create:ident, $_page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![0x34, 0xAB, 0xCD, 0xEF])];
            let mut storage = storage_new($create(&trans));
            storage.write(0x34, &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(storage.eeprom);
        }
    };
}
for_all_ics_with_1b_addr_and_page_size!(can_write_array_1byte_addr);

macro_rules! can_write_array_2byte_addr {
    ($name:ident, $create:ident, $_page_size:expr) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![0xF, 0x34, 0xAB, 0xCD, 0xEF])];
            let mut storage = storage_new($create(&trans));
            storage.write(0xF34, &[0xAB, 0xCD, 0xEF]).unwrap();
            destroy(storage.eeprom);
        }
    };
}
for_all_ics_with_2b_addr_and_page_size!(can_write_array_2byte_addr);

macro_rules! cannot_write_too_much_data {
    ($name:ident, $create:ident, $capacity:expr) => {
        #[test]
        fn $name() {
            let mut storage = storage_new($create(&[]));
            match storage.write(0x34, &[0xAB; 1 + $capacity]) {
                Err(Error::TooMuchData) => (),
                _ => panic!("Error::TooMuchData not returned."),
            }
            destroy(storage.eeprom);
        }
    };
}
for_all_writestorage_ics_with_capacity!(cannot_write_too_much_data);
