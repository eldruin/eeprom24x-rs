use SlaveAddr;

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
            SlaveAddr::Alternative(a2, a1, a0) =>
                SlaveAddr::default().addr() |
                ((a2 as u8) << 2)           |
                ((a1 as u8) << 1)           |
                  a0 as u8
        }
    }

    /// Get the device address possibly including some bits from the memory address
    pub(crate) fn devaddr(self, memory_address: u32, address_bits: u8, shift: u8) -> u8 {
        let devmask = ((1 << address_bits) - 1) >> shift;
        let hi = (memory_address & !(1 << address_bits)) >> shift;
        (self.addr() & !(devmask as u8)) | hi as u8
    }
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;
    use super::*;

    #[test]
    fn default_address_is_correct() {
        assert_eq!(0b101_0000, SlaveAddr::default().addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b101_0000, SlaveAddr::Alternative(false, false, false).addr());
        assert_eq!(0b101_0001, SlaveAddr::Alternative(false, false,  true).addr());
        assert_eq!(0b101_0010, SlaveAddr::Alternative(false,  true, false).addr());
        assert_eq!(0b101_0100, SlaveAddr::Alternative( true, false, false).addr());
        assert_eq!(0b101_0111, SlaveAddr::Alternative( true,  true,  true).addr());
    }
}
