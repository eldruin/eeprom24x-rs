use embedded_hal_mock::i2c::Transaction as I2cTrans;
mod common;
use crate::common::{
    destroy, new_24csx01, new_24csx02, new_24csx04, new_24csx08, new_24csx16, new_24csx32,
    new_24csx64,
};

#[allow(unused)]
pub const DEV_SERIAL: [u8; 16] = [
    0xDE, 0xAD, 0xBE, 0xEF, 0xB0, 0xBA, 0xCA, 0xFE, 0xFE, 0xED, 0xC0, 0xDE, 0x1, 0x2, 0x3, 0x4,
];

macro_rules! can_read_serial_number_1byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(
                0b101_1000,
                vec![0x80],
                DEV_SERIAL.to_vec(),
            )];
            let mut eeprom = $create(&trans);
            let serial_number = eeprom.read_unique_serial().unwrap();
            assert_eq!(DEV_SERIAL, serial_number);
            destroy(eeprom);
        }
    };
}

for_all_with_serial_with_1b_addr!(can_read_serial_number_1byte_addr);

macro_rules! can_read_serial_number_2byte_addr {
    ($name:ident, $create:ident) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write_read(
                0b101_1000,
                vec![0x8, 0x0],
                DEV_SERIAL.to_vec(),
            )];
            let mut eeprom = $create(&trans);
            let serial_number = eeprom.read_unique_serial().unwrap();
            assert_eq!(DEV_SERIAL, serial_number);
            destroy(eeprom);
        }
    };
}

for_all_with_serial_with_2b_addr!(can_read_serial_number_2byte_addr);
