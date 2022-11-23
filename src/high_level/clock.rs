use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::{f32::Frequency, frequency::megahertz};

use crate::{afe4404::LedMode, errors::AfeError, register_structs::R29h, AFE4404};

/// Represents the clock mode of the [`AFE4404`].
#[derive(Debug, Clone, Copy)]
pub enum ClockConfiguration {
    /// The clock is driven by the internal oscillator at 4 MHz.
    Internal,
    /// The clock is driven by the internal oscillator at 4 MHz and propagated to the `CLK` pin.
    InternalToOutput {
        /// The division factor of the clock output.
        division_ratio: u8,
    },
    /// The clock is driven by an external oscillator.
    External {
        /// The frequency of the external oscillator.
        frequency: Frequency,
    },
}

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
    /// Setting a division ratio greater than 128 will result in an error.
    pub fn set_clock_source(
        &mut self,
        configuration: &ClockConfiguration,
    ) -> Result<ClockConfiguration, AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        let (internal, frequency, output, reg_ratio) = match configuration {
            ClockConfiguration::Internal => (true, Frequency::new::<megahertz>(4.0), false, 0),
            ClockConfiguration::InternalToOutput { division_ratio } => {
                let reg_ratio = (*division_ratio as f32).log2().round() as u8;
                if reg_ratio > 7 {
                    return Err(AfeError::DivisionRatioOutsideAllowedRange);
                }
                (true, Frequency::new::<megahertz>(4.0), true, reg_ratio)
            }
            ClockConfiguration::External { frequency } => (false, *frequency, false, 0),
        };

        self.clock = frequency;

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
            ClockConfiguration::External { frequency } => ClockConfiguration::External {
                frequency: *frequency,
            },
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
            ClockConfiguration::External {
                frequency: Frequency::new::<megahertz>(4.0),
            }
        })
    }
}
