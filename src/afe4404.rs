//! The main AFE4404 module.

use std::cell::RefCell;
use std::rc::Rc;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};
use uom::si::f32::Frequency;

use crate::RegisterBlock;

pub struct UninitializedMode;
pub struct ThreeLedsMode;
pub struct TwoLedsMode;

pub trait LedMode {}

impl LedMode for UninitializedMode {}
impl LedMode for ThreeLedsMode {}
impl LedMode for TwoLedsMode {}

pub struct AFE4404<I2C, MODE>
where
    MODE: LedMode {
    pub(crate) registers: RegisterBlock<I2C>,
    pub(crate) clock: Frequency,
    mode: std::marker::PhantomData<MODE>
}

impl<I2C> AFE4404<I2C, UninitializedMode>
where
    I2C: I2c<SevenBitAddress>,
{
    pub fn with_three_leds(i2c: I2C, address: SevenBitAddress, clock: Frequency) -> AFE4404<I2C, ThreeLedsMode> {
        AFE4404::<I2C, ThreeLedsMode> {
            registers: RegisterBlock::new(address, &Rc::new(RefCell::new(i2c))),
            clock,
            mode: std::marker::PhantomData,
        }
    }

    pub fn with_two_leds(i2c: I2C, address: SevenBitAddress, clock: Frequency) -> AFE4404<I2C, TwoLedsMode> {
        AFE4404::<I2C, TwoLedsMode> {
            registers: RegisterBlock::new(address, &Rc::new(RefCell::new(i2c))),
            clock,
            mode: std::marker::PhantomData,
        }
    }
}
