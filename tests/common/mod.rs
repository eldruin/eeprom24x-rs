use eeprom24x::{addr_size, page_size, Eeprom24x, SlaveAddr};
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

#[allow(unused)]
pub const DEV_ADDR: u8 = 0b101_0000;

macro_rules! create {
    ($create:ident, $AS:ident, $PS:ident) => {
        pub fn $create(
            transactions: &[I2cTrans],
        ) -> Eeprom24x<I2cMock, page_size::$PS, addr_size::$AS> {
            Eeprom24x::$create(I2cMock::new(&transactions), SlaveAddr::default())
        }
    };
}

pub fn destroy<T, V>(eeprom: Eeprom24x<I2cMock, T, V>) {
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

#[macro_export]
macro_rules! for_all_ics {
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

#[macro_export]
macro_rules! for_all_ics_with_1b_addr {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x00, new_24x00);
            $name!(for_24x01, new_24x01);
            $name!(for_24x02, new_24x02);
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

#[macro_export]
macro_rules! for_all_ics_with_1b_addr_and_page_size {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_24x01, new_24x01, 8);
            $name!(for_24x02, new_24x02, 8);
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
            $name!(for_24x32,  new_24x32,   32);
            $name!(for_24x64,  new_24x64,   32);
            $name!(for_24x128, new_24x128,  64);
            $name!(for_24x256, new_24x256,  64);
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
