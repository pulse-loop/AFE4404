use uom::si::{f32::Time, time::microsecond};

use crate::afe4404::{LedMode, ThreeLedsMode, TwoLedsMode};

/// Represents a period of the measurement window.
#[derive(Copy, Clone, Debug)]
pub struct MeasurementWindowConfiguration<MODE: LedMode> {
    period: Time,
    active_timing_configuration: ActiveTiming<MODE>,
    inactive_timing_configuration: PowerDownTiming,
}

impl<MODE> MeasurementWindowConfiguration<MODE>
where
    MODE: LedMode,
{
    /// Creates a new measurement window configuration.
    pub fn new(
        period: Time,
        active_timing_configuration: ActiveTiming<MODE>,
        inactive_timing_configuration: PowerDownTiming,
    ) -> MeasurementWindowConfiguration<MODE> {
        MeasurementWindowConfiguration {
            period,
            active_timing_configuration,
            inactive_timing_configuration,
        }
    }

    /// Gets an immutable reference of the period of the measurement window.
    pub fn period(&self) -> &Time {
        &self.period
    }

    /// Gets an immutable reference of the active timing configuration.
    pub fn active_timing_configuration(&self) -> &ActiveTiming<MODE> {
        &self.active_timing_configuration
    }

    /// Gets an immutable reference of the inactive timing configuration.
    pub fn inactive_timing_configuration(&self) -> &PowerDownTiming {
        &self.inactive_timing_configuration
    }

    /// Gets a mutable reference of the period of the measurement window.
    pub fn period_mut(&mut self) -> &mut Time {
        &mut self.period
    }

    /// Gets a mutable reference of the active timing configuration.
    pub fn active_timing_configuration_mut(&mut self) -> &mut ActiveTiming<MODE> {
        &mut self.active_timing_configuration
    }

    /// Gets a mutable reference of the inactive timing configuration.
    pub fn inactive_timing_configuration_mut(&mut self) -> &mut PowerDownTiming {
        &mut self.inactive_timing_configuration
    }
}

/// Represents the active phase of the measurement window.
#[derive(Copy, Clone, Debug)]
pub struct ActiveTiming<MODE: LedMode> {
    led1: LedTiming,
    led2: LedTiming,
    led3: LedTiming,
    ambient1: AmbientTiming,
    ambient2: AmbientTiming,
    mode: core::marker::PhantomData<MODE>,
}

impl<MODE> ActiveTiming<MODE>
where
    MODE: LedMode,
{
    /// Gets an immutable reference of the LED1 timings.
    pub fn led1(&self) -> &LedTiming {
        &self.led1
    }

    /// Gets an immutable reference of the LED2 timings.
    pub fn led2(&self) -> &LedTiming {
        &self.led2
    }

    /// Gets a mutable reference of the LED1 timings.
    pub fn led1_mut(&mut self) -> &mut LedTiming {
        &mut self.led1
    }

    /// Gets a mutable reference of the LED2 timings.
    pub fn led2_mut(&mut self) -> &mut LedTiming {
        &mut self.led2
    }
}

impl ActiveTiming<ThreeLedsMode> {
    /// Creates a new active timing configuration.
    pub fn new(led1: LedTiming, led2: LedTiming, led3: LedTiming, ambient: AmbientTiming) -> Self {
        ActiveTiming {
            led1,
            led2,
            led3,
            ambient1: ambient,
            ambient2: AmbientTiming::default(),
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the LED3 timings.
    pub fn led3(&self) -> &LedTiming {
        &self.led3
    }

    /// Gets an immutable reference of the ambient timings.
    pub fn ambient(&self) -> &AmbientTiming {
        &self.ambient1
    }

    /// Gets a mutable reference of the LED3 timings.
    pub fn led3_mut(&mut self) -> &mut LedTiming {
        &mut self.led3
    }

    /// Gets a mutable reference of the ambient timings.
    pub fn ambient_mut(&mut self) -> &mut AmbientTiming {
        &mut self.ambient1
    }
}

impl ActiveTiming<TwoLedsMode> {
    /// Creates a new active timing configuration.
    pub fn new(
        led1: LedTiming,
        led2: LedTiming,
        ambient1: AmbientTiming,
        ambient2: AmbientTiming,
    ) -> Self {
        ActiveTiming {
            led1,
            led2,
            led3: LedTiming::default(),
            ambient1,
            ambient2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the ambient1 timings.
    pub fn ambient1(&self) -> &AmbientTiming {
        &self.ambient1
    }

    /// Gets an immutable reference of the ambient2 timings.
    pub fn ambient2(&self) -> &AmbientTiming {
        &self.ambient2
    }

    /// Gets a mutable reference of the ambient1 timings.
    pub fn ambient1_mut(&mut self) -> &mut AmbientTiming {
        &mut self.ambient1
    }

    /// Gets a mutable reference of the ambient2 timings.
    pub fn ambient2_mut(&mut self) -> &mut AmbientTiming {
        &mut self.ambient2
    }
}

/// Represents the timings of a single LED phase.
#[derive(Copy, Clone, Debug, Default)]
pub struct LedTiming {
    /// The time at which the LED is turned on.
    pub lighting_st: Time,
    /// The time at which the LED is turned off.
    pub lighting_end: Time,
    /// The time at which the ADC starts sampling.
    pub sample_st: Time,
    /// The time at which the ADC stops sampling.
    pub sample_end: Time,
    /// The time at which the ADC starts resetting.
    pub reset_st: Time,
    /// The time at which the ADC stops resetting.
    pub reset_end: Time,
    /// The time at which the ADC starts converting.
    pub conv_st: Time,
    /// The time at which the ADC stops converting.
    pub conv_end: Time,
}

/// Represents the timings of the ambient phase.
#[derive(Copy, Clone, Debug, Default)]
pub struct AmbientTiming {
    /// The time at which the ADC starts sampling.
    pub sample_st: Time,
    /// The time at which the ADC stops sampling.
    pub sample_end: Time,
    /// The time at which the ADC starts resetting.
    pub reset_st: Time,
    /// The time at which the ADC stops resetting.
    pub reset_end: Time,
    /// The time at which the ADC starts converting.
    pub conv_st: Time,
    /// The time at which the ADC stops converting.
    pub conv_end: Time,
}

impl From<AmbientTiming> for LedTiming {
    fn from(other: AmbientTiming) -> Self {
        Self {
            lighting_st: Time::new::<microsecond>(0.0),
            lighting_end: Time::new::<microsecond>(0.0),
            sample_st: other.sample_st,

            sample_end: other.sample_end,
            reset_st: other.reset_st,
            reset_end: other.reset_end,
            conv_st: other.conv_st,
            conv_end: other.conv_end,
        }
    }
}

/// Represents the inactive phase of the measurement window.
#[derive(Copy, Clone, Debug)]
pub struct PowerDownTiming {
    /// The time at which the dynamic blocks are powered down.
    pub power_down_st: Time,
    /// The time at which the dynamic blocks are powered up.
    pub power_down_end: Time,
}

impl PowerDownTiming {
    /// Creates a new power down timing configuration.
    pub fn new(power_down_st: Time, power_down_end: Time) -> Self {
        PowerDownTiming {
            power_down_st,
            power_down_end,
        }
    }
}
