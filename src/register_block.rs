use std::cell::RefCell;
use std::rc::Rc;

use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::register::Register;
use crate::register_structs::*;

pub(crate) struct RegisterBlock<I2C> {
    pub(crate) r00h: Register<I2C, R00h>,
    pub(crate) r01h: Register<I2C, R01h>,
}

impl<I2C> RegisterBlock<I2C>
    where I2C: I2c {
    pub fn new(phy_addr: SevenBitAddress, i2c: Rc<RefCell<I2C>>) -> Self {
        RegisterBlock {
            r00h: Register::new(0x00, phy_addr, Rc::clone(&i2c)),
            r01h: Register::new(0x01, phy_addr, Rc::clone(&i2c)),
        }
    }
}