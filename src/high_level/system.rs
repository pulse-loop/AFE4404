use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::{afe4404::LedMode, errors::AfeError, register_structs::R00h, AFE4404};

/// Represents the dynamic blocks of the [`AFE4404<I2C>`].]
#[derive(Clone, Copy)]
pub struct DynamicConfiguration {
    transmitter: State,
    adc: State,
    tia: State,
    rest_of_adc: State,
}

/// Represents the power state of a dynamic block.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// The block is powered on.
    Enabled,
    /// The block is powered off.
    Disabled,
}

impl From<bool> for State {
    fn from(val: bool) -> Self {
        // Attention: negative logic!
        if val {
            State::Disabled
        } else {
            State::Enabled
        }
    }
}

impl From<State> for bool {
    fn from(val: State) -> Self {
        val == State::Disabled
    }
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
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_power_up(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(r23h_prev.with_pdnafe(false))?;

        Ok(())
    }

    /// Sets the functional blocks to disable during dynamic power down.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_dynamic(
        &mut self,
        configuration: &DynamicConfiguration,
    ) -> Result<DynamicConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(
            r23h_prev
                .with_dynamic1(configuration.transmitter.into())
                .with_dynamic2(configuration.adc.into())
                .with_dynamic3(configuration.tia.into())
                .with_dynamic4(configuration.rest_of_adc.into()),
        )?;

        Ok(*configuration)
    }

    /// Gets the functional blocks to disable during dynamic power down.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_dynamic(&mut self) -> Result<DynamicConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        Ok(DynamicConfiguration {
            transmitter: r23h_prev.dynamic1().into(),
            adc: r23h_prev.dynamic2().into(),
            tia: r23h_prev.dynamic3().into(),
            rest_of_adc: r23h_prev.dynamic4().into(),
        })
    }
}
