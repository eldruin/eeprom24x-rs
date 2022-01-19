use crate::{
    eeprom24x::{MultiSizeAddr, PageWrite},
    Eeprom24x, Error, Storage,
};
use core::{cmp::min, time::Duration};
use embedded_hal::{
    blocking::i2c::{Write, WriteRead},
    timer::CountDown,
};
use embedded_storage::ReadStorage;

/// Common methods
impl<I2C, PS, AS, CD> Storage<I2C, PS, AS, CD> {}

/// Common methods
impl<I2C, PS, AS, CD> Storage<I2C, PS, AS, CD> {
    /// Create a new Storage instance wrapping the given Eeprom
    pub fn new(eeprom: Eeprom24x<I2C, PS, AS>, count_down: CD) -> Self {
        Storage { eeprom, count_down }
    }

    /// Destroy driver instance, return I²C bus and timer instance.
    pub fn destroy(self) -> (I2C, CD) {
        (self.eeprom.destroy(), self.count_down)
    }
}

impl<I2C, E, PS, AS, CD> embedded_storage::ReadStorage for Storage<I2C, PS, AS, CD>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
    CD: CountDown<Time = Duration>,
{
    type Error = Error<E>;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.eeprom.read_data(offset, bytes)
    }

    fn capacity(&self) -> usize {
        1 << self.eeprom.address_bits
    }
}

impl<I2C, E, PS, AS, CD> embedded_storage::Storage for Storage<I2C, PS, AS, CD>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
    Eeprom24x<I2C, PS, AS>: PageWrite<E>,
    CD: CountDown<Time = Duration>,
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
            self.count_down.start(Duration::from_millis(5));
            let _ = nb::block!(self.count_down.wait()); // CountDown::wait() never fails
        }
        Ok(())
    }
}
