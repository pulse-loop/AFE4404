//! This module contains the measurement window low level functions.

use embedded_hal::i2c::{I2c, SevenBitAddress};
use uom::si::f32::Time;

use crate::{
    afe4404::{LedMode, ThreeLedsMode, TwoLedsMode, AFE4404},
    errors::AfeError,
    register_structs::{
        R01h, R02h, R03h, R04h, R05h, R06h, R07h, R08h, R09h, R0Ah, R0Bh, R0Ch, R0Dh, R0Eh, R0Fh,
        R10h, R11h, R12h, R13h, R14h, R15h, R16h, R17h, R18h, R19h, R1Ah, R1Bh, R1Ch, R32h, R33h,
        R36h, R37h,
    },
};

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Converts a 'Time' into a tuple of `Time` rounded to the closest actual value and register value.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_timing(
        &mut self,
        timing: Time,
    ) -> Result<(Time, u16), AfeError<I2C::Error>> {
        let r39h_prev = self.registers.r39h.read()?;

        let clk_div: f32 = match r39h_prev.clkdiv_prf() {
            0 => 1.0,
            4 => 2.0,
            5 => 4.0,
            6 => 8.0,
            7 => 16.0,
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr: 0x39 }),
        };
        let quantisation = clk_div / self.clock;

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let value = (timing / quantisation).value.round() as u16;

        Ok((f32::from(value) * quantisation, value))
    }

    /// Converts a register value into a `Time`.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn into_timing(&mut self, reg_value: u16) -> Result<Time, AfeError<I2C::Error>> {
        let r39h_prev = self.registers.r39h.read()?;

        let clk_div: f32 = match r39h_prev.clkdiv_prf() {
            0 => 1.0,
            4 => 2.0,
            5 => 4.0,
            6 => 8.0,
            7 => 16.0,
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr: 0x39 }),
        };
        let quantisation = clk_div / self.clock;

        Ok(f32::from(reg_value) * quantisation)
    }

    /// Sets the LED1 lighting start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_lighting_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r03h
            .write(R03h::new().with_led1ledstc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 lighting end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_lighting_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r04h
            .write(R04h::new().with_led1ledendc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 sample start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_sample_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r07h
            .write(R07h::new().with_led1stc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 sample end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_sample_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r08h
            .write(R08h::new().with_led1endc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 reset start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_reset_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r19h
            .write(R19h::new().with_adcrststct2(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 reset end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_reset_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r1Ah
            .write(R1Ah::new().with_adcrstendct2(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 conversion start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_conv_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r11h
            .write(R11h::new().with_led1convst(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED1 conversion end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led1_conv_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r12h
            .write(R12h::new().with_led1convend(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 lighting start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_lighting_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r09h
            .write(R09h::new().with_led2ledstc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 lighting end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_lighting_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Ah
            .write(R0Ah::new().with_led2ledendc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 sample start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_sample_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r01h
            .write(R01h::new().with_led2stc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 sample end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_sample_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r02h
            .write(R02h::new().with_led2endc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 reset start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_reset_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r15h
            .write(R15h::new().with_adcrststct0(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 reset end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_reset_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r16h
            .write(R16h::new().with_adcrstendct0(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 conversion start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_conv_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Dh
            .write(R0Dh::new().with_led2convst(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED2 conversion end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led2_conv_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Eh
            .write(R0Eh::new().with_led2convend(value.1))?;

        Ok(value.0)
    }

    /// Sets the dynamic power down start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_dynamic_power_down_st(
        &mut self,
        timing: Time,
    ) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r32h
            .write(R32h::new().with_pdncyclestc(value.1))?;

        Ok(value.0)
    }

    /// Sets the dynamic power down end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_dynamic_power_down_end(
        &mut self,
        timing: Time,
    ) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r33h
            .write(R33h::new().with_pdncycleendc(value.1))?;

        Ok(value.0)
    }

    /// Gets the LED1 lighting start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_lighting_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r03h_prev = self.registers.r03h.read()?;

        let value = self.into_timing(r03h_prev.led1ledstc())?;

        Ok(value)
    }

    /// Gets the LED1 lighting end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_lighting_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r04h_prev = self.registers.r04h.read()?;

        let value = self.into_timing(r04h_prev.led1ledendc())?;

        Ok(value)
    }

    /// Gets the LED1 sample start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_sample_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r07h_prev = self.registers.r07h.read()?;

        let value = self.into_timing(r07h_prev.led1stc())?;

        Ok(value)
    }

    /// Gets the LED1 sample end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_sample_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r08h_prev = self.registers.r08h.read()?;

        let value = self.into_timing(r08h_prev.led1endc())?;

        Ok(value)
    }

    /// Gets the LED1 reset start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_reset_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r19h_prev = self.registers.r19h.read()?;

        let value = self.into_timing(r19h_prev.adcrststct2())?;

        Ok(value)
    }

    /// Gets the LED1 reset end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_reset_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r1ah_prev = self.registers.r1Ah.read()?;

        let value = self.into_timing(r1ah_prev.adcrstendct2())?;

        Ok(value)
    }

    /// Gets the LED1 conversion start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_conv_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r11h_prev = self.registers.r11h.read()?;

        let value = self.into_timing(r11h_prev.led1convst())?;

        Ok(value)
    }

    /// Gets the LED1 conversion end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led1_conv_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r12h_prev = self.registers.r12h.read()?;

        let value = self.into_timing(r12h_prev.led1convend())?;

        Ok(value)
    }

    /// Gets the LED2 lighting start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_lighting_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r09h_prev = self.registers.r09h.read()?;

        let value = self.into_timing(r09h_prev.led2ledstc())?;

        Ok(value)
    }

    /// Gets the LED2 lighting end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_lighting_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0ah_prev = self.registers.r0Ah.read()?;

        let value = self.into_timing(r0ah_prev.led2ledendc())?;

        Ok(value)
    }

    /// Gets the LED2 sample start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_sample_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r01h_prev = self.registers.r01h.read()?;

        let value = self.into_timing(r01h_prev.led2stc())?;

        Ok(value)
    }

    /// Gets the LED2 sample end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_sample_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r02h_prev = self.registers.r02h.read()?;

        let value = self.into_timing(r02h_prev.led2endc())?;

        Ok(value)
    }

    /// Gets the LED2 reset start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_reset_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r15h_prev = self.registers.r15h.read()?;

        let value = self.into_timing(r15h_prev.adcrststct0())?;

        Ok(value)
    }

    /// Gets the LED2 reset end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_reset_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r16h_prev = self.registers.r16h.read()?;

        let value = self.into_timing(r16h_prev.adcrstendct0())?;

        Ok(value)
    }

    /// Gets the LED2 conversion start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_conv_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0dh_prev = self.registers.r0Dh.read()?;

        let value = self.into_timing(r0dh_prev.led2convst())?;

        Ok(value)
    }

    /// Gets the LED2 conversion end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led2_conv_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0eh_prev = self.registers.r0Eh.read()?;

        let value = self.into_timing(r0eh_prev.led2convend())?;

        Ok(value)
    }

    /// Gets the dynamic power down start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_dynamic_power_down_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r32h_prev = self.registers.r32h.read()?;

        let value = self.into_timing(r32h_prev.pdncyclestc())?;

        Ok(value)
    }

    /// Gets the dynamic power down end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_dynamic_power_down_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r33h_prev = self.registers.r33h.read()?;

        let value = self.into_timing(r33h_prev.pdncycleendc())?;

        Ok(value)
    }
}

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the window period.
    ///
    /// # Errors
    ///
    ///
    pub fn set_window_period(&mut self, period: Time) -> Result<Time, AfeError<I2C::Error>> {
        let mut configuration_prev = self.get_measurement_window()?;

        *configuration_prev.period_mut() = period;

        let configuration = self.set_measurement_window(&configuration_prev)?;

        Ok(*configuration.period())
    }

    /// Sets the LED3 lighting start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_lighting_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r36h
            .write(R36h::new().with_led3ledstc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 lighting end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_lighting_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r37h
            .write(R37h::new().with_led3ledendc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 sample start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_sample_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r05h
            .write(R05h::new().with_aled2stc_or_led3stc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 sample end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_sample_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r06h
            .write(R06h::new().with_aled2endc_or_led3endc(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 reset start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_reset_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r17h
            .write(R17h::new().with_adcrststct1(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 reset end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_reset_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r18h
            .write(R18h::new().with_adcrstendct1(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 conversion start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_conv_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Fh
            .write(R0Fh::new().with_aled2convst_or_led3convst(value.1))?;

        Ok(value.0)
    }

    /// Sets the LED3 conversion end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_led3_conv_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r10h
            .write(R10h::new().with_aled2convend_or_led3convend(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient sample start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient_sample_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Bh
            .write(R0Bh::new().with_aled1stc(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient sample end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient_sample_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Ch
            .write(R0Ch::new().with_aled1endc(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient reset start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient_reset_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r1Bh
            .write(R1Bh::new().with_adcrststct3(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient reset end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient_reset_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r1Ch
            .write(R1Ch::new().with_adcrstendct3(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient conversion start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient_conv_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r13h
            .write(R13h::new().with_aled1convst(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient conversion end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient_conv_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r14h
            .write(R14h::new().with_aled1convend(value.1))?;

        Ok(value.0)
    }

    /// Gets the LED3 lighting start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_lighting_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r36h_prev = self.registers.r36h.read()?;

        let value = self.into_timing(r36h_prev.led3ledstc())?;

        Ok(value)
    }

    /// Gets the LED3 lighting end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_lighting_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r37h_prev = self.registers.r37h.read()?;

        let value = self.into_timing(r37h_prev.led3ledendc())?;

        Ok(value)
    }

    /// Gets the LED3 sample start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_sample_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r05h_prev = self.registers.r05h.read()?;

        let value = self.into_timing(r05h_prev.aled2stc_or_led3stc())?;

        Ok(value)
    }

    /// Gets the LED3 sample end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_sample_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r06h_prev = self.registers.r06h.read()?;

        let value = self.into_timing(r06h_prev.aled2endc_or_led3endc())?;

        Ok(value)
    }

    /// Gets the LED3 reset start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_reset_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r17h_prev = self.registers.r17h.read()?;

        let value = self.into_timing(r17h_prev.adcrststct1())?;

        Ok(value)
    }

    /// Gets the LED3 reset end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_reset_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r18h_prev = self.registers.r18h.read()?;

        let value = self.into_timing(r18h_prev.adcrstendct1())?;

        Ok(value)
    }

    /// Gets the LED3 conversion start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_conv_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0fh_prev = self.registers.r0Fh.read()?;

        let value = self.into_timing(r0fh_prev.aled2convst_or_led3convst())?;

        Ok(value)
    }

    /// Gets the LED3 conversion end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_led3_conv_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r10h_prev = self.registers.r10h.read()?;

        let value = self.into_timing(r10h_prev.aled2convend_or_led3convend())?;

        Ok(value)
    }

    /// Gets the Ambient sample start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient_sample_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0bh_prev = self.registers.r0Bh.read()?;

        let value = self.into_timing(r0bh_prev.aled1stc())?;

        Ok(value)
    }

    /// Gets the Ambient sample end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient_sample_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0ch_prev = self.registers.r0Ch.read()?;

        let value = self.into_timing(r0ch_prev.aled1endc())?;

        Ok(value)
    }

    /// Gets the Ambient reset start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient_reset_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r1bh_prev = self.registers.r1Bh.read()?;

        let value = self.into_timing(r1bh_prev.adcrststct3())?;

        Ok(value)
    }

    /// Gets the Ambient reset end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient_reset_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r1ch_prev = self.registers.r1Ch.read()?;

        let value = self.into_timing(r1ch_prev.adcrstendct3())?;

        Ok(value)
    }

    /// Gets the Ambient conversion start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient_conv_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r13h_prev = self.registers.r13h.read()?;

        let value = self.into_timing(r13h_prev.aled1convst())?;

        Ok(value)
    }

    /// Gets the Ambient conversion end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient_conv_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r14h_prev = self.registers.r14h.read()?;

        let value = self.into_timing(r14h_prev.aled1convend())?;

        Ok(value)
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the window period.
    ///
    /// # Errors
    ///
    ///
    pub fn set_window_period(&mut self, period: Time) -> Result<Time, AfeError<I2C::Error>> {
        let mut configuration_prev = self.get_measurement_window()?;

        *configuration_prev.period_mut() = period;

        let configuration = self.set_measurement_window(&configuration_prev)?;

        Ok(*configuration.period())
    }

    /// Sets the Ambient1 sample start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient1_sample_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Bh
            .write(R0Bh::new().with_aled1stc(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient1 sample end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient1_sample_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Ch
            .write(R0Ch::new().with_aled1endc(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient1 reset start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient1_reset_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r1Bh
            .write(R1Bh::new().with_adcrststct3(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient1 reset end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient1_reset_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r1Ch
            .write(R1Ch::new().with_adcrstendct3(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient1 conversion start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient1_conv_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r13h
            .write(R13h::new().with_aled1convst(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient1 conversion end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient1_conv_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r14h
            .write(R14h::new().with_aled1convend(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient2 sample start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient2_sample_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r05h
            .write(R05h::new().with_aled2stc_or_led3stc(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient2 sample end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient2_sample_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r06h
            .write(R06h::new().with_aled2endc_or_led3endc(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient2 reset start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient2_reset_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r17h
            .write(R17h::new().with_adcrststct1(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient2 reset end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient2_reset_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r18h
            .write(R18h::new().with_adcrstendct1(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient2 conversion start timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient2_conv_st(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r0Fh
            .write(R0Fh::new().with_aled2convst_or_led3convst(value.1))?;

        Ok(value.0)
    }

    /// Sets the Ambient2 conversion end timing.
    ///
    /// # Notes
    ///
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn set_ambient2_conv_end(&mut self, timing: Time) -> Result<Time, AfeError<I2C::Error>> {
        let value = self.from_timing(timing)?;

        self.registers
            .r10h
            .write(R10h::new().with_aled2convend_or_led3convend(value.1))?;

        Ok(value.0)
    }

    /// Gets the Ambient1 sample start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient1_sample_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0bh_prev = self.registers.r0Bh.read()?;

        let value = self.into_timing(r0bh_prev.aled1stc())?;

        Ok(value)
    }

    /// Gets the Ambient1 sample end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient1_sample_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0ch_prev = self.registers.r0Ch.read()?;

        let value = self.into_timing(r0ch_prev.aled1endc())?;

        Ok(value)
    }

    /// Gets the Ambient1 reset start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient1_reset_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r1bh_prev = self.registers.r1Bh.read()?;

        let value = self.into_timing(r1bh_prev.adcrststct3())?;

        Ok(value)
    }

    /// Gets the Ambient1 reset end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient1_reset_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r1ch_prev = self.registers.r1Ch.read()?;

        let value = self.into_timing(r1ch_prev.adcrstendct3())?;

        Ok(value)
    }

    /// Gets the Ambient1 conversion start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient1_conv_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r13h_prev = self.registers.r13h.read()?;

        let value = self.into_timing(r13h_prev.aled1convst())?;

        Ok(value)
    }

    /// Gets the Ambient1 conversion end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient1_conv_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r14h_prev = self.registers.r14h.read()?;

        let value = self.into_timing(r14h_prev.aled1convend())?;

        Ok(value)
    }

    /// Gets the Ambient2 sample start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient2_sample_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r05h_prev = self.registers.r05h.read()?;

        let value = self.into_timing(r05h_prev.aled2stc_or_led3stc())?;

        Ok(value)
    }

    /// Gets the Ambient2 sample end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient2_sample_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r06h_prev = self.registers.r06h.read()?;

        let value = self.into_timing(r06h_prev.aled2endc_or_led3endc())?;

        Ok(value)
    }

    /// Gets the Ambient2 reset start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient2_reset_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r17h_prev = self.registers.r17h.read()?;

        let value = self.into_timing(r17h_prev.adcrststct1())?;

        Ok(value)
    }

    /// Gets the Ambient2 reset end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient2_reset_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r18h_prev = self.registers.r18h.read()?;

        let value = self.into_timing(r18h_prev.adcrstendct1())?;

        Ok(value)
    }

    /// Gets the Ambient2 conversion start timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient2_conv_st(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r0fh_prev = self.registers.r0Fh.read()?;

        let value = self.into_timing(r0fh_prev.aled2convst_or_led3convst())?;

        Ok(value)
    }

    /// Gets the Ambient2 conversion end timing.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_ambient2_conv_end(&mut self) -> Result<Time, AfeError<I2C::Error>> {
        let r10h_prev = self.registers.r10h.read()?;

        let value = self.into_timing(r10h_prev.aled2convend_or_led3convend())?;

        Ok(value)
    }
}
