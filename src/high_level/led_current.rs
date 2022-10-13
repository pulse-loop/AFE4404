use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_current::milliampere;
use uom::si::f32::ElectricCurrent;

use crate::{errors::AfeError, R22h, AFE4404};

impl<I2C> AFE4404<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Set the LED current.
    ///
    /// The current is expressed in milliamperes.
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
        led1: ElectricCurrent,
        led2: ElectricCurrent,
        led3: ElectricCurrent,
    ) -> Result<[ElectricCurrent; 3], AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let high_current: bool = led1.get::<milliampere>() > 50.0
            || led2.get::<milliampere>() > 50.0
            || led3.get::<milliampere>() > 50.0;
        
        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 64.0;

        if led1 > range
            || led2 > range
            || led3 > range
            || led1.get::<milliampere>() < 0.0
            || led2.get::<milliampere>() < 0.0
            || led3.get::<milliampere>() < 0.0
        {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (led1 / quantisation).value.round() as u8,
            (led2 / quantisation).value.round() as u8,
            (led3 / quantisation).value.round() as u8,
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

        Ok(values.map(|v| f32::from(v) * quantisation))
    }
}
