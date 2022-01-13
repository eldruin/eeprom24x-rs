use crate::SlaveAddr;

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    /// Get slave address as u8
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => 0b101_0000,
            SlaveAddr::Alternative(a2, a1, a0) => {
                SlaveAddr::default().addr() | ((a2 as u8) << 2) | ((a1 as u8) << 1) | a0 as u8
            }
        }
    }

    /// Get the device address possibly including some bits from the memory address, e.g. for
    /// AT24C16 the 8 bit device address is: 1 0 1 0 A10 A9 A8 R/W , i.e. the highest 3 bits of
    /// the memory address are moved into the device address.
    ///
    /// num_address_bits is the total number of address bits, shift is the number of address bits
    /// which are transmitted separately. Theoretically, max(0, num_address_bits-shift) is the
    /// number of address bits that are moved into the device address, but this overflows in many
    /// cases and requires special handling for num_address_bits < shift.
    pub(crate) fn devaddr(self, memory_address: u32, num_address_bits: u8, shift: u8) -> u8 {
        // the part in parentheses creates num_address_bits ones; after right-shifting, 0..3 ones
        // remain; the calculations have to be done in u32 to prevent overflow
        let memmask: u32 = ((1 << num_address_bits) - 1) >> shift;
        // the inverse is the part of the device address that we keep
        let devmask = !memmask as u8;
        let hi_addr_bits = memory_address >> shift;
        (self.addr() & devmask) | hi_addr_bits as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_address_is_correct() {
        assert_eq!(0b101_0000, SlaveAddr::default().addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            0b101_0000,
            SlaveAddr::Alternative(false, false, false).addr()
        );
        assert_eq!(
            0b101_0001,
            SlaveAddr::Alternative(false, false, true).addr()
        );
        assert_eq!(
            0b101_0010,
            SlaveAddr::Alternative(false, true, false).addr()
        );
        assert_eq!(
            0b101_0100,
            SlaveAddr::Alternative(true, false, false).addr()
        );
        assert_eq!(0b101_0111, SlaveAddr::Alternative(true, true, true).addr());
    }

    #[test]
    fn assemble_devaddr() {
        assert_eq!(0b101_0001, SlaveAddr::Default.devaddr(0b1_1111_1111, 9, 8));
        assert_eq!(0b101_0000, SlaveAddr::Default.devaddr(0b0_1111_1111, 9, 8));
        assert_eq!(
            0b101_0011,
            SlaveAddr::Default.devaddr(0b11_1111_1111, 10, 8)
        );
        assert_eq!(
            0b101_0010,
            SlaveAddr::Default.devaddr(0b10_1111_1111, 10, 8)
        );
        assert_eq!(
            0b101_0111,
            SlaveAddr::Default.devaddr(0b111_1111_1111, 11, 8)
        );
        assert_eq!(
            0b101_0101,
            SlaveAddr::Default.devaddr(0b101_1111_1111, 11, 8)
        );
        assert_eq!(
            0b101_0101,
            SlaveAddr::Default.devaddr(0b101_1111_1111_1111_1111, 19, 16)
        );
    }
}
