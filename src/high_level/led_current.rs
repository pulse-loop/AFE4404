use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

use crate::{errors::AfeError, R22h, AFE4404};

#[derive(Debug)]
pub struct LedConfiguration {
    pub led1_current: ElectricCurrent,
    pub led2_current: ElectricCurrent,
    pub led3_current: ElectricCurrent,
}

impl<I2C> AFE4404<I2C>
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
        configuration: &LedConfiguration,
    ) -> Result<LedConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let high_current: bool = configuration.led1_current.get::<milliampere>() > 50.0
            || configuration.led2_current.get::<milliampere>() > 50.0
            || configuration.led3_current.get::<milliampere>() > 50.0;

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if configuration.led1_current > range
            || configuration.led2_current > range
            || configuration.led3_current > range
            || configuration.led1_current.get::<milliampere>() < 0.0
            || configuration.led2_current.get::<milliampere>() < 0.0
            || configuration.led3_current.get::<milliampere>() < 0.0
        {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (configuration.led1_current / quantisation).value.round() as u8,
            (configuration.led2_current / quantisation).value.round() as u8,
            (configuration.led3_current / quantisation).value.round() as u8,
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

        Ok(LedConfiguration {
            led1_current: values[0] as f32 * quantisation,
            led2_current: values[1] as f32 * quantisation,
            led3_current: values[2] as f32 * quantisation,
        })
    }

    /// Get the LEDs current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_leds_current(&mut self) -> Result<LedConfiguration, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let range = if r23h_prev.iled_2x() {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(LedConfiguration {
            led1_current: r22h_prev.iled1() as f32 * quantisation,
            led2_current: r22h_prev.iled2() as f32 * quantisation,
            led3_current: r22h_prev.iled3() as f32 * quantisation,
        })
    }
}
