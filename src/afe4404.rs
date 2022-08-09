//! The main AFE4404 module.

use std::cell::RefCell;
use std::rc::Rc;

use embedded_hal::{
    i2c::{
        blocking::I2c,
        SevenBitAddress,
    }
};

use crate::register_block::RegisterBlock;
use crate::register_structs::R00h;

pub struct AFE4404<I2C> {
    address: SevenBitAddress,
    registers: RegisterBlock<I2C>,
}

impl<I2C> AFE4404<I2C>
    where I2C: I2c<SevenBitAddress> {
    pub fn new(i2c: Rc<RefCell<I2C>>, address: SevenBitAddress) -> Self {
        AFE4404 {
            address,
            registers: RegisterBlock::new(address, i2c),
        }
    }

    fn test(&mut self) {
        self.registers.r00h.write(R00h::new());
    }
}