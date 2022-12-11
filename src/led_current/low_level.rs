//! This module contains the LED current and offset current low level functions.

use embedded_hal::i2c::{I2c, SevenBitAddress};
use uom::si::{
    electric_current::{microampere, milliampere},
    f32::ElectricCurrent,
};

use crate::{
    device::AFE4404,
    errors::AfeError,
    modes::{LedMode, ThreeLedsMode, TwoLedsMode},
    register_structs::R22h,
};

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Checks if the current range has changed and returns a scaled value.
    fn scale_current(reg_value: u8, prev_2x: bool, curr_2x: bool) -> u8 {
        if prev_2x == curr_2x {
            reg_value
        } else if curr_2x {
            reg_value / 2
        } else {
            reg_value * 2
        }
    }

    /// Sets the LED1 current.
    ///
    /// # Notes
    ///
    /// This function automatically expands the current range to 0-100 mA if the current is above 50 mA.
    /// When the range is expanded to 0-100 mA, the unit step is doubled from 0.8 to 1.6 mA.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range 0-100mA will result in an error.
    pub fn set_led1_current(
        &mut self,
        current: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let high_current = current.get::<milliampere>() > 50.0
            || (r23h_prev.iled_2x() && (r22h_prev.iled2() > 31 || r22h_prev.iled3() > 31));

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if current > range || current.get::<milliampere>() < 0.0 {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            (current / quantisation).value.round() as u8,
            Self::scale_current(r22h_prev.iled2(), r23h_prev.iled_2x(), high_current),
            Self::scale_current(r22h_prev.iled3(), r23h_prev.iled_2x(), high_current),
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

        Ok(f32::from(values[0]) * quantisation)
    }

    /// Sets the LED2 current.
    ///
    /// # Notes
    ///
    /// This function automatically expands the current range to 0-100 mA if the current is above 50 mA.
    /// When the range is expanded to 0-100 mA, the unit step is doubled from 0.8 to 1.6 mA.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range 0-100mA will result in an error.
    pub fn set_led2_current(
        &mut self,
        current: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let high_current = current.get::<milliampere>() > 50.0
            || (r23h_prev.iled_2x() && (r22h_prev.iled1() > 31 || r22h_prev.iled3() > 31));

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if current > range || current.get::<milliampere>() < 0.0 {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            Self::scale_current(r22h_prev.iled1(), r23h_prev.iled_2x(), high_current),
            (current / quantisation).value.round() as u8,
            Self::scale_current(r22h_prev.iled3(), r23h_prev.iled_2x(), high_current),
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

        Ok(f32::from(values[1]) * quantisation)
    }

    /// Gets the LED1 current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_led1_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let range = if r23h_prev.iled_2x() {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(f32::from(r22h_prev.iled1()) * quantisation)
    }

    /// Gets the LED2 current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_led2_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let range = if r23h_prev.iled_2x() {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(f32::from(r22h_prev.iled2()) * quantisation)
    }

    /// Sets the offset cancellation current of the LED1.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_led1_current(
        &mut self,
        offset: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if offset > range || offset < -range {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let value = (
            (offset.abs() / quantisation).value.round() as u8,
            offset.get::<microampere>() < 0.0,
        );

        self.registers.r3Ah.write(
            r3ah_prev
                .with_i_offdac_led1(value.0)
                .with_pol_offdac_led1(value.1),
        )?;

        Ok(f32::from(value.0) * quantisation * if value.1 { -1.0 } else { 1.0 })
    }

    /// Sets the offset cancellation current of the LED2.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_led2_current(
        &mut self,
        offset: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if offset > range || offset < -range {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let value = (
            (offset.abs() / quantisation).value.round() as u8,
            offset.get::<microampere>() < 0.0,
        );

        self.registers.r3Ah.write(
            r3ah_prev
                .with_i_offdac_led2(value.0)
                .with_pol_offdac_led2(value.1),
        )?;

        Ok(f32::from(value.0) * quantisation * if value.1 { -1.0 } else { 1.0 })
    }

    /// Gets the offset cancellation current of the LED1.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_led1_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(f32::from(r3ah_prev.i_offdac_led1())
            * quantisation
            * if r3ah_prev.pol_offdac_led1() {
                -1.0
            } else {
                1.0
            })
    }

    /// Gets the offset cancellation current of the LED2.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_led2_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(f32::from(r3ah_prev.i_offdac_led2())
            * quantisation
            * if r3ah_prev.pol_offdac_led2() {
                -1.0
            } else {
                1.0
            })
    }
}

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the LED3 current.
    ///
    /// # Notes
    ///
    /// This function automatically expands the current range to 0-100 mA if the current is above 50 mA.
    /// When the range is expanded to 0-100 mA, the unit step is doubled from 0.8 to 1.6 mA.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range 0-100mA will result in an error.
    pub fn set_led3_current(
        &mut self,
        current: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let high_current = current.get::<milliampere>() > 50.0
            || (r23h_prev.iled_2x() && (r22h_prev.iled1() > 31 || r22h_prev.iled2() > 31));

        let range = if high_current {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };

        let quantisation = range / 63.0;

        if current > range || current.get::<milliampere>() < 0.0 {
            return Err(AfeError::LedCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let values = [
            Self::scale_current(r22h_prev.iled1(), r23h_prev.iled_2x(), high_current),
            Self::scale_current(r22h_prev.iled2(), r23h_prev.iled_2x(), high_current),
            (current / quantisation).value.round() as u8,
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

        Ok(f32::from(values[2]) * quantisation)
    }

    /// Gets the LED3 current.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_led3_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r22h_prev = self.registers.r22h.read()?;
        let r23h_prev = self.registers.r23h.read()?;

        let range = if r23h_prev.iled_2x() {
            ElectricCurrent::new::<milliampere>(100.0)
        } else {
            ElectricCurrent::new::<milliampere>(50.0)
        };
        let quantisation = range / 63.0;

        Ok(f32::from(r22h_prev.iled3()) * quantisation)
    }

    /// Sets the offset cancellation current of the LED3.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_led3_current(
        &mut self,
        offset: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if offset > range || offset < -range {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let value = (
            (offset.abs() / quantisation).value.round() as u8,
            offset.get::<microampere>() < 0.0,
        );

        self.registers.r3Ah.write(
            r3ah_prev
                .with_i_offdac_amb2_or_i_offdac_led3(value.0)
                .with_pol_offdac_amb2_or_pol_offdac_led3(value.1),
        )?;

        Ok(f32::from(value.0) * quantisation * if value.1 { -1.0 } else { 1.0 })
    }

    /// Sets the offset cancellation current of the Ambient.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_amb_current(
        &mut self,
        offset: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if offset > range || offset < -range {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let value = (
            (offset.abs() / quantisation).value.round() as u8,
            offset.get::<microampere>() < 0.0,
        );

        self.registers.r3Ah.write(
            r3ah_prev
                .with_i_offdac_amb1(value.0)
                .with_pol_offdac_amb1(value.1),
        )?;

        Ok(f32::from(value.0) * quantisation * if value.1 { -1.0 } else { 1.0 })
    }

    /// Sets the offset cancellation current of the LED3.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_led3_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(f32::from(r3ah_prev.i_offdac_amb2_or_i_offdac_led3())
            * quantisation
            * if r3ah_prev.pol_offdac_amb2_or_pol_offdac_led3() {
                -1.0
            } else {
                1.0
            })
    }

    /// Sets the offset cancellation current of the Ambient.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_amb_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(f32::from(r3ah_prev.i_offdac_amb1())
            * quantisation
            * if r3ah_prev.pol_offdac_amb1() {
                -1.0
            } else {
                1.0
            })
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the offset cancellation current of the Ambient1.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_amb1_current(
        &mut self,
        offset: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if offset > range || offset < -range {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let value = (
            (offset.abs() / quantisation).value.round() as u8,
            offset.get::<microampere>() < 0.0,
        );

        self.registers.r3Ah.write(
            r3ah_prev
                .with_i_offdac_amb1(value.0)
                .with_pol_offdac_amb1(value.1),
        )?;

        Ok(f32::from(value.0) * quantisation * if value.1 { -1.0 } else { 1.0 })
    }

    /// Sets the offset cancellation current of the Ambient2.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a current value outside the range -7-7uA will result in an error.
    pub fn set_offset_amb2_current(
        &mut self,
        offset: ElectricCurrent,
    ) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        if offset > range || offset < -range {
            return Err(AfeError::OffsetCurrentOutsideAllowedRange);
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let value = (
            (offset.abs() / quantisation).value.round() as u8,
            offset.get::<microampere>() < 0.0,
        );

        self.registers.r3Ah.write(
            r3ah_prev
                .with_i_offdac_amb2_or_i_offdac_led3(value.0)
                .with_pol_offdac_amb2_or_pol_offdac_led3(value.1),
        )?;

        Ok(f32::from(value.0) * quantisation * if value.1 { -1.0 } else { 1.0 })
    }

    /// Sets the offset cancellation current of the Ambient1.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_amb1_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(f32::from(r3ah_prev.i_offdac_amb1())
            * quantisation
            * if r3ah_prev.pol_offdac_amb1() {
                -1.0
            } else {
                1.0
            })
    }

    /// Sets the offset cancellation current of the Ambient2.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_offset_amb2_current(&mut self) -> Result<ElectricCurrent, AfeError<I2C::Error>> {
        let r3ah_prev = self.registers.r3Ah.read()?;

        let range = ElectricCurrent::new::<microampere>(7.0);
        let quantisation = range / 15.0;

        Ok(f32::from(r3ah_prev.i_offdac_amb2_or_i_offdac_led3())
            * quantisation
            * if r3ah_prev.pol_offdac_amb2_or_pol_offdac_led3() {
                -1.0
            } else {
                1.0
            })
    }
}
