# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2022-01-20
### Added
- Add support for STM M24C01 and M24C02.
- Implement `embedded_storage::ReadStorage` and `embedded_storage::Storage` traits.

### Changed
- [breaking-change] Increase MSRV to version 1.51.0.

## [0.4.0] - 2021-09-04
### Added
- `PartialEq` implementation for `SlaveAddr`.

### Changed
- [breaking-change] Remove `Default` derive for `Eeprom24x`.
  Technically a breaking change but it should not affect anybody.

## [0.3.0] - 2019-01-20
### Changed
- [breaking-change] The addresses are now passed as a single `u32`.
User code should be easy to adapt:
`eeprom.read_byte([0x12, 0x34])` now becomes: `eeprom.read_byte(0x1234)`.

### Fixed
- High memory addressing in devices using some device address bits for memory
addressing: `24x04`, `24x08`, `24x16`, `24xM01`, `24xM02`.
- Protect against memory address rollover.
- Protect against page address rollover.

## [0.2.1] - 2019-01-20
### Removed
- [breaking-change] Removed support for devices that use some device address
bits for memory addressing: `24x04`, `24x08`, `24x16`, `24xM01`, `24xM02` as
the addressing was erroneous. Please upgrade to version `0.3.0` to use them.

## [0.2.0] - 2018-11-22
### Added
- Add support for many more devices.

### Changed
- [breaking-change] The addresses are now passed to the methods per value for
efficiency reasons. i.e. `address: &[u8; 2]` has now become `address: [u8; 2]`
in all methods. User code should be easy to adapt:
`eeprom.read_byte(&[0x12, 0x34])` now becomes: `eeprom.read_byte([0x12, 0x34])`.

- [breaking-change] Changed type of parameter of the `Eeprom24x` struct. Now it
is a marker type for the page size instead of the device name.

## [0.1.1] - 2018-08-22
### Fixed
- Disallow setting a different slave address through `SlaveAddr::Default`.

## 0.1.0 - 2018-08-18

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future, in case I decide that something can be
further improved. All changes will be documented in this CHANGELOG.

[Unreleased]: https://github.com/eldruin/eeprom24x-rs/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/eldruin/eeprom24x-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/eldruin/eeprom24x-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/eldruin/eeprom24x-rs/compare/v0.2.0...v0.3.0
[0.2.1]: https://github.com/eldruin/eeprom24x-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/eeprom24x-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/eldruin/eeprom24x-rs/compare/v0.1.0...v0.1.1
