use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_current::{ampere, microampere, milliampere};
use uom::si::f32::ElectricCurrent;

use crate::afe4404::{LedMode, ThreeLedsMode, TwoLedsMode};
use crate::{
    errors::AfeError,
    register_structs::{R22h, R3Ah},
    AFE4404,
};

#[derive(Debug)]
pub struct LedCurrentConfiguration<MODE: LedMode> {
    led1: ElectricCurrent,
    led2: ElectricCurrent,
    led3: ElectricCurrent,
    mode: std::marker::PhantomData<MODE>,
}

impl LedCurrentConfiguration<ThreeLedsMode> {
    pub fn new(led1: ElectricCurrent, led2: ElectricCurrent, led3: ElectricCurrent) -> Self {
        Self {
            led1,
            led2,
            led3,
            mode: std::marker::PhantomData,
        }
    }
    pub fn led1(&self) -> &ElectricCurrent {
        &self.led1
    }
    pub fn led2(&self) -> &ElectricCurrent {
        &self.led2
    }
    pub fn led3(&self) -> &ElectricCurrent {
        &self.led3
    }
    pub fn led1_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led1
    }
    pub fn led2_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led2
    }
    pub fn led3_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led3
    }
}

impl LedCurrentConfiguration<TwoLedsMode> {
    pub fn new(led1: ElectricCurrent, led2: ElectricCurrent) -> Self {
        Self {
            led1,
            led2,
            led3: ElectricCurrent::new::<ampere>(0.0),
            mode: std::marker::PhantomData,
        }
    }
    pub fn led1(&self) -> &ElectricCurrent {
        &self.led1
    }
    pub fn led2(&self) -> &ElectricCurrent {
        &self.led2
    }
    pub fn led1_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led1
    }
    pub fn led2_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led2
    }
}
pub struct OffsetCurrentConfiguration<MODE: LedMode> {
    led1: ElectricCurrent,
    led2: ElectricCurrent,
    ambient1: ElectricCurrent,
    ambient2_or_led3: ElectricCurrent,
    mode: std::marker::PhantomData<MODE>,
}

impl OffsetCurrentConfiguration<ThreeLedsMode> {
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
            mode: std::marker::PhantomData,
        }
    }
    pub fn led1(&self) -> &ElectricCurrent {
        &self.led1
    }
    pub fn led2(&self) -> &ElectricCurrent {
        &self.led2
    }
    pub fn led3(&self) -> &ElectricCurrent {
        &self.ambient2_or_led3
    }
    pub fn ambient(&self) -> &ElectricCurrent {
        &self.ambient1
    }
}

impl OffsetCurrentConfiguration<TwoLedsMode> {
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
            mode: std::marker::PhantomData,
        }
    }
    pub fn led1(&self) -> &ElectricCurrent {
        &self.led1
    }
    pub fn led2(&self) -> &ElectricCurrent {
        &self.led2
    }
    pub fn ambient1(&self) -> &ElectricCurrent {
        &self.ambient1
    }
    pub fn ambient2(&self) -> &ElectricCurrent {
        &self.ambient2_or_led3
    }
}

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Set the LEDs current.
    ///
    /// # Notes
    ///
    /// This function automatically expands the current range to 0-100 mA if any of the three currents is above 50 mA.
    /// When the range is expanded to 0-100 mA, the unit step is doubled from 0.8 to 1.6 mA.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range 0-100mA will result in an error.
    pub fn set_leds_current(
        &mut self,
        configuration: &LedCurrentConfiguration<ThreeLedsMode>,
    ) -> Result<LedCurrentConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let high_current: bool = configuration.led1.get::<milliampere>() > 50.0
            || configuration.led2.get::<milliampere>() > 50.0
            || configuration.led3.get::<milliampere>() > 50.0;

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if configuration.led1 > range
            || configuration.led2 > range
            || configuration.led3 > range
            || configuration.led1.get::<milliampere>() < 0.0
            || configuration.led2.get::<milliampere>() < 0.0
            || configuration.led3.get::<milliampere>() < 0.0
        {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (configuration.led1 / quantisation).value.round() as u8,
            (configuration.led2 / quantisation).value.round() as u8,
            (configuration.led3 / quantisation).value.round() as u8,
        ];

        self.registers.r22h.write(
            R22h::new()
                .with_iled1(values[0])
                .with_iled2(values[1])
                .with_iled3(values[2]),
        )?;
        self.registers
            .r23h
            .write(r23h_prev.with_iled_2x(high_current))?;

        Ok(LedCurrentConfiguration::<ThreeLedsMode>::new(
            values[0] as f32 * quantisation,
            values[1] as f32 * quantisation,
            values[2] as f32 * quantisation,
        ))
    }

    /// Get the LEDs current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_leds_current(
        &mut self,
    ) -> Result<LedCurrentConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let range = if r23h_prev.iled_2x() {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(LedCurrentConfiguration::<ThreeLedsMode>::new(
            r22h_prev.iled1() as f32 * quantisation,
            r22h_prev.iled2() as f32 * quantisation,
            r22h_prev.iled3() as f32 * quantisation,
        ))
    }

    /// Set the offset cancellation currents.
    ///
    /// # Errors
    ///
    /// This function will return an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_current(
        &mut self,
        configuration: &OffsetCurrentConfiguration<ThreeLedsMode>,
    ) -> Result<OffsetCurrentConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if configuration.led1 > range
            || configuration.led2 > range
            || configuration.ambient2_or_led3 > range
            || configuration.ambient1 > range
            || configuration.led1 < -range
            || configuration.led2 < -range
            || configuration.ambient2_or_led3 < -range
            || configuration.ambient1 < -range
        {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        let values: [(u8, bool); 4] = [
            (
                (configuration.led1.abs() / quantisation).value.round() as u8,
                configuration.led1.value < 0.0,
            ),
            (
                (configuration.led2.abs() / quantisation).value.round() as u8,
                configuration.led2.value < 0.0,
            ),
            (
                (configuration.ambient2_or_led3.abs() / quantisation)
                    .value
                    .round() as u8,
                configuration.ambient2_or_led3.value < 0.0,
            ),
            (
                (configuration.ambient1.abs() / quantisation).value.round() as u8,
                configuration.ambient1.value < 0.0,
            ),
        ];

        self.registers.r3Ah.write(
            R3Ah::new()
                .with_i_offdac_led1(values[0].0)
                .with_pol_offdac_led1(values[0].1)
                .with_i_offdac_led2(values[1].0)
                .with_pol_offdac_led2(values[1].1)
                .with_i_offdac_amb2_or_i_offdac_led3(values[2].0)
                .with_pol_offdac_amb2_or_pol_offdac_led3(values[2].1)
                .with_i_offdac_amb1(values[3].0)
                .with_pol_offdac_amb1(values[3].1),
        )?;
        Ok(OffsetCurrentConfiguration::<ThreeLedsMode>::new(
            values[0].0 as f32 * quantisation * if values[0].1 { -1.0 } else { 1.0 },
            values[1].0 as f32 * quantisation * if values[1].1 { -1.0 } else { 1.0 },
            values[2].0 as f32 * quantisation * if values[2].1 { -1.0 } else { 1.0 },
            values[3].0 as f32 * quantisation * if values[3].1 { -1.0 } else { 1.0 },
        ))
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Set the LEDs current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_leds_current(
        &mut self,
        configuration: &LedCurrentConfiguration<TwoLedsMode>,
    ) -> Result<LedCurrentConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let high_current = configuration.led1 > ElectricCurrent::new::<milliampere>(50.0)
            || configuration.led2 > ElectricCurrent::new::<milliampere>(50.0);

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if configuration.led1 > range
            || configuration.led2 > range
            || configuration.led1.get::<milliampere>() < 0.0
            || configuration.led2.get::<milliampere>() < 0.0
        {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (configuration.led1 / quantisation).value.round() as u8,
            (configuration.led2 / quantisation).value.round() as u8,
        ];

        self.registers.r22h.write(
            R22h::new()
                .with_iled1(values[0])
                .with_iled2(values[1])
                .with_iled3(0u8),
        )?;
        self.registers
            .r23h
            .write(r23h_prev.with_iled_2x(high_current))?;

        Ok(LedCurrentConfiguration::<TwoLedsMode>::new(
            values[0] as f32 * quantisation,
            values[1] as f32 * quantisation,
        ))
    }

    pub fn get_leds_current(
        &mut self,
    ) -> Result<LedCurrentConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let high_current = r23h_prev.iled_2x();
        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(LedCurrentConfiguration::<TwoLedsMode>::new(
            r22h_prev.iled1() as f32 * quantisation,
            r22h_prev.iled2() as f32 * quantisation,
        ))
    }

    pub fn set_offset_current(
        &mut self,
        configuration: &OffsetCurrentConfiguration<TwoLedsMode>,
    ) -> Result<OffsetCurrentConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if configuration.led1 > range
            || configuration.led2 > range
            || configuration.ambient1 > range
            || configuration.ambient2_or_led3 > range
            || configuration.led1 < -range
            || configuration.led2 < -range
            || configuration.ambient1 < -range
            || configuration.ambient2_or_led3 < -range
        {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        let values: [(u8, bool); 4] = [
            (
                (configuration.led1.abs() / quantisation).value.round() as u8,
                configuration.led1.value < 0.0,
            ),
            (
                (configuration.led2.abs() / quantisation).value.round() as u8,
                configuration.led2.value < 0.0,
            ),
            (
                (configuration.ambient1.abs() / quantisation).value.round() as u8,
                configuration.ambient1.value < 0.0,
            ),
            (
                (configuration.ambient2_or_led3.abs() / quantisation)
                    .value
                    .round() as u8,
                configuration.ambient2_or_led3.value < 0.0,
            ),
        ];

        self.registers.r3Ah.write(
            R3Ah::new()
                .with_i_offdac_led1(values[0].0)
                .with_pol_offdac_led1(values[0].1)
                .with_i_offdac_led2(values[1].0)
                .with_pol_offdac_led2(values[1].1)
                .with_i_offdac_amb1(values[2].0)
                .with_pol_offdac_amb1(values[2].1)
                .with_i_offdac_amb2_or_i_offdac_led3(values[3].0)
                .with_pol_offdac_amb2_or_pol_offdac_led3(values[3].1),
        )?;
        Ok(OffsetCurrentConfiguration::<TwoLedsMode>::new(
            values[0].0 as f32 * quantisation * if values[0].1 { -1.0 } else { 1.0 },
            values[1].0 as f32 * quantisation * if values[1].1 { -1.0 } else { 1.0 },
            values[2].0 as f32 * quantisation * if values[2].1 { -1.0 } else { 1.0 },
            values[3].0 as f32 * quantisation * if values[3].1 { -1.0 } else { 1.0 },
        ))
    }
}
