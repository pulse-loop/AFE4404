use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::{errors::AfeError, register_structs::R00h, AFE4404, afe4404::LedMode};

#[derive(Clone, Copy)]
pub struct DynamicConfiguration {
    transmitter: State,
    adc: State,
    tia: State,
    rest_of_adc: State,
}

#[derive(Clone, Copy)]
pub enum State {
    Enabled,
    Disabled,
}

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Software reset the [`AFE4404<I2C>`].
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_reset(&mut self) -> Result<(), AfeError<I2C::Error>> {
        self.registers.r00h.write(R00h::new().with_sw_reset(true))?;

        Ok(())
    }

    /// Software power down the [`AFE4404<I2C>`].
    ///
    /// # Notes
    ///
    /// To resume the [`AFE4404<I2C>`] call `sw_power_up()` function.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_power_down(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(r23h_prev.with_pdnafe(true))?;

        Ok(())
    }

    /// Software power up the [`AFE4404<I2C>`].
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of tCHANNEL should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_power_up(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(r23h_prev.with_pdnafe(false))?;

        Ok(())
    }

    /// Set the functional blocks to disable during dynamic power down.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_dynamic(
        &mut self,
        configuration: &DynamicConfiguration,
    ) -> Result<DynamicConfiguration, AfeError<I2C::Error>> {
        fn is_powered_down(state: State) -> bool {
            match state {
                State::Enabled => false,
                State::Disabled => true,
            }
        }

        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(
            r23h_prev
                .with_dynamic1(is_powered_down(configuration.transmitter))
                .with_dynamic2(is_powered_down(configuration.adc))
                .with_dynamic3(is_powered_down(configuration.tia))
                .with_dynamic4(is_powered_down(configuration.rest_of_adc)),
        )?;

        Ok(configuration.clone())
    }

    /// Get the functional blocks to disable during dynamic power down.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_dynamic(&mut self) -> Result<DynamicConfiguration, AfeError<I2C::Error>> {
        fn is_powered_down(state: bool) -> State {
            match state {
                false => State::Enabled,
                true => State::Disabled,
            }
        }

        let r23h_prev = self.registers.r23h.read()?;

        Ok(DynamicConfiguration {
            transmitter: is_powered_down(r23h_prev.dynamic1()),
            adc: is_powered_down(r23h_prev.dynamic2()),
            tia: is_powered_down(r23h_prev.dynamic3()),
            rest_of_adc: is_powered_down(r23h_prev.dynamic4()),
        })
    }
}
