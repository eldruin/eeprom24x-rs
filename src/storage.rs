use crate::{
    eeprom24x::{MultiSizeAddr, PageWrite},
    Eeprom24x, Error, Storage,
};
use core::cmp::min;
use embedded_hal::{delay::DelayNs, i2c::I2c};
use embedded_storage::ReadStorage;

/// Common methods
impl<I2C, PS, AS, SN, D> Storage<I2C, PS, AS, SN, D> {}

/// Common methods
impl<I2C, PS, AS, SN, D> Storage<I2C, PS, AS, SN, D>
where
    D: DelayNs,
{
    /// Create a new Storage instance wrapping the given Eeprom
    pub fn new(eeprom: Eeprom24x<I2C, PS, AS, SN>, delay: D) -> Self {
        // When writing to the eeprom, we delay by 5 ms after each page
        // before writing to the next page.
        Storage { eeprom, delay }
    }
}

/// Common methods
impl<I2C, PS, AS, SN, D> Storage<I2C, PS, AS, SN, D> {
    /// Destroy driver instance, return I²C bus and timer instance.
    pub fn destroy(self) -> (I2C, D) {
        (self.eeprom.destroy(), self.delay)
    }
}

impl<I2C, E, PS, AS, SN, D> embedded_storage::ReadStorage for Storage<I2C, PS, AS, SN, D>
where
    I2C: I2c<Error = E>,
    AS: MultiSizeAddr,
    D: DelayNs,
{
    type Error = Error<E>;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.eeprom.read_data(offset, bytes)
    }

    fn capacity(&self) -> usize {
        1 << self.eeprom.address_bits
    }
}

impl<I2C, E, PS, AS, SN, D> embedded_storage::Storage for Storage<I2C, PS, AS, SN, D>
where
    I2C: I2c<Error = E>,
    AS: MultiSizeAddr,
    Eeprom24x<I2C, PS, AS, SN>: PageWrite<E>,
    D: DelayNs,
{
    fn write(&mut self, mut offset: u32, mut bytes: &[u8]) -> Result<(), Self::Error> {
        if offset as usize + bytes.len() > self.capacity() {
            return Err(Error::TooMuchData);
        }
        let page_size = self.eeprom.page_size();
        while !bytes.is_empty() {
            let this_page_offset = offset as usize % page_size;
            let this_page_remaining = page_size - this_page_offset;
            let chunk_size = min(bytes.len(), this_page_remaining);
            self.eeprom.page_write(offset, &bytes[..chunk_size])?;
            offset += chunk_size as u32;
            bytes = &bytes[chunk_size..];
            // TODO At least ST's eeproms allow polling, i.e. trying the next i2c access which will
            // just be NACKed as long as the device is still busy. This could potentially speed up
            // the write process.
            // A (theoretically needless) delay after the last page write ensures that the user can
            // call Storage::write() again immediately.
            self.delay.delay_ms(5);
        }
        Ok(())
    }
}
