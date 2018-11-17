# Rust 24x EEPROM Driver

[![crates.io](https://img.shields.io/crates/v/eeprom24x.svg)](https://crates.io/crates/eeprom24x)
[![Docs](https://docs.rs/eeprom24x/badge.svg)](https://docs.rs/eeprom24x)
[![Build Status](https://travis-ci.org/eldruin/eeprom24x-rs.svg?branch=master)](https://travis-ci.org/eldruin/eeprom24x-rs)

This is a platform agnostic Rust driver for the 24x series serial EEPROM,
based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

This driver allows you to:
- Read a single byte from a memory address. See: `read_byte`
- Read a byte array starting on a memory address. See: `read_data`
- Read the current memory address (please read notes). See: `read_current_address`
- Write a byte to a memory address. See: `write_byte`
- Write a byte array (up to a memory page) to a memory address. See: `write_page`

Can be used at least with the devices AT24C32, AT24C64, AT24C128, AT24C256 and AT24C512.

## The devices
These devices provides a number of bits of serial electrically erasable and
programmable read only memory (EEPROM) organized as a number of words of 8 bits
each. The devices' cascadable feature allows up to 8 devices to share a common
2-wire bus. The devices are optimized for use in many industrial and commercial
applications where low power and low voltage operation are essential.

Datasheets:
- [AT24C32/AT24C64](http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf)
- [AT24C128C](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8734-SEEPROM-AT24C128C-Datasheet.pdf)
- [AT24C256C](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf)
- [AT24C512C](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8720-SEEPROM-AT24C512C-Datasheet.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

