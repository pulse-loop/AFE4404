//! This module contains the clock related functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::{f32::Frequency, frequency::megahertz};

use crate::{device::AFE4404, errors::AfeError, modes::LedMode, register_structs::R29h};

pub use configuration::ClockConfiguration;

mod configuration;

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Sets the clock source.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting an internal clock value different from 4MHz will result in an error.
    /// Setting an output clock division ratio greater than 128 will result in an error.
    pub fn set_clock_source(
        &mut self,
        configuration: ClockConfiguration,
    ) -> Result<ClockConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let (internal, output, reg_ratio) = match configuration {
            ClockConfiguration::Internal => (true, false, 0),
            ClockConfiguration::InternalToOutput { division_ratio } => {
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                let reg_ratio = f32::from(division_ratio).log2().round() as u8;
                if reg_ratio > 7 {
                    return Err(AfeError::ClockDivisionRatioOutsideAllowedRange);
                }
                (true, true, reg_ratio)
            }
            ClockConfiguration::External => (false, false, 0),
        };

        if internal && self.clock != Frequency::new::<megahertz>(4.0) {
            return Err(AfeError::IncorrectInternalClock);
        }

        self.registers
            .r23h
            .write(r23h_prev.with_osc_enable(internal))?;

        self.registers.r29h.write(
            R29h::new()
                .with_enable_clkout(output)
                .with_clkdiv_clkout(reg_ratio),
        )?;

        Ok(match configuration {
            ClockConfiguration::Internal => ClockConfiguration::Internal,
            ClockConfiguration::InternalToOutput { division_ratio: _ } => {
                ClockConfiguration::InternalToOutput {
                    division_ratio: 2 ^ reg_ratio,
                }
            }
            ClockConfiguration::External => ClockConfiguration::External,
        })
    }

    /// Gets the clock source.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_clock_source(&mut self) -> Result<ClockConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;
        let r29h_prev = self.registers.r29h.read()?;

        Ok(if r23h_prev.osc_enable() {
            if r29h_prev.enable_clkout() {
                ClockConfiguration::InternalToOutput {
                    division_ratio: 2 ^ r29h_prev.clkdiv_clkout(),
                }
            } else {
                ClockConfiguration::Internal
            }
        } else {
            ClockConfiguration::External
        })
    }
}
