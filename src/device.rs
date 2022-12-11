//! This module contains the device initialization functions.

use alloc::sync::Arc;
use spin::Mutex;

use embedded_hal::i2c::{I2c, SevenBitAddress};
use uom::si::f32::Frequency;

use crate::{
    modes::{LedMode, ThreeLedsMode, TwoLedsMode, UninitializedMode},
    register_block::RegisterBlock,
};

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
            registers: RegisterBlock::new(address, &Arc::new(Mutex::new(i2c))),
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
            registers: RegisterBlock::new(address, &Arc::new(Mutex::new(i2c))),
            clock,
            mode: core::marker::PhantomData,
        }
    }
}
