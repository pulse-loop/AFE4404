//! The main AFE4404 module.

use std::cell::RefCell;
use std::rc::Rc;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};
use uom::si::f32::Frequency;

use crate::RegisterBlock;

pub struct AFE4404<I2C> {
    pub(crate) registers: RegisterBlock<I2C>,
    pub(crate) clock: Frequency,
}

impl<I2C> AFE4404<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    pub fn new(i2c: I2C, address: SevenBitAddress, clock: Frequency) -> Self {
        Self {
            registers: RegisterBlock::new(address, &Rc::new(RefCell::new(i2c))),
            clock,
        }
    }
}
