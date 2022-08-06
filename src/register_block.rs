use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::register::Register;
use crate::register_structs::*;

pub(crate) struct RegisterBlock<'a, I2C> {
    pub(crate) r00h: Register<'a, I2C, R00h>,
    pub(crate) r01h: Register<'a, I2C, R01h>,
}

impl<'a, I2C> RegisterBlock<'a, I2C>
    where I2C: I2c {
    pub fn new(phy_addr: SevenBitAddress, i2c: &'a mut I2C) -> Self {
        RegisterBlock {
            r00h: Register::new(0x00, phy_addr, i2c),
            r01h: Register::new(0x01, phy_addr, i2c),
        }
    }
}