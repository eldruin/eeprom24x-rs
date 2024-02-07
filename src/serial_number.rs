use crate::{
    addr_size::{OneByte, TwoBytes},
    unique_serial, Eeprom24x, Error,
};
use embedded_hal::i2c::I2c;

/// Determine the peripheral address for accessing the secure region
/// of 24CS devices.
fn secure_region_addr(address_bits: u8, base_addr: u8) -> u8 {
    match address_bits {
        7 | 8 | 12 | 13 => 0b101_1000 | (base_addr & 0b111), // CS01,CS02, CS32, CS64
        9 => 0b101_1000 | (base_addr & 0b110),               // CS04
        10 => 0b101_1000 | (base_addr & 0b100),              // CS08
        11 => 0b101_1000,                                    // CS16
        _ => unreachable!(),
    }
}

/// Methods for interacting with the factory-programmed unique serial number
/// for devices with one byte addresses. e.g. 24CSx01, 24CSx02,24CSx04, 24CSx08,
/// and 24CSx16.
impl<I2C, PS, E> Eeprom24x<I2C, PS, OneByte, unique_serial::Yes>
where
    I2C: I2c<Error = E>,
{
    /// Read the 128-bit unique serial number.
    pub fn read_unique_serial(&mut self) -> Result<[u8; 16], Error<E>> {
        let addr = secure_region_addr(self.address_bits, self.address.addr());
        let mut serial_bytes = [0u8; 16];
        self.i2c
            .write_read(addr, &[0x80], &mut serial_bytes)
            .map_err(Error::I2C)?;
        Ok(serial_bytes)
    }
}

/// Methods for interacting with the factory-programmed unique serial number
/// for devices with two byte addresses. e.g. 24CSx32 and 24CSx64
impl<I2C, PS, E> Eeprom24x<I2C, PS, TwoBytes, unique_serial::Yes>
where
    I2C: I2c<Error = E>,
{
    /// Read the 128-bit unique serial number.
    pub fn read_unique_serial(&mut self) -> Result<[u8; 16], Error<E>> {
        let secure_region_addr = 0b101_1000 | (self.address.addr() & 0b111);
        let mut serial_bytes = [0u8; 16];
        self.i2c
            .write_read(secure_region_addr, &[0x08, 0x0], &mut serial_bytes)
            .map_err(Error::I2C)?;
        Ok(serial_bytes)
    }
}
