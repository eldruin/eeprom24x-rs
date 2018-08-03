//! This is a platform agnostic Rust driver for the AT24CXXX serial EEPROM,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2c(E)
}

/// AT24CXXX driver
#[derive(Debug, Default)]
pub struct At24cxxx<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8
}

impl<I2C, E> At24cxxx<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new(i2c: I2C, address: u8) -> Self {
        At24cxxx {
            i2c,
            address
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write a single byte into an address.
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


