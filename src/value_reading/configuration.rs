use uom::si::f32::ElectricPotential;

use crate::modes::{LedMode, ThreeLedsMode, TwoLedsMode};

/// Represents the values read from the [`AFE4404`].
#[derive(Copy, Clone, Debug)]
pub struct Readings<MODE: LedMode> {
    led1: ElectricPotential,
    led2: ElectricPotential,
    ambient1: ElectricPotential,
    ambient2_or_led3: ElectricPotential,
    mode: core::marker::PhantomData<MODE>,
}

impl<MODE> Readings<MODE>
where
    MODE: LedMode,
{
    /// Gets an immutable reference of the LED1 value.
    pub fn led1(&self) -> &ElectricPotential {
        &self.led1
    }

    /// Gets an immutable reference of the LED2 value.
    pub fn led2(&self) -> &ElectricPotential {
        &self.led2
    }
}

impl Readings<ThreeLedsMode> {
    pub(crate) fn new(
        led1: ElectricPotential,
        led2: ElectricPotential,
        led3: ElectricPotential,
        ambient: ElectricPotential,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1: ambient,
            ambient2_or_led3: led3,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the LED3 value.
    pub fn led3(&self) -> &ElectricPotential {
        &self.ambient2_or_led3
    }

    /// Gets an immutable reference of the Ambient value.
    pub fn ambient(&self) -> &ElectricPotential {
        &self.ambient1
    }
}

impl Readings<TwoLedsMode> {
    pub(crate) fn new(
        led1: ElectricPotential,
        led2: ElectricPotential,
        ambient1: ElectricPotential,
        ambient2: ElectricPotential,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1,
            ambient2_or_led3: ambient2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the Ambient1 value.
    pub fn ambient1(&self) -> &ElectricPotential {
        &self.ambient1
    }

    /// Gets an immutable reference of the Ambient2 value.
    pub fn ambient2(&self) -> &ElectricPotential {
        &self.ambient2_or_led3
    }
}
