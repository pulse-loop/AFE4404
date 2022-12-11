use uom::si::{electric_potential::volt, f32::ElectricPotential};

use crate::modes::{LedMode, ThreeLedsMode, TwoLedsMode};

/// Represents the values read from the [`AFE4404`].
#[derive(Copy, Clone, Debug)]
pub struct Readings<MODE: LedMode> {
    led1: ElectricPotential,
    led2: ElectricPotential,
    ambient1: ElectricPotential,
    ambient2_or_led3: ElectricPotential,
    led1_minus_ambient1: ElectricPotential,
    led2_minus_ambient2: ElectricPotential,
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
        led1_minus_ambient: ElectricPotential,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1: ambient,
            ambient2_or_led3: led3,
            led1_minus_ambient1: led1_minus_ambient,
            led2_minus_ambient2: ElectricPotential::new::<volt>(0.0),
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

    /// Gets an immutable reference of the LED1 minus Ambient value.
    pub fn led1_minus_ambient(&self) -> &ElectricPotential {
        &self.led1_minus_ambient1
    }
}

impl Readings<TwoLedsMode> {
    pub(crate) fn new(
        led1: ElectricPotential,
        led2: ElectricPotential,
        ambient1: ElectricPotential,
        ambient2: ElectricPotential,
        led1_minus_ambient1: ElectricPotential,
        led2_minus_ambient2: ElectricPotential,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1,
            ambient2_or_led3: ambient2,
            led1_minus_ambient1,
            led2_minus_ambient2,
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

    /// Gets an immutable reference of the LED1 minus Ambient1 value.
    pub fn led1_minus_ambient1(&self) -> &ElectricPotential {
        &self.led1_minus_ambient1
    }

    /// Gets an immutable reference of the LED2 minus Ambient2 value.
    pub fn led2_minus_ambient2(&self) -> &ElectricPotential {
        &self.led2_minus_ambient2
    }
}

/// Represents the averaged values read from the [`AFE4404`].
#[derive(Copy, Clone, Debug)]
pub struct AveragedReadings<MODE: LedMode> {
    avg_led1_minus_ambient1: ElectricPotential,
    avg_led2_minus_ambient2: ElectricPotential,
    mode: core::marker::PhantomData<MODE>,
}

impl AveragedReadings<ThreeLedsMode> {
    pub(crate) fn new(avg_led1_minus_ambient: ElectricPotential) -> Self {
        Self {
            avg_led1_minus_ambient1: avg_led1_minus_ambient,
            avg_led2_minus_ambient2: ElectricPotential::new::<volt>(0.0),
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the averaged LED1 minus Ambient value.
    pub fn avg_led1_minus_ambient(&self) -> &ElectricPotential {
        &self.avg_led1_minus_ambient1
    }
}

impl AveragedReadings<TwoLedsMode> {
    pub(crate) fn new(
        avg_led1_minus_ambient1: ElectricPotential,
        avg_led2_minus_ambient2: ElectricPotential,
    ) -> Self {
        Self {
            avg_led1_minus_ambient1,
            avg_led2_minus_ambient2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the averaged LED1 minus Ambient1 value.
    pub fn avg_led1_minus_ambient1(&self) -> &ElectricPotential {
        &self.avg_led1_minus_ambient1
    }

    /// Gets an immutable reference of the averaged LED2 minus Ambient2 value.
    pub fn avg_led2_minus_ambient2(&self) -> &ElectricPotential {
        &self.avg_led2_minus_ambient2
    }
}
