use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::{f32::Frequency, frequency::megahertz};

use crate::{afe4404::LedMode, errors::AfeError, AFE4404};

#[derive(Debug, Clone, Copy)]
pub enum ClockConfiguration {
    Internal,
    External,
}

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Set the clock source.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting an internal clock value different from 4MHz will result in an error.
    pub fn set_clock_source(
        &mut self,
        configuration: &ClockConfiguration,
    ) -> Result<ClockConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let internal = match configuration {
            ClockConfiguration::Internal => true,
            ClockConfiguration::External => false,
        };

        if internal && self.clock != Frequency::new::<megahertz>(4.0) {
            return Err(AfeError::IncorrectInternalClock);
        }

        self.registers
            .r23h
            .write(r23h_prev.with_osc_enable(internal))?;

        Ok(*configuration)
    }

    /// Get the clock source.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_clock_source(&mut self) -> Result<ClockConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        Ok(if r23h_prev.osc_enable() {
            ClockConfiguration::Internal
        } else {
            ClockConfiguration::External
        })
    }

    // TODO: Rearrange function.
    pub fn enable_clock_out(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r29h_prev = self.registers.r29h.read()?;

        self.registers
            .r29h
            .write(r29h_prev.with_enable_clkout(true))?;

        Ok(())
    }
}
