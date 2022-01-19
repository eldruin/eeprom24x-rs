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
impl<I2C, PS, AS, CD> Storage<I2C, PS, AS, CD>
where
    CD: CountDown<Time = Duration>,
{
    /// Create a new Storage instance wrapping the given Eeprom
    pub fn new(eeprom: Eeprom24x<I2C, PS, AS>, mut count_down: CD) -> Self {
        // When writing to the eeprom, we start a countdown of 5 ms after each page and wait for
        // the timer before writing to the next page. Therefore, we always need a valid countdown
        // so we start it here without any delay.
        // Furthermore, we also have to wait for the countdown before reading the eeprom again.
        // Basically, we have to wait before any I2C access and ensure that the countdown is
        // running again afterwards.
        count_down.start(Duration::from_millis(0));
        Storage { eeprom, count_down }
    }
}

/// Common methods
impl<I2C, PS, AS, CD> Storage<I2C, PS, AS, CD> {
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
        let _ = nb::block!(self.count_down.wait()); // CountDown::wait() never fails
        self.count_down.start(Duration::from_millis(0));
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
            let _ = nb::block!(self.count_down.wait()); // CountDown::wait() never fails
            let this_page_offset = offset as usize % page_size;
            let this_page_remaining = page_size - this_page_offset;
            let chunk_size = min(bytes.len(), this_page_remaining);
            self.eeprom.page_write(offset, &bytes[..chunk_size])?;
            offset += chunk_size as u32;
            bytes = &bytes[chunk_size..];
            // TODO At least ST's eeproms allow polling, i.e. trying the next i2c access which will
            // just be NACKed as long as the device is still busy. This could potentially speed up
            // the write process.
            // TODO Currently outdated comment:
            // A (theoretically needless) delay after the last page write ensures that the user can
            // call Storage::write() again immediately.
            self.count_down.start(Duration::from_millis(5));
        }
        Ok(())
    }
}
