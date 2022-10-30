use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_current::{microampere, milliampere};
use uom::si::f32::ElectricCurrent;

use crate::{errors::AfeError, R22h, R3Ah, AFE4404};

#[derive(Debug)]
pub struct LedCurrentConfiguration {
    pub led1: ElectricCurrent,
    pub led2: ElectricCurrent,
    pub led3: ElectricCurrent,
}

pub enum OffsetCurrentConfiguration {
    ThreeLeds {
        led1: ElectricCurrent,
        led2: ElectricCurrent,
        led3: ElectricCurrent,
        ambient: ElectricCurrent,
    },
    TwoLeds {
        led1: ElectricCurrent,
        led2: ElectricCurrent,
        ambient1: ElectricCurrent,
        ambient2: ElectricCurrent,
    },
}

impl Into<[ElectricCurrent; 4]> for OffsetCurrentConfiguration {
    fn into(self) -> [ElectricCurrent; 4] {
        match self {
            OffsetCurrentConfiguration::ThreeLeds {
                led1,
                led2,
                led3,
                ambient,
            } => [led2, led3, led1, ambient],
            OffsetCurrentConfiguration::TwoLeds {
                led1,
                led2,
                ambient1,
                ambient2,
            } => [led2, ambient2, led1, ambient1],
        }
    }
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
        configuration: &LedCurrentConfiguration,
    ) -> Result<LedCurrentConfiguration, AfeError<I2C::Error>> {
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

        Ok(LedCurrentConfiguration {
            led1: values[0] as f32 * quantisation,
            led2: values[1] as f32 * quantisation,
            led3: values[2] as f32 * quantisation,
        })
    }

    /// Get the LEDs current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_leds_current(&mut self) -> Result<LedCurrentConfiguration, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let range = if r23h_prev.iled_2x() {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(LedCurrentConfiguration {
            led1: r22h_prev.iled1() as f32 * quantisation,
            led2: r22h_prev.iled2() as f32 * quantisation,
            led3: r22h_prev.iled3() as f32 * quantisation,
        })
    }

    /// Set the offset cancellation currents.
    ///
    /// # Errors
    ///
    /// This function will return an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_current(
        &mut self,
        configuration: &OffsetCurrentConfiguration,
    ) -> Result<OffsetCurrentConfiguration, AfeError<I2C::Error>> {
        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        let values: Vec<(u8, bool)> = match *configuration {
            OffsetCurrentConfiguration::ThreeLeds {
                led1,
                led2,
                led3,
                ambient,
            } => {
                if led1 < -range
                    || led2 < -range
                    || led3 < -range
                    || ambient < -range
                    || led2 > range
                    || led1 > range
                    || led3 > range
                    || ambient > range
                {
                    return Err(AfeError::OffsetCurrentOutsideAllowedRange);
                }
                [led2, led3, led1, ambient]
            }
            OffsetCurrentConfiguration::TwoLeds {
                led1,
                led2,
                ambient1,
                ambient2,
            } => {
                if led1 < -range
                    || led2 < -range
                    || ambient1 < -range
                    || ambient2 < -range
                    || led2 > range
                    || led1 > range
                    || ambient1 > range
                    || ambient2 > range
                {
                    return Err(AfeError::OffsetCurrentOutsideAllowedRange);
                }
                [led2, ambient2, led1, ambient1]
            }
        }
        .iter()
        .map(|offset| {
            (
                (offset.abs() / quantisation).value.round() as u8,
                offset.value < 0.0,
            )
        })
        .collect();

        self.registers.r3Ah.write(
            R3Ah::new()
                .with_i_offdac_led2(values[0].0)
                .with_pol_offdac_led2(values[0].1)
                .with_i_offdac_amb2_or_i_offdac_led3(values[1].0)
                .with_pol_offdac_amb2_or_pol_offdac_led3(values[1].1)
                .with_i_offdac_led1(values[2].0)
                .with_pol_offdac_led1(values[2].1)
                .with_i_offdac_amb1(values[3].0)
                .with_pol_offdac_amb1(values[3].1),
        )?;

        Ok(match configuration {
            OffsetCurrentConfiguration::ThreeLeds {
                led1: _,
                led2: _,
                led3: _,
                ambient: _,
            } => OffsetCurrentConfiguration::ThreeLeds {
                led1: values[2].0 as f32 * quantisation * if values[2].1 { -1.0 } else { 1.0 },
                led2: values[0].0 as f32 * quantisation * if values[0].1 { -1.0 } else { 1.0 },
                led3: values[1].0 as f32 * quantisation * if values[1].1 { -1.0 } else { 1.0 },
                ambient: values[3].0 as f32 * quantisation * if values[3].1 { -1.0 } else { 1.0 },
            },
            OffsetCurrentConfiguration::TwoLeds {
                led1: _,
                led2: _,
                ambient1: _,
                ambient2: _,
            } => OffsetCurrentConfiguration::TwoLeds {
                led1: values[2].0 as f32 * quantisation * if values[2].1 { -1.0 } else { 1.0 },
                led2: values[0].0 as f32 * quantisation * if values[0].1 { -1.0 } else { 1.0 },
                ambient1: values[3].0 as f32 * quantisation * if values[3].1 { -1.0 } else { 1.0 },
                ambient2: values[1].0 as f32 * quantisation * if values[1].1 { -1.0 } else { 1.0 },
            },
        })
    }
}
