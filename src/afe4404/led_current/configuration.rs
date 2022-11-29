use uom::si::{electric_current::milliampere, f32::ElectricCurrent};

use crate::afe4404::{LedMode, ThreeLedsMode, TwoLedsMode};

/// Represents the currents of the LEDs.
#[derive(Debug, Clone, Copy)]
pub struct LedCurrentConfiguration<MODE: LedMode> {
    led1: ElectricCurrent,
    led2: ElectricCurrent,
    led3: ElectricCurrent,
    mode: core::marker::PhantomData<MODE>,
}

impl<MODE> LedCurrentConfiguration<MODE>
where
    MODE: LedMode,
{
    /// Gets an immutable reference of the current of LED1.
    pub fn led1(&self) -> &ElectricCurrent {
        &self.led1
    }

    /// Gets an immutable reference of the current of LED2.
    pub fn led2(&self) -> &ElectricCurrent {
        &self.led2
    }

    /// Gets a mutable reference of the current of LED1.
    pub fn led1_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led1
    }

    /// Gets a mutable reference of the current of LED2.
    pub fn led2_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led2
    }
}

impl LedCurrentConfiguration<ThreeLedsMode> {
    /// Creates a new `LedCurrentConfiguration`.
    pub fn new(led1: ElectricCurrent, led2: ElectricCurrent, led3: ElectricCurrent) -> Self {
        Self {
            led1,
            led2,
            led3,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the current of LED3.
    pub fn led3(&self) -> &ElectricCurrent {
        &self.led3
    }

    /// Gets a mutable reference of the current of LED3.
    pub fn led3_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led3
    }
}

impl LedCurrentConfiguration<TwoLedsMode> {
    /// Creates a new `LedCurrentConfiguration`.
    pub fn new(led1: ElectricCurrent, led2: ElectricCurrent) -> Self {
        Self {
            led1,
            led2,
            led3: ElectricCurrent::new::<milliampere>(0.0),
            mode: core::marker::PhantomData,
        }
    }
}

/// Represents the offset currents of the LEDs.
pub struct OffsetCurrentConfiguration<MODE: LedMode> {
    led1: ElectricCurrent,
    led2: ElectricCurrent,
    ambient1: ElectricCurrent,
    ambient2_or_led3: ElectricCurrent,
    mode: core::marker::PhantomData<MODE>,
}

impl<MODE> OffsetCurrentConfiguration<MODE>
where
    MODE: LedMode,
{
    /// Gets an immutable reference of the offset current of LED1.
    pub fn led1(&self) -> &ElectricCurrent {
        &self.led1
    }

    /// Gets an immutable reference of the offset current of LED2.
    pub fn led2(&self) -> &ElectricCurrent {
        &self.led2
    }

    /// Gets a mutable reference of the offset current of LED1.
    pub fn led1_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led1
    }

    /// Gets a mutable reference of the offset current of LED2.
    pub fn led2_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led2
    }
}

impl OffsetCurrentConfiguration<ThreeLedsMode> {
    /// Creates a new `OffsetCurrentConfiguration` for the three LEDs mode.
    pub fn new(
        led1: ElectricCurrent,
        led2: ElectricCurrent,
        led3: ElectricCurrent,
        ambient: ElectricCurrent,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1: ambient,
            ambient2_or_led3: led3,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the offset current of LED3.
    pub fn led3(&self) -> &ElectricCurrent {
        &self.ambient2_or_led3
    }

    /// Gets an immutable reference of the ambient offset current.
    pub fn ambient(&self) -> &ElectricCurrent {
        &self.ambient1
    }

    /// Gets a mutable reference of the offset current of LED3.
    pub fn led3_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.ambient2_or_led3
    }

    /// Gets a mutable reference of the ambient offset current.
    pub fn ambient_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.ambient1
    }
}

impl OffsetCurrentConfiguration<TwoLedsMode> {
    /// Creates a new `OffsetCurrentConfiguration` for the two LEDs mode.
    pub fn new(
        led1: ElectricCurrent,
        led2: ElectricCurrent,
        ambient1: ElectricCurrent,
        ambient2: ElectricCurrent,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1,
            ambient2_or_led3: ambient2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the ambient1 offset current.
    pub fn ambient1(&self) -> &ElectricCurrent {
        &self.ambient1
    }

    /// Gets an immutable reference of the ambient2 offset current.
    pub fn ambient2(&self) -> &ElectricCurrent {
        &self.ambient2_or_led3
    }

    /// Gets a mutable reference of the ambient1 offset current.
    pub fn ambient1_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.ambient1
    }

    /// Gets a mutable reference of the ambient2 offset current.
    pub fn ambient2_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.ambient2_or_led3
    }
}
