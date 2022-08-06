//! The main AFE4404 module.

use embedded_hal::{
    i2c::{
        blocking::I2c,
        SevenBitAddress,
    }
};

use crate::register_block::RegisterBlock;
use crate::register_structs::R00h;

pub struct AFE4404<'a, I2C> {
    address: SevenBitAddress,
    registers: RegisterBlock<'a, I2C>,
}

impl<'a, I2C> AFE4404<'a, I2C>
    where I2C: I2c<SevenBitAddress> {
    pub fn new(i2c: &'a mut I2C, address: SevenBitAddress) -> Self {
        AFE4404 {
            address,
            registers: RegisterBlock::new(address, i2c),
        }
    }

    fn test(&mut self) {
        self.registers.r00h.write(R00h::new());
    }
}