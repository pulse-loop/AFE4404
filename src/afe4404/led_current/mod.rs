//! This module contains the LEDs current and offset current related functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_current::{microampere, milliampere};
use uom::si::f32::ElectricCurrent;

use super::AFE4404;
use crate::{
    afe4404::{ThreeLedsMode, TwoLedsMode},
    errors::AfeError,
    register_structs::{R22h, R3Ah},
};

pub use configuration::{LedCurrentConfiguration, OffsetCurrentConfiguration};

mod configuration;

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the LEDs current.
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

        let high_current: bool = configuration.led1().get::<milliampere>() > 50.0
            || configuration.led2().get::<milliampere>() > 50.0
            || configuration.led3().get::<milliampere>() > 50.0;

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if *configuration.led1() > range
            || *configuration.led2() > range
            || *configuration.led3() > range
            || configuration.led1().get::<milliampere>() < 0.0
            || configuration.led2().get::<milliampere>() < 0.0
            || configuration.led3().get::<milliampere>() < 0.0
        {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (*configuration.led1() / quantisation).value.round() as u8,
            (*configuration.led2() / quantisation).value.round() as u8,
            (*configuration.led3() / quantisation).value.round() as u8,
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
            f32::from(values[0]) * quantisation,
            f32::from(values[1]) * quantisation,
            f32::from(values[2]) * quantisation,
        ))
    }

    /// Gets the LEDs current.
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
            f32::from(r22h_prev.iled1()) * quantisation,
            f32::from(r22h_prev.iled2()) * quantisation,
            f32::from(r22h_prev.iled3()) * quantisation,
        ))
    }

    /// Sets the offset cancellation currents.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_current(
        &mut self,
        configuration: &OffsetCurrentConfiguration<ThreeLedsMode>,
    ) -> Result<OffsetCurrentConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if *configuration.led1() > range
            || *configuration.led2() > range
            || *configuration.led3() > range
            || *configuration.ambient() > range
            || *configuration.led1() < -range
            || *configuration.led2() < -range
            || *configuration.led3() < -range
            || *configuration.ambient() < -range
        {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values: [(u8, bool); 4] = [
            (
                (configuration.led1().abs() / quantisation).value.round() as u8,
                configuration.led1().value < 0.0,
            ),
            (
                (configuration.led2().abs() / quantisation).value.round() as u8,
                configuration.led2().value < 0.0,
            ),
            (
                (configuration.led3().abs() / quantisation).value.round() as u8,
                configuration.led3().value < 0.0,
            ),
            (
                (configuration.ambient().abs() / quantisation).value.round() as u8,
                configuration.ambient().value < 0.0,
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
            f32::from(values[0].0) * quantisation * if values[0].1 { -1.0 } else { 1.0 },
            f32::from(values[1].0) * quantisation * if values[1].1 { -1.0 } else { 1.0 },
            f32::from(values[2].0) * quantisation * if values[2].1 { -1.0 } else { 1.0 },
            f32::from(values[3].0) * quantisation * if values[3].1 { -1.0 } else { 1.0 },
        ))
    }

    /// Gets the offset cancellation currents.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_current(
        &mut self,
    ) -> Result<OffsetCurrentConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(OffsetCurrentConfiguration::<ThreeLedsMode>::new(
            f32::from(r3ah_prev.i_offdac_led1())
                * quantisation
                * if r3ah_prev.pol_offdac_led1() {
                    -1.0
                } else {
                    1.0
                },
            f32::from(r3ah_prev.i_offdac_led2())
                * quantisation
                * if r3ah_prev.pol_offdac_led2() {
                    -1.0
                } else {
                    1.0
                },
            f32::from(r3ah_prev.i_offdac_amb2_or_i_offdac_led3())
                * quantisation
                * if r3ah_prev.pol_offdac_amb2_or_pol_offdac_led3() {
                    -1.0
                } else {
                    1.0
                },
            f32::from(r3ah_prev.i_offdac_amb1())
                * quantisation
                * if r3ah_prev.pol_offdac_amb1() {
                    -1.0
                } else {
                    1.0
                },
        ))
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the LEDs current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_leds_current(
        &mut self,
        configuration: &LedCurrentConfiguration<TwoLedsMode>,
    ) -> Result<LedCurrentConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let high_current = *configuration.led1() > ElectricCurrent::new::<milliampere>(50.0)
            || *configuration.led2() > ElectricCurrent::new::<milliampere>(50.0);

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if *configuration.led1() > range
            || *configuration.led2() > range
            || configuration.led1().get::<milliampere>() < 0.0
            || configuration.led2().get::<milliampere>() < 0.0
        {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (*configuration.led1() / quantisation).value.round() as u8,
            (*configuration.led2() / quantisation).value.round() as u8,
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
            f32::from(values[0]) * quantisation,
            f32::from(values[1]) * quantisation,
        ))
    }

    /// Gets the LEDs current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
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
            f32::from(r22h_prev.iled1()) * quantisation,
            f32::from(r22h_prev.iled2()) * quantisation,
        ))
    }

    /// Sets the offset cancellation currents.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_current(
        &mut self,
        configuration: &OffsetCurrentConfiguration<TwoLedsMode>,
    ) -> Result<OffsetCurrentConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if *configuration.led1() > range
            || *configuration.led2() > range
            || *configuration.ambient1() > range
            || *configuration.ambient2() > range
            || *configuration.led1() < -range
            || *configuration.led2() < -range
            || *configuration.ambient1() < -range
            || *configuration.ambient2() < -range
        {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values: [(u8, bool); 4] = [
            (
                (configuration.led1().abs() / quantisation).value.round() as u8,
                configuration.led1().value < 0.0,
            ),
            (
                (configuration.led2().abs() / quantisation).value.round() as u8,
                configuration.led2().value < 0.0,
            ),
            (
                (configuration.ambient1().abs() / quantisation)
                    .value
                    .round() as u8,
                configuration.ambient1().value < 0.0,
            ),
            (
                (configuration.ambient2().abs() / quantisation)
                    .value
                    .round() as u8,
                configuration.ambient2().value < 0.0,
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
            f32::from(values[0].0) * quantisation * if values[0].1 { -1.0 } else { 1.0 },
            f32::from(values[1].0) * quantisation * if values[1].1 { -1.0 } else { 1.0 },
            f32::from(values[2].0) * quantisation * if values[2].1 { -1.0 } else { 1.0 },
            f32::from(values[3].0) * quantisation * if values[3].1 { -1.0 } else { 1.0 },
        ))
    }

    /// Gets the offset cancellation currents.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_current(
        &mut self,
    ) -> Result<OffsetCurrentConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(OffsetCurrentConfiguration::<TwoLedsMode>::new(
            f32::from(r3ah_prev.i_offdac_led1())
                * quantisation
                * if r3ah_prev.pol_offdac_led1() {
                    -1.0
                } else {
                    1.0
                },
            f32::from(r3ah_prev.i_offdac_led2())
                * quantisation
                * if r3ah_prev.pol_offdac_led2() {
                    -1.0
                } else {
                    1.0
                },
            f32::from(r3ah_prev.i_offdac_amb1())
                * quantisation
                * if r3ah_prev.pol_offdac_amb1() {
                    -1.0
                } else {
                    1.0
                },
            f32::from(r3ah_prev.i_offdac_amb2_or_i_offdac_led3())
                * quantisation
                * if r3ah_prev.pol_offdac_amb2_or_pol_offdac_led3() {
                    -1.0
                } else {
                    1.0
                },
        ))
    }
}
