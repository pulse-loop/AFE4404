//! This module contains the low level functions for reading values from the AFE4404.

use crate::{device::AFE4404, errors::AfeError, modes::LedMode};

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Reads the LED1 sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_led1(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2ch_prev = self.registers.r2Ch.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2ch_prev.led1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED2 sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_led2(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2ah_prev = self.registers.r2Ah.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2ah_prev.led2val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }
}

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Reads the LED3 sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_led3(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2bh_prev = self.registers.r2Bh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2bh_prev.aled2val_or_led3val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the Ambient sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_ambient(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2dh_prev = self.registers.r2Dh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2dh_prev.aled1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED1 minus Ambient sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_led1_minus_ambient(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2fh_prev = self.registers.r2Fh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2fh_prev.led1_minus_aled1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED1 minus Ambient value averaged over a number of samples set by the `decimation_factor`.
    ///
    /// # Notes
    ///
    /// When the decimation factor is greater than one, call this function after an `ADC_RDY` pulse, data will remain valid untill next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_averaged_led1_minus_ambient(
        &mut self,
    ) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r40h_prev = self.registers.r40h.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r40h_prev.avg_led1_minus_aled1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Reads the Ambient1 sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_ambient1(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2dh_prev = self.registers.r2Dh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2dh_prev.aled1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the Ambient2 sampled value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_ambient2(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2bh_prev = self.registers.r2Bh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2bh_prev.aled2val_or_led3val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED1 minus Ambient1 value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_led1_minus_ambient1(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2fh_prev = self.registers.r2Fh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2fh_prev.led1_minus_aled1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED2 minus Ambient2 value.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid until next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_led2_minus_ambient2(&mut self) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r2eh_prev = self.registers.r2Eh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r2eh_prev.led2_minus_aled2val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED1 minus Ambient1 value averaged over a number of samples set by the `decimation_factor`.
    ///
    /// # Notes
    ///
    /// When the decimation factor is greater than one, call this function after an `ADC_RDY` pulse, data will remain valid untill next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_averaged_led1_minus_ambient1(
        &mut self,
    ) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r40h_prev = self.registers.r40h.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r40h_prev.avg_led1_minus_aled1val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }

    /// Reads the LED2 minus Ambient2 value averaged over a number of samples set by the `decimation_factor`.
    ///
    /// # Notes
    ///
    /// When the decimation factor is greater than one, call this function after an `ADC_RDY` pulse, data will remain valid untill next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read_averaged_led2_minus_ambient2(
        &mut self,
    ) -> Result<ElectricPotential, AfeError<I2C::Error>> {
        let r41h_prev = self.registers.r41h.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        let sign_extension_bits = ((r41h_prev.avg_led2_minus_aled2val() & 0x00FF_FFFF) >> 21) as u8;
        let signed_value = match sign_extension_bits {
            0b000 => register_value as i32, // The value is positive.
            0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
            _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
        };

        Ok(signed_value as f32 * quantisation);
    }
}
