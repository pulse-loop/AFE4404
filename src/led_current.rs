use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::{AFE4404, R22h};

impl<I2C> AFE4404<I2C>
    where
        I2C: I2c<SevenBitAddress>, {

    // TODO: Implement custom errors otherwise clippy fucking complains in the next function.

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
    pub fn set_leds_current(&mut self, led1: f32, led2: f32, led3: f32) -> Result<[f32; 3], ()> {
        let r23h_prev = self
            .registers
            .r23h
            .read()?;

        let high_current: bool = led1 > 50.0 || led2 > 50.0 || led3 > 50.0;
        let range = if high_current { 100.0 } else { 50.0 };
        let quantisation = range / 64.0;

        if led1 > range || led2 > range || led3 > range || led1 < 0.0 || led2 < 0.0 || led3 < 0.0 {
            return Err(());
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (led1 / quantisation).round() as u8,
            (led2 / quantisation).round() as u8,
            (led3 / quantisation).round() as u8,
        ];

        self.registers
            .r22h
            .write(
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
