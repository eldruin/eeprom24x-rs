use eeprom24x::{Eeprom24x, SlaveAddr};
use embedded_hal::blocking::delay::DelayMs;
use linux_embedded_hal::{Delay, I2cdev};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut eeprom = Eeprom24x::new_24x256(dev, address);
    let memory_address = 0x1234;
    let data = 0xAB;

    eeprom.write_byte(memory_address, data).unwrap();

    Delay.delay_ms(5u16);

    let read_data = eeprom.read_byte(memory_address).unwrap();

    println!(
        "Read memory address: {}, retrieved content: {}",
        memory_address, &read_data
    );

    let _dev = eeprom.destroy(); // Get the I2C device back
}
