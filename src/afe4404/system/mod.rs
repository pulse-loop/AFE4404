//! This module contains the system related functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;

use super::AFE4404;
use crate::{afe4404::LedMode, errors::AfeError, register_structs::R00h};

pub use configuration::{DynamicConfiguration, State};

mod configuration;

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Software resets the [`AFE4404`].
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_reset(&mut self) -> Result<(), AfeError<I2C::Error>> {
        self.registers.r00h.write(R00h::new().with_sw_reset(true))?;

        Ok(())
    }

    /// Software powers down the entire [`AFE4404`].
    ///
    /// # Notes
    ///
    /// To resume the entire [`AFE4404`] call `sw_power_up()` function.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_power_down(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(r23h_prev.with_pdnafe(true))?;

        Ok(())
    }

    /// Software powers up the entire [`AFE4404`].
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

    /// Software powers down the RX portion of the [`AFE4404`].
    ///
    /// # Notes
    ///
    /// To resume the RX portion of the [`AFE4404`] call `sw_power_up_rx()` function.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_power_down_rx(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(r23h_prev.with_pdnrx(true))?;

        Ok(())
    }

    /// Software powers up the RX portion of the [`AFE4404`].
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn sw_power_up_rx(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers.r23h.write(r23h_prev.with_pdnrx(false))?;

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

    /// Sets the photodiode state.
    ///
    /// # Notes
    ///
    /// When the photodiode is disabled, the readings are determined only by the offset currents.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_photodiode(&mut self, state: State) -> Result<State, AfeError<I2C::Error>> {
        let r31h_prev = self.registers.r31h.read()?;

        self.registers
            .r31h
            .write(r31h_prev.with_pd_disconnect(state.into()))?;

        Ok(state)
    }

    /// Gets the photodiode state.
    ///
    /// # Notes
    ///
    /// When the photodiode is disabled, the readings are determined only by the offset currents.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_photodiode(&mut self) -> Result<State, AfeError<I2C::Error>> {
        let r31h_prev = self.registers.r31h.read()?;

        Ok(r31h_prev.pd_disconnect().into())
    }
}
