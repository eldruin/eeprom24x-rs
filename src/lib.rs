//! This is a platform agnostic Rust driver for the 24x series serial EEPROM,
//! based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read a single byte from a memory address. See [`read_byte()`].
//! - Read a byte array starting on a memory address. See: [`read_data()`].
//! - Read the current memory address (please read notes). See: [`read_current_address()`].
//! - Write a byte to a memory address. See: [`write_byte()`].
//! - Write a byte array (up to a memory page) to a memory address. See: [`write_page()`].
//!
//! [`read_byte()`]: Eeprom24x::read_byte
//! [`read_data()`]: Eeprom24x::read_data
//! [`read_current_address()`]: Eeprom24x::read_current_address
//! [`write_byte()`]: Eeprom24x::write_byte
//! [`write_page()`]: Eeprom24x::write_page
//!
//! Can be used at least with the devices listed below.
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
//! |-------:|------------:|------------:|----------:|:-----------|
//! |  24x00 |    128 bits |          16 |       N/A | [24C00]    |
//! |  24x01 |      1 Kbit |         128 |   8 bytes | [AT24C01]  |
//! |  24x02 |      2 Kbit |         256 |   8 bytes | [AT24C02]  |
//! |  24x04 |      4 Kbit |         512 |  16 bytes | [AT24C04]  |
//! |  24x08 |      8 Kbit |       1,024 |  16 bytes | [AT24C08]  |
//! |  24x16 |     16 Kbit |       2,048 |  16 bytes | [AT24C16]  |
//! |  24x32 |     32 Kbit |       4,096 |  32 bytes | [AT24C32]  |
//! |  24x64 |     64 Kbit |       8,192 |  32 bytes | [AT24C64]  |
//! | 24x128 |    128 Kbit |      16,384 |  64 bytes | [AT24C128] |
//! | 24x256 |    256 Kbit |      32,768 |  64 bytes | [AT24C256] |
//! | 24x512 |    512 Kbit |      65,536 | 128 bytes | [AT24C512] |
//! | 24xM01 |      1 Mbit |     131,072 | 256 bytes | [AT24CM01] |
//! | 24xM02 |      2 Mbit |     262,144 | 256 bytes | [AT24CM02] |
//!
//! [24C00]: http://ww1.microchip.com/downloads/en/DeviceDoc/24AA00-24LC00-24C00-Data-Sheet-20001178J.pdf
//! [AT24C01]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8871F-SEEPROM-AT24C01D-02D-Datasheet.pdf
//! [AT24C02]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8871F-SEEPROM-AT24C01D-02D-Datasheet.pdf
//! [AT24C04]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8896E-SEEPROM-AT24C04D-Datasheet.pdf
//! [AT24C08]: http://ww1.microchip.com/downloads/en/DeviceDoc/AT24C08D-I2C-Compatible-2-Wire-Serial-EEPROM-20006022A.pdf
//! [AT24C16]: http://ww1.microchip.com/downloads/en/DeviceDoc/20005858A.pdf
//! [AT24C32]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
//! [AT24C64]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
//! [AT24C128]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8734-SEEPROM-AT24C128C-Datasheet.pdf
//! [AT24C256]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf
//! [AT24C512]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8720-SEEPROM-AT24C512C-Datasheet.pdf
//! [AT24CM01]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8812-SEEPROM-AT24CM01-Datasheet.pdf
//! [AT24CM02]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8828-SEEPROM-AT24CM02-Datasheet.pdf
//!
//! ## Usage examples (see also examples folder)
//!
//! To create a new instance you can use the `new_<device>` methods.
//! There are many compatible vendors so the method has a somewhat generic name.
//! For example, if you are using an AT24C32, you can create a device by calling
//! `Eeprom24x::new_24x32(...)`.
//! Please refer to the [device table](#the-devices) above for further examples
//! of device names.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
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
//! let address = 0x1234;
//! let data = 0xAB;
//! eeprom.write_byte(address, data);
//! // EEPROM enters internally-timed write cycle. Will not respond for some time.
//! let retrieved_data = eeprom.read_byte(address);
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
//! let address = 0x1234;
//! let data = [0xAB; 64];
//! eeprom.write_page(address, &data);
//! // EEPROM enters internally-timed write cycle. Will not respond for some time.
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/eeprom24x/0.3.0")]
#![deny(missing_docs, unsafe_code)]
#![no_std]

extern crate embedded_hal as hal;
use core::marker::PhantomData;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Too much data passed for a write
    TooMuchData,
    /// Memory address is out of range
    InvalidAddr,
}

/// Possible slave addresses
///
/// Note that in some devices some of the address bits are used for memory addressing and
/// will therefore be ignored.
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    ///
    /// Depending on the device, some of the device address bits may be used for memory addresses.
    /// In this case, the value provided here for these bits will be ignored. Consult the
    /// device addressing in your device datasheet.
    ///
    /// e.g. For the 24xM01 devices use the A0 bit of the device address as the highest memory
    /// address bit. The value provided here will be ignored.
    Alternative(bool, bool, bool),
}

/// Memory address size markers
pub mod addr_size {
    /// 1-byte memory address.
    /// e.g. for AT24x00, AT24x01, AT24x02, AT24x04, AT24x08, AT24x16
    pub struct OneByte(());
    /// 2-byte memory address.
    /// e.g. for AT24x32, AT24x64, AT24x128, AT24x256, AT24x512, AT24xM01, AT24xM02
    pub struct TwoBytes(());
}

/// Page size markers
pub mod page_size {
    /// No page write supported. e.g. for AT24x00
    pub struct No(());
    /// 8-byte pages. e.g. for AT24x01, AT24x02
    pub struct B8(());
    /// 16-byte pages. e.g. for AT24x04, AT24x08, AT24x16
    pub struct B16(());
    /// 32-byte pages. e.g. for AT24x32, AT24x64
    pub struct B32(());
    /// 64-byte pages. e.g. for AT24x128, AT24x256
    pub struct B64(());
    /// 128-byte pages. e.g. for AT24x512
    pub struct B128(());
    /// 256-byte pages. e.g. for AT24xM01, AT24xM02
    pub struct B256(());
}

/// EEPROM24X driver
#[derive(Debug, Default)]
pub struct Eeprom24x<I2C, PS, AS> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: SlaveAddr,
    /// Number or bits used for memory addressing.
    address_bits: u8,
    /// Page size marker type.
    _ps: PhantomData<PS>,
    /// Address size marker type.
    _as: PhantomData<AS>,
}

mod private {
    use addr_size;

    pub trait Sealed {}

    impl Sealed for addr_size::OneByte {}
    impl Sealed for addr_size::TwoBytes {}
}

mod eeprom24x;
mod slave_addr;
