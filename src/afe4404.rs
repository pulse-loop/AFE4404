//! The main AFE4404 module.

use embedded_hal::{
    i2c::{
        blocking::I2c,
        SevenBitAddress,
    }
};

pub struct AFE4404<I2C> {
    i2c: I2C,
}

impl<I2C> AFE4404<I2C>
    where I2C: I2c<SevenBitAddress> {
    pub fn new(i2c: I2C) -> Self {
        AFE4404 {
            i2c,
        }
    }
}