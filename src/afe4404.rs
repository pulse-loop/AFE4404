//! The main AFE4404 module.

use std::cell::RefCell;
use std::rc::Rc;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};
use uom::si::{f32::Frequency, frequency::hertz};

use crate::register_block::RegisterBlock;

/// Uninitialized mode.
pub struct UninitializedMode;

/// Three LEDs mode.
pub struct ThreeLedsMode;

/// Two LEDs mode.
pub struct TwoLedsMode;

/// Represents the lighting mode of the [`AFE4404`].
pub trait LedMode {}

impl LedMode for UninitializedMode {}
impl LedMode for ThreeLedsMode {}
impl LedMode for TwoLedsMode {}

/// Represents the [`AFE4404`] device.
pub struct AFE4404<I2C, MODE>
where
    MODE: LedMode,
{
    pub(crate) registers: RegisterBlock<I2C>,
    pub(crate) clock: Frequency,
    mode: std::marker::PhantomData<MODE>,
}

impl<I2C> AFE4404<I2C, UninitializedMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Creates a new AFE4404 instance with three LEDs.
    pub fn with_three_leds(i2c: I2C, address: SevenBitAddress) -> AFE4404<I2C, ThreeLedsMode> {
        AFE4404::<I2C, ThreeLedsMode> {
            registers: RegisterBlock::new(address, &Rc::new(RefCell::new(i2c))),
            clock: Frequency::new::<hertz>(0.0),
            mode: std::marker::PhantomData,
        }
    }

    /// Creates a new AFE4404 instance with two LEDs.
    pub fn with_two_leds(
        i2c: I2C,
        address: SevenBitAddress,
        clock: Frequency,
    ) -> AFE4404<I2C, TwoLedsMode> {
        AFE4404::<I2C, TwoLedsMode> {
            registers: RegisterBlock::new(address, &Rc::new(RefCell::new(i2c))),
            clock: Frequency::new::<hertz>(0.0),
            mode: std::marker::PhantomData,
        }
    }
}
