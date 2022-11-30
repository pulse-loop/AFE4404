//! The main AFE4404 module.

#![allow(clippy::module_name_repetitions)]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;

use embedded_hal::i2c::{I2c, SevenBitAddress};
use uom::si::f32::Frequency;

use crate::register_block::RegisterBlock;

pub mod adc;
pub mod clock;
pub mod led_current;
pub mod measurement_window;
pub mod system;
pub mod tia;
pub mod value_reading;

/// Uninitialized mode.
#[derive(Debug)]
pub struct UninitializedMode;

/// Three LEDs mode.
#[derive(Debug)]
pub struct ThreeLedsMode;

/// Two LEDs mode.
#[derive(Debug)]
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
    mode: core::marker::PhantomData<MODE>,
}

impl<I2C> AFE4404<I2C, UninitializedMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Creates a new AFE4404 instance with three LEDs.
    pub fn with_three_leds(
        i2c: I2C,
        address: SevenBitAddress,
        clock: Frequency,
    ) -> AFE4404<I2C, ThreeLedsMode> {
        AFE4404::<I2C, ThreeLedsMode> {
            registers: RegisterBlock::new(address, &Rc::new(RefCell::new(i2c))),
            clock,
            mode: core::marker::PhantomData,
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
            clock,
            mode: core::marker::PhantomData,
        }
    }
}
