//! This is a platform agnostic Rust driver for the AT24CX series serial EEPROM,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! Can be used with the devices AT24C32, AT24C64, AT24C128, AT24C256 and AT24C512.
//!
//! ## The devices
//!
//! These devices provides a number of bits of serial electrically erasable and
//! programmable read only memory (EEPROM) organized as a number of words of 8 bits
//! each. The devices' cascadable feature allows up to 8 devices to share a common
//! 2-wire bus. The devices are optimized for use in many industrial and commercial
//! applications where low power and low voltage operation are essential
//!
//! ### AT24C32
//!
//! Provides 32,768 bits of EEPROM organized as 4096 words of 8 bits each.
//! The memory page size is 32 bits.
//!
//! - Datasheet [AT24C32/AT24C64](http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf)
//!
//! ### AT24C64
//!
//! Provides 65,536 bits of EEPROM organized as 8192 words of 8 bits each.
//! The memory page size is 32 bits.
//!
//! - Datasheet [AT24C32/AT24C64](http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf)
//!
//! ### AT24C128
//!
//! Provides 131,072 bits of EEPROM organized as 16,384 words of 8 bits each.
//! The memory page size is 64 bits.
//!
//! - Datasheet [AT24C128C](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8734-SEEPROM-AT24C128C-Datasheet.pdf)
//!
//! ### AT24C256
//!
//! Provides 262,144 bits of EEPROM organized as 32,768 words of 8 bits each.
//! The memory page size is 64 bits.
//!
//! - Datasheet [AT24C256C](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf)
//!
//! ### AT24C512
//!
//! Provides 524,288 bits of EEPROM organized as 65,536 words of 8 bits each.
//! The memory page size is 128 bits.
//!
//! - Datasheet [AT24C512C](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8720-SEEPROM-AT24C512C-Datasheet.pdf)
//!
//! ## Usage
//!
//! ### Instantiating
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate at24cx;
//!
//! use hal::{I2cdev};
//! use at24cx::{At24cx, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default().addr();
//! let mut eeprom = At24cx::new_at24c256(dev, address);
//! # }
//! ```
//!
//! ### Providing an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate at24cx;
//!
//! use hal::{I2cdev};
//! use at24cx::{At24cx, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0).addr();
//! let mut eeprom = At24cx::new_at24c256(dev, address);
//! # }
//! ```
//!
//! ### Writting and reading a byte
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate at24cx;
//!
//! use hal::{I2cdev};
//! use at24cx::{At24cx, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut eeprom = At24cx::new_at24c256(dev, SlaveAddr::default().addr());
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
//! extern crate at24cx;
//!
//! use hal::{I2cdev};
//! use at24cx::{At24cx, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut eeprom = At24cx::new_at24c256(dev, SlaveAddr::default().addr());
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
    I2c(E),
    /// Too much data passed for a write
    TooMuchData
}

/// Possible slave addresses
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    Alternative(bool, bool, bool)
}

impl Default for SlaveAddr {
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    /// Get slave address as u8
    pub fn addr(&self) -> u8 {
        let default_address = 0b101_0000;
        match self {
            SlaveAddr::Default => default_address,
            SlaveAddr::Alternative(a2, a1, a0) =>
                default_address    |
                ((*a2 as u8) << 2) |
                ((*a1 as u8) << 1) |
                  *a0 as u8
        }
    }
    

}

/// AT24CX driver
#[derive(Debug, Default)]
pub struct At24cx<I2C, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,

    _ic: PhantomData<IC>,
}

impl<I2C, E, IC> At24cx<I2C, IC>
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
            .write(self.address, &payload)
            .map_err(Error::I2c)
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: &[u8; 2]) -> Result<u8, Error<E>> {
        let mut data = [0; 1];
        self.i2c
            .write_read(self.address, &[address[0], address[1]], &mut data)
            .map_err(Error::I2c).and(Ok(data[0]))
    }
}

impl<I2C, E> At24cx<I2C, ic::AT24C32>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new_at24c32(i2c: I2C, address: u8) -> Self {
        At24cx {
            i2c,
            address,
            _ic : PhantomData,
        }
    }

    /// Write up to a page starting in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
        if data.len() == 0 {
            return Ok(());
        }
        const PAGE_SIZE: usize = 32;
        if data.len() > PAGE_SIZE {
            // This would actually be supported by the EEPROM but
            // the data would be overwritten
            return Err(Error::TooMuchData);
        }
        
        let mut payload : [u8; 2 + PAGE_SIZE] = [0; 2 + PAGE_SIZE];
        write_payload(self.address, &address, &data, &mut payload, &mut self.i2c)
    }
}

impl<I2C, E> At24cx<I2C, ic::AT24C64>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new_at24c64(i2c: I2C, address: u8) -> Self {
        At24cx {
            i2c,
            address,
            _ic : PhantomData,
        }
    }

    /// Write up to a page starting in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
        if data.len() == 0 {
            return Ok(());
        }
        const PAGE_SIZE: usize = 32;
        if data.len() > PAGE_SIZE {
            // This would actually be supported by the EEPROM but
            // the data would be overwritten
            return Err(Error::TooMuchData);
        }
        
        let mut payload : [u8; 2 + PAGE_SIZE] = [0; 2 + PAGE_SIZE];
        write_payload(self.address, &address, &data, &mut payload, &mut self.i2c)
    }
}


impl<I2C, E> At24cx<I2C, ic::AT24C128>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new_at24c128(i2c: I2C, address: u8) -> Self {
        At24cx {
            i2c,
            address,
            _ic : PhantomData,
        }
    }

    /// Write up to a page starting in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
        if data.len() == 0 {
            return Ok(());
        }
        const PAGE_SIZE: usize = 64;
        if data.len() > PAGE_SIZE {
            // This would actually be supported by the EEPROM but
            // the data would be overwritten
            return Err(Error::TooMuchData);
        }
        
        let mut payload : [u8; 2 + PAGE_SIZE] = [0; 2 + PAGE_SIZE];
        write_payload(self.address, &address, &data, &mut payload, &mut self.i2c)
    }
}


impl<I2C, E> At24cx<I2C, ic::AT24C256>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new_at24c256(i2c: I2C, address: u8) -> Self {
        At24cx {
            i2c,
            address,
            _ic : PhantomData,
        }
    }

    /// Write up to a page starting in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
        if data.len() == 0 {
            return Ok(());
        }
        const PAGE_SIZE: usize = 64;
        if data.len() > PAGE_SIZE {
            // This would actually be supported by the EEPROM but
            // the data would be overwritten
            return Err(Error::TooMuchData);
        }
        
        let mut payload : [u8; 2 + PAGE_SIZE] = [0; 2 + PAGE_SIZE];
        write_payload(self.address, &address, &data, &mut payload, &mut self.i2c)
    }
}

impl<I2C, E> At24cx<I2C, ic::AT24C512>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new_at24c512(i2c: I2C, address: u8) -> Self {
        At24cx {
            i2c,
            address,
            _ic : PhantomData,
        }
    }

    /// Write up to a page starting in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
        if data.len() == 0 {
            return Ok(());
        }
        const PAGE_SIZE: usize = 128;
        if data.len() > PAGE_SIZE {
            // This would actually be supported by the EEPROM but
            // the data would be overwritten
            return Err(Error::TooMuchData);
        }
        
        let mut payload : [u8; 2 + PAGE_SIZE] = [0; 2 + PAGE_SIZE];
        write_payload(self.address, &address, &data, &mut payload, &mut self.i2c)
    }
}


fn write_payload<I2C, E>(device_address: u8, address: &[u8; 2],
                         data: &[u8], payload: &mut [u8], i2c: &mut I2C) -> Result<(), Error<E>>
    where I2C: Write<Error = E>
{
    payload[0] = address[0];
    payload[1] = address[1];
    payload[2..=(1+data.len())].copy_from_slice(&data);
    i2c.write(device_address, &payload[..=(1 + data.len())])
       .map_err(Error::I2c)
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;

    const DEVICE_ADDRESS : u8 = 0x50;

    fn setup<'a>() -> At24cx<hal::I2cMock<'a>, ic::AT24C256> {
        let mut dev = hal::I2cMock::new();
        dev.set_read_data(&[0xAB]);
        At24cx::new_at24c256(dev, DEVICE_ADDRESS)
    }

    fn check_sent_data(eeprom: At24cx<hal::I2cMock, ic::AT24C256>, data: &[u8]) {
        let dev = eeprom.destroy();
        assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
        assert_eq!(dev.get_write_data(), &data[..]);
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

