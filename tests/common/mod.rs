use eeprom24x::{addr_size, page_size, Eeprom24x, SlaveAddr};
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};

#[allow(unused)]
pub const DEV_ADDR: u8 = 0b101_0000;

macro_rules! create {
    ($create:ident, $AS:ident, $PS:ident) => {
        pub fn $create(
            transactions: &[I2cTrans],
        ) -> Eeprom24x<I2cMock, page_size::$PS, addr_size::$AS> {
            Eeprom24x::$create(I2cMock::new(transactions), SlaveAddr::default())
        }
    };
}

pub fn destroy<T, V>(eeprom: Eeprom24x<I2cMock, T, V>) {
    eeprom.destroy().done();
}

create!(new_24x00, OneByte, No);
create!(new_24x01, OneByte, B8);
create!(new_m24x01, OneByte, B16);
create!(new_24x02, OneByte, B8);
create!(new_m24x02, OneByte, B16);
create!(new_24x04, OneByte, B16);
create!(new_24x08, OneByte, B16);
create!(new_24x16, OneByte, B16);
create!(new_24x32, TwoBytes, B32);
create!(new_24x64, TwoBytes, B32);
create!(new_24x128, TwoBytes, B64);
create!(new_24x256, TwoBytes, B64);
create!(new_24x512, TwoBytes, B128);
create!(new_24xm01, TwoBytes, B256);
create!(new_24xm02, TwoBytes, B256);

#[macro_export]
macro_rules! for_all_ics {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00, new_24x00);
            $name!(for_24x01, new_24x01);
            $name!(for_m24x01, new_m24x01);
            $name!(for_24x02, new_24x02);
            $name!(for_m24x02, new_m24x02);
            $name!(for_24x04, new_24x04);
            $name!(for_24x08, new_24x08);
            $name!(for_24x16, new_24x16);
            $name!(for_24x32, new_24x32);
            $name!(for_24x64, new_24x64);
            $name!(for_24x128, new_24x128);
            $name!(for_24x256, new_24x256);
            $name!(for_24x512, new_24x512);
            $name!(for_24xm01, new_24xm01);
            $name!(for_24xm02, new_24xm02);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_1b_addr {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00, new_24x00);
            $name!(for_24x01, new_24x01);
            $name!(for_m24x01, new_m24x01);
            $name!(for_24x02, new_24x02);
            $name!(for_m24x02, new_m24x02);
            $name!(for_24x04, new_24x04);
            $name!(for_24x08, new_24x08);
            $name!(for_24x16, new_24x16);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_2b_addr {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x32, new_24x32);
            $name!(for_24x64, new_24x64);
            $name!(for_24x128, new_24x128);
            $name!(for_24x256, new_24x256);
            $name!(for_24x512, new_24x512);
            $name!(for_24xm01, new_24xm01);
            $name!(for_24xm02, new_24xm02);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_1b_addr_and_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01, new_24x01, 8);
            $name!(for_m24x01, new_m24x01, 16);
            $name!(for_24x02, new_24x02, 8);
            $name!(for_m24x02, new_m24x02, 16);
            $name!(for_24x04, new_24x04, 16);
            $name!(for_24x08, new_24x08, 16);
            $name!(for_24x16, new_24x16, 16);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_2b_addr_and_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x32, new_24x32, 32);
            $name!(for_24x64, new_24x64, 32);
            $name!(for_24x128, new_24x128, 64);
            $name!(for_24x256, new_24x256, 64);
            $name!(for_24x512, new_24x512, 128);
            $name!(for_24xm01, new_24xm01, 256_usize);
            $name!(for_24xm02, new_24xm02, 256_usize);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01, new_24x01, 8);
            $name!(for_m24x01, new_m24x01, 16);
            $name!(for_24x02, new_24x02, 8);
            $name!(for_m24x02, new_m24x02, 16);
            $name!(for_24x04, new_24x04, 16);
            $name!(for_24x08, new_24x08, 16);
            $name!(for_24x16, new_24x16, 16);
            $name!(for_24x32, new_24x32, 32);
            $name!(for_24x64, new_24x64, 32);
            $name!(for_24x128, new_24x128, 64);
            $name!(for_24x256, new_24x256, 64);
            $name!(for_24x512, new_24x512, 128);
            $name!(for_24xm01, new_24xm01, 256);
            $name!(for_24xm02, new_24xm02, 256);
        }
    };
}

#[macro_export]
macro_rules! for_all_ics_with_capacity {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00, new_24x00, 16);
            $name!(for_24x01, new_24x01, 1 << 7);
            $name!(for_m24x01, new_m24x01, 1 << 7);
            $name!(for_24x02, new_24x02, 1 << 8);
            $name!(for_m24x02, new_m24x02, 1 << 8);
            $name!(for_24x04, new_24x04, 1 << 9);
            $name!(for_24x08, new_24x08, 1 << 10);
            $name!(for_24x16, new_24x16, 1 << 11);
            $name!(for_24x32, new_24x32, 1 << 12);
            $name!(for_24x64, new_24x64, 1 << 13);
            $name!(for_24x128, new_24x128, 1 << 14);
            $name!(for_24x256, new_24x256, 1 << 15);
            $name!(for_24x512, new_24x512, 1 << 16);
            $name!(for_24xm01, new_24xm01, 1 << 17);
            $name!(for_24xm02, new_24xm02, 1 << 18);
        }
    };
}

#[macro_export]
macro_rules! for_all_writestorage_ics_with_capacity {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01, new_24x01, 1 << 7);
            $name!(for_m24x01, new_m24x01, 1 << 7);
            $name!(for_24x02, new_24x02, 1 << 8);
            $name!(for_m24x02, new_m24x02, 1 << 8);
            $name!(for_24x04, new_24x04, 1 << 9);
            $name!(for_24x08, new_24x08, 1 << 10);
            $name!(for_24x16, new_24x16, 1 << 11);
            $name!(for_24x32, new_24x32, 1 << 12);
            $name!(for_24x64, new_24x64, 1 << 13);
            $name!(for_24x128, new_24x128, 1 << 14);
            $name!(for_24x256, new_24x256, 1 << 15);
            $name!(for_24x512, new_24x512, 1 << 16);
            $name!(for_24xm01, new_24xm01, 1 << 17);
            $name!(for_24xm02, new_24xm02, 1 << 18);
        }
    };
}
