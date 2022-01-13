use crate::{addr_size, page_size, private, Eeprom24x, Error, SlaveAddr};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub trait MultiSizeAddr: private::Sealed {
    const ADDRESS_BYTES: usize;

    fn fill_address(address: u32, payload: &mut [u8]);
}

impl MultiSizeAddr for addr_size::OneByte {
    const ADDRESS_BYTES: usize = 1;

    fn fill_address(address: u32, payload: &mut [u8]) {
        payload[0] = address as u8;
    }
}

impl MultiSizeAddr for addr_size::TwoBytes {
    const ADDRESS_BYTES: usize = 2;

    fn fill_address(address: u32, payload: &mut [u8]) {
        payload[0] = (address >> 8) as u8;
        payload[1] = address as u8;
    }
}

/// Common methods
impl<I2C, PS, AS> Eeprom24x<I2C, PS, AS> {
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, PS, AS> Eeprom24x<I2C, PS, AS>
where
    AS: MultiSizeAddr,
{
    fn get_device_address<E>(&self, memory_address: u32) -> Result<u8, Error<E>> {
        if memory_address >= (1 << self.address_bits) {
            return Err(Error::InvalidAddr);
        }
        let addr = self.address.devaddr(
            memory_address,
            self.address_bits,
            AS::ADDRESS_BYTES as u8 * 8,
        );
        Ok(addr)
    }
}

/// Common methods
impl<I2C, E, PS, AS> Eeprom24x<I2C, PS, AS>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
{
    /// Write a single byte in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_byte(&mut self, address: u32, data: u8) -> Result<(), Error<E>> {
        let devaddr = self.get_device_address(address)?;
        let mut payload = [0; 3];
        AS::fill_address(address, &mut payload);
        payload[AS::ADDRESS_BYTES] = data;
        self.i2c
            .write(devaddr, &payload[..=AS::ADDRESS_BYTES])
            .map_err(Error::I2C)
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: u32) -> Result<u8, Error<E>> {
        let devaddr = self.get_device_address(address)?;
        let mut memaddr = [0; 2];
        AS::fill_address(address, &mut memaddr);
        let mut data = [0; 1];
        self.i2c
            .write_read(devaddr, &memaddr[..AS::ADDRESS_BYTES], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    /// Read starting in an address as many bytes as necessary to fill the data array provided.
    pub fn read_data(&mut self, address: u32, data: &mut [u8]) -> Result<(), Error<E>> {
        let devaddr = self.get_device_address(address)?;
        let mut memaddr = [0; 2];
        AS::fill_address(address, &mut memaddr);
        self.i2c
            .write_read(devaddr, &memaddr[..AS::ADDRESS_BYTES], data)
            .map_err(Error::I2C)
    }
}

/// Specialization for platforms which implement `embedded_hal::blocking::i2c::Read`
impl<I2C, E, PS, AS> Eeprom24x<I2C, PS, AS>
where
    I2C: embedded_hal::blocking::i2c::Read<Error = E>,
{
    /// Read the contents of the last address accessed during the last read
    /// or write operation, _incremented by one_.
    ///
    /// Note: This may not be available on your platform.
    pub fn read_current_address(&mut self) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .read(self.address.addr(), &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}

/// Specialization for devices without page access (e.g. 24C00)
impl<I2C, E> Eeprom24x<I2C, page_size::No, addr_size::OneByte>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance of a 24x00 device (e.g. 24C00)
    pub fn new_24x00(i2c: I2C, address: SlaveAddr) -> Self {
        Eeprom24x {
            i2c,
            address,
            address_bits: 4,
            _ps: PhantomData,
            _as: PhantomData,
        }
    }
}

macro_rules! impl_create {
    ( $dev:expr, $part:expr, $address_bits:expr, $create:ident ) => {
        impl_create! {
            @gen [$create, $address_bits,
                concat!("Create a new instance of a ", $dev, " device (e.g. ", $part, ")")]
        }
    };

    (@gen [$create:ident, $address_bits:expr, $doc:expr] ) => {
        #[doc = $doc]
        pub fn $create(i2c: I2C, address: SlaveAddr) -> Self {
            Self::new(i2c, address, $address_bits)
        }
    };
}

// This macro could be simplified once https://github.com/rust-lang/rust/issues/42863 is fixed.
macro_rules! impl_for_page_size {
    ( $AS:ident, $addr_bytes:expr, $PS:ident, $page_size:expr,
        $( [ $dev:expr, $part:expr, $address_bits:expr, $create:ident ] ),* ) => {
        impl_for_page_size!{
            @gen [$AS, $addr_bytes, $PS, $page_size,
            concat!("Specialization for devices with a page size of ", stringify!($page_size), " bytes."),
            concat!("Create generic instance for devices with a page size of ", stringify!($page_size), " bytes."),
            $( [ $dev, $part, $address_bits, $create ] ),* ]
        }
    };

    (@gen [$AS:ident, $addr_bytes:expr, $PS:ident, $page_size:expr, $doc_impl:expr, $doc_new:expr,
        $( [ $dev:expr, $part:expr, $address_bits:expr, $create:ident ] ),* ] ) => {
        #[doc = $doc_impl]
        impl<I2C, E> Eeprom24x<I2C, page_size::$PS, addr_size::$AS>
        where
            I2C: Write<Error = E>
        {
            $(
                impl_create!($dev, $part, $address_bits, $create);
            )*

            #[doc = $doc_new]
            fn new(i2c: I2C, address: SlaveAddr, address_bits: u8) -> Self {
                Eeprom24x {
                    i2c,
                    address,
                    address_bits,
                    _ps: PhantomData,
                    _as: PhantomData,
                }
            }
        }

        impl<I2C, E, AS> Eeprom24x<I2C, page_size::$PS, AS>
        where
            I2C: Write<Error = E>,
            AS: MultiSizeAddr,
        {
            /// Write up to a page starting in an address.
            ///
            /// The maximum amount of data that can be written depends on the page
            /// size of the device and its overall capacity. If too much data is passed,
            /// the error `Error::TooMuchData` will be returned.
            ///
            /// After writing a byte, the EEPROM enters an internally-timed write cycle
            /// to the nonvolatile memory.
            /// During this time all inputs are disabled and the EEPROM will not
            /// respond until the write is complete.
            pub fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), Error<E>> {
                if data.len() == 0 {
                    return Ok(());
                }

                // check this before to ensure that data.len() fits into u32
                // ($page_size always fits as its maximum value is 256).
                if data.len() > $page_size {
                    // This would actually be supported by the EEPROM but
                    // the data in the page would be overwritten
                    return Err(Error::TooMuchData);
                }

                let page_boundary = address | ($page_size as u32 - 1);
                if address + data.len() as u32 > page_boundary + 1 {
                    // This would actually be supported by the EEPROM but
                    // the data in the page would be overwritten
                    return Err(Error::TooMuchData);
                }

                let devaddr = self.get_device_address(address)?;
                let mut payload: [u8; $addr_bytes + $page_size] = [0; $addr_bytes + $page_size];
                AS::fill_address(address, &mut payload);
                // only available since Rust 1.31: #[allow(clippy::range_plus_one)]
                payload[$addr_bytes..$addr_bytes + data.len()].copy_from_slice(&data);
                // only available since Rust 1.31: #[allow(clippy::range_plus_one)]
                self.i2c
                    .write(devaddr, &payload[..$addr_bytes + data.len()])
                    .map_err(Error::I2C)
            }
        }

        impl<I2C, E, AS> PageWrite<E> for Eeprom24x<I2C, page_size::$PS, AS>
        where
            I2C: Write<Error = E>,
            AS: MultiSizeAddr,
        {
            fn page_write(&mut self, address: u32, data: &[u8]) -> Result<(), Error<E>> {
                self.write_page(address, data)
            }

            fn page_size(&self) -> usize {
                $page_size
            }
        }

    };
}

/// Helper trait which gives the Storage implementation access to the `write_page` method and
/// information about the page size
pub trait PageWrite<E> {
    fn page_write(&mut self, address: u32, data: &[u8]) -> Result<(), Error<E>>;
    fn page_size(&self) -> usize;
}

impl_for_page_size!(
    OneByte,
    1,
    B8,
    8,
    ["24x01", "AT24C01", 7, new_24x01],
    ["24x02", "AT24C02", 8, new_24x02]
);
impl_for_page_size!(
    OneByte,
    1,
    B16,
    16,
    ["24x04", "AT24C04", 9, new_24x04],
    ["24x08", "AT24C08", 10, new_24x08],
    ["24x16", "AT24C16", 11, new_24x16],
    ["M24C01", "M24C01", 7, new_m24x01],
    ["M24C02", "M24C02", 8, new_m24x02]
);
impl_for_page_size!(
    TwoBytes,
    2,
    B32,
    32,
    ["24x32", "AT24C32", 12, new_24x32],
    ["24x64", "AT24C64", 13, new_24x64]
);
impl_for_page_size!(
    TwoBytes,
    2,
    B64,
    64,
    ["24x128", "AT24C128", 14, new_24x128],
    ["24x256", "AT24C256", 15, new_24x256]
);
impl_for_page_size!(
    TwoBytes,
    2,
    B128,
    128,
    ["24x512", "AT24C512", 16, new_24x512]
);
impl_for_page_size!(
    TwoBytes,
    2,
    B256,
    256,
    ["24xM01", "AT24CM01", 17, new_24xm01],
    ["24xM02", "AT24CM02", 18, new_24xm02]
);
