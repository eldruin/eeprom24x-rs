//! This is a platform agnostic Rust driver for the 24x series serial EEPROM,
//! based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read a single byte from a memory address: `read_byte`
//! - Read a byte array starting on a memory address: `read_data`
//! - Read the current memory address (please read notes): `read_current_address`
//! - Write a byte to a memory address: `write_byte`
//! - Write a byte array (up to a memory page) to a memory address: `write_page`
//!
//! Can be used at least with the devices AT24C32, AT24C64, AT24C128, AT24C256 and AT24C512.
//!
//! ## The devices
//!
//! These devices provides a number of bits of serial electrically erasable and
//! programmable read only memory (EEPROM) organized as a number of words of 8 bits
//! each. The devices' cascadable feature allows up to 8 devices to share a common
//! 2-wire bus. The devices are optimized for use in many industrial and commercial
//! applications where low power and low voltage operation are essential.
//!
//! | Device | Memory bits | 8-bit words | Page size | Datasheet  |
//! |--------|-------------|-------------|-----------|------------|
//! | 24x32  | 32,768      | 4096        | 32 bytes  | [AT24C32]  |
//! | 24x64  | 65,536      | 8192        | 32 bytes  | [AT24C64]  |
//! | 24x128 | 131,072     | 16,384      | 64 bytes  | [AT24C128] |
//! | 24x256 | 262,144     | 32,768      | 64 bytes  | [AT24C256] |
//! | 24x512 | 524,288     | 65,536      | 128 bytes | [AT24C512] |
//!
//! [AT24C32]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
//! [AT24C64]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
//! [AT24C128]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8734-SEEPROM-AT24C128C-Datasheet.pdf
//! [AT24C256]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf
//! [AT24C512]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8720-SEEPROM-AT24C512C-Datasheet.pdf
//!
//! ## Usage examples (see also examples folder)
//!
//! To create a new instance you can use the `new_<device>` methods.
//! There are many compatible vendors so the method has a somewhat generic name.
//! For example, if you are using an AT24C32, you can create a device by calling
//! `Eeprom24x::new_24x32(...)`.
//! Please refer to the [device table](#the-devices) above for more examples.
//!
//! ### Instantiating with the default address
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! // using the AT24C256
//! let mut eeprom = Eeprom24x::new_24x256(dev, address);
//! # }
//! ```
//!
//! ### Providing an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut eeprom = Eeprom24x::new_24x256(dev, address);
//! # }
//! ```
//!
//! ### Writting and reading a byte
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut eeprom = Eeprom24x::new_24x256(dev, SlaveAddr::default());
//! let address = [0x12, 0x34];
//! let data = 0xAB;
//! eeprom.write_byte(&address, data);
//! // EEPROM enters internally-timed write cycle. Will not respond for some time.
//! let retrieved_data = eeprom.read_byte(&address);
//! # }
//! ```
//!
//! ### Writting a page
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate eeprom24x;
//!
//! use hal::I2cdev;
//! use eeprom24x::{ Eeprom24x, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut eeprom = Eeprom24x::new_24x256(dev, SlaveAddr::default());
//! let address = [0x12, 0x34];
//! let data = [0xAB; 64];
//! eeprom.write_page(&address, &data);
//! // EEPROM enters internally-timed write cycle. Will not respond for some time.
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};
use core::marker::PhantomData;

pub mod ic;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Too much data passed for a write
    TooMuchData
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    Alternative(bool, bool, bool)
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    /// Get slave address as u8
    fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => 0b101_0000,
            SlaveAddr::Alternative(a2, a1, a0) =>
                SlaveAddr::default().addr() |
                ((a2 as u8) << 2)           |
                ((a1 as u8) << 1)           |
                  a0 as u8
        }
    }
}

/// EEPROM24X driver
#[derive(Debug, Default)]
pub struct Eeprom24x<I2C, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: SlaveAddr,

    _ic: PhantomData<IC>,
}

/// Common methods
impl<I2C, E, IC> Eeprom24x<I2C, IC>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write a single byte in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_byte(&mut self, address: &[u8; 2], data: u8) -> Result<(), Error<E>> {
        let payload = [address[0], address[1], data];
        self.i2c
            .write(self.address.addr(), &payload)
            .map_err(Error::I2C)
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: &[u8; 2]) -> Result<u8, Error<E>> {
        let mut data = [0; 1];
        self.i2c
            .write_read(self.address.addr(), &[address[0], address[1]], &mut data)
            .map_err(Error::I2C).and(Ok(data[0]))
    }

    /// Read starting in an address as many bytes as necessary to fill the data array provided.
    pub fn read_data(&mut self, address: &[u8; 2], data: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(self.address.addr(), &[address[0], address[1]], data)
            .map_err(Error::I2C)
    }
}

/// Specialization for platforms which implement `embedded_hal::blocking::i2c::Read`
impl<I2C, E, IC> Eeprom24x<I2C, IC>
where
    I2C: hal::blocking::i2c::Read<Error = E>
{
    /// Read the contents of the last address accessed during the last read
    /// or write operation, _incremented by one_.
    ///
    /// Note: This may not be available on your platform.
    pub fn read_current_address(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .read(self.address.addr(), &mut data)
            .map_err(Error::I2C).and(Ok(data[0]))
    }
}

macro_rules! impl_device_with_write_page {
    ( $dev:expr, $part:ident, $create:ident, $ic:ident, $page_size:expr ) => {
        impl_device_with_write_page!{
            @gen [$create, $ic, $page_size,
            concat!("Specialization for ", $dev, " devices (e.g. ", stringify!($part), ")"),
            concat!("Create a new instance of a ", $dev, " device (e.g. ", stringify!($part), ")")]
        }
    };

    (@gen [$create:ident, $ic:ident, $page_size:expr, $doc_impl:expr, $doc_new:expr] ) => {
        #[doc = $doc_impl]
        impl<I2C, E> Eeprom24x<I2C, ic::$ic>
        where
            I2C: Write<Error = E> + WriteRead<Error = E>,
        {
            #[doc = $doc_new]
            pub fn $create(i2c: I2C, address: SlaveAddr) -> Self {
                Eeprom24x {
                    i2c,
                    address,
                    _ic : PhantomData,
                }
            }

            /// Write up to a page starting in an address.
            ///
            /// The maximum amount of data that can be written depends on the page
            /// size of the device. If too much data is passed, the error
            /// `Error::TooMuchData` will be returned.
            ///
            /// After writing a byte, the EEPROM enters an internally-timed write cycle
            /// to the nonvolatile memory.
            /// During this time all inputs are disabled and the EEPROM will not
            /// respond until the write is complete.
            pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
                if data.len() == 0 {
                    return Ok(());
                }
                if data.len() > $page_size {
                    // This would actually be supported by the EEPROM but
                    // the data would be overwritten
                    return Err(Error::TooMuchData);
                }

                let mut payload : [u8; 2 + $page_size] = [0; 2 + $page_size];
                payload[0] = address[0];
                payload[1] = address[1];
                payload[2..=(1+data.len())].copy_from_slice(&data);
                self.i2c
                    .write(self.address.addr(), &payload[..=(1 + data.len())])
                    .map_err(Error::I2C)
            }
        }
    };
}
impl_device_with_write_page!("24x32",  AT24C32,  new_24x32,  IC24x32,   32);
impl_device_with_write_page!("24x64",  AT24C64,  new_24x64,  IC24x64,   32);
impl_device_with_write_page!("24x128", AT24C128, new_24x128, IC24x128,  64);
impl_device_with_write_page!("24x256", AT24C256, new_24x256, IC24x256,  64);
impl_device_with_write_page!("24x512", AT24C512, new_24x512, IC24x512, 128);

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;
    use super::*;

    #[test]
    fn default_address_is_correct() {
        assert_eq!(0b101_0000, SlaveAddr::default().addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b101_0000, SlaveAddr::Alternative(false, false, false).addr());
        assert_eq!(0b101_0001, SlaveAddr::Alternative(false, false,  true).addr());
        assert_eq!(0b101_0010, SlaveAddr::Alternative(false,  true, false).addr());
        assert_eq!(0b101_0100, SlaveAddr::Alternative( true, false, false).addr());
        assert_eq!(0b101_0111, SlaveAddr::Alternative( true,  true,  true).addr());
    }
}

