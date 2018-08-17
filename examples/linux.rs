extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate eeprom24x;

use embedded_hal::blocking::delay::DelayMs;
use linux_embedded_hal::{I2cdev, Delay};
use eeprom24x::{Eeprom24x, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut eeprom = Eeprom24x::new_24x256(dev, SlaveAddr::default());
    let memory_address = [0x12, 0x34];
    let data = 0xAB;

    eeprom.write_byte(&memory_address, data).unwrap();

    Delay.delay_ms(5u16);

    let retrieved_data = eeprom.read_byte(&memory_address).unwrap();

    println!("Read memory address: [{},{}], retrieved content: {}",
             memory_address[0], memory_address[1], &retrieved_data);
}
