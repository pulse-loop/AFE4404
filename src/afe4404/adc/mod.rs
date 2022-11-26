//! This module contains the ADC related functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;

use super::AFE4404;
use crate::{afe4404::LedMode, errors::AfeError, register_structs::R3Dh};

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Sets the number of averages performed by the adc.
    ///
    /// # Notes
    ///
    /// When the number of averages is not a power of two the converted values will deviate from ideal values.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a number of averages greater than 16 will result in an error.
    pub fn set_averaging(&mut self, averages: u8) -> Result<u8, AfeError<I2C::Error>> {
        let r1eh_prev = self.registers.r1Eh.read()?;

        if !(1..=16).contains(&averages) {
            return Err(AfeError::NumberOfAveragesOutsideAllowedRange);
        }

        self.registers
            .r1Eh
            .write(r1eh_prev.with_numav(averages - 1))?;

        Ok(averages)
    }

    /// Gets the number of averages performed by the adc.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404<I2C>`] contains invalid data.
    pub fn get_averaging(&mut self) -> Result<u8, AfeError<I2C::Error>> {
        let r1eh_prev = self.registers.r1Eh.read()?;

        if r1eh_prev.numav() > 15 {
            return Err(AfeError::InvalidRegisterValue { reg_addr: 0x1e });
        }

        Ok(r1eh_prev.numav() + 1)
    }

    /// Sets the decimation factor.
    ///
    /// # Notes
    ///
    /// `ADC_RDY` signal period is proportional to the decimation factor.
    /// To read the averaged values call the function `read_averaged`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a wrong decimation factor will result in an error.
    pub fn set_decimation(&mut self, decimation_factor: u8) -> Result<u8, AfeError<I2C::Error>> {
        let decimation_reg: u8 = match decimation_factor {
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            _ => return Err(AfeError::DecimationFactorOutsideAllowedRange),
        };

        self.registers.r3Dh.write(
            R3Dh::new()
                .with_dec_en(decimation_factor != 1)
                .with_dec_factor(decimation_reg),
        )?;

        Ok(decimation_factor)
    }

    /// Gets the decimation factor.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404<I2C>`] contains invalid data.
    pub fn get_decimation(&mut self) -> Result<u8, AfeError<I2C::Error>> {
        let r3dh_prev = self.registers.r3Dh.read()?;

        let decimation_factor: u8 = match r3dh_prev.dec_factor() {
            0 => 1,
            1 => 2,
            2 => 4,
            3 => 8,
            4 => 16,
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr: 0x3d }),
        };

        Ok(decimation_factor)
    }
}
