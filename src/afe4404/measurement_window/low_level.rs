//! This module contains the measurement window low level functions.

use embedded_hal::i2c::{I2c, SevenBitAddress};
use uom::si::f32::Time;

use crate::{
    afe4404::{LedMode, ThreeLedsMode, TwoLedsMode, AFE4404},
    errors::AfeError,
    register_structs::{
        R01h,
        R02h,
        R03h,
        R04h,
        R05h,
        R06h,
        R07h,
        R08h,
        R09h,
        R0Ah,
        R0Bh,
        R0Ch,
        R0Dh,
        R0Eh,
        R0Fh,
        R10h,
        R11h,
        R12h,
        R13h,
        R14h,
        R15h,
        R16h,
        R17h,
        R18h,
        R19h,
        R1Ah,
        R1Bh,
        R1Ch,
        R32h,
        R33h,
        R36h,
        R37h,
        // R1Dh, R39h,
    },
};

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Converts 'Time' into a tuple of `Time` rounded to the closest actual value and register value.
    #[allow(clippy::wrong_self_convention)]
    fn from_timing(&mut self, timing: Time) -> Result<(Time, u16), AfeError<I2C::Error>> {
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

    // TODO: Update all measurement window values in case of clk_div changes.
    // pub fn set_period(&mut self, period: Time) -> Result<Time, AfeError<I2C::Error>> {
    //     let r1eh_prev = self.registers.r1Eh.read()?;

    //     let clk_div = ((period * self.clock).value / 65536.0).ceil() as u8;
    //     let clk_div: (f32, u8) = match clk_div {
    //         1 => (1.0, 0), // (division ratio, register value).
    //         2 => (2.0, 4),
    //         d if d <= 4 => (4.0, 5),
    //         d if d <= 8 => (8.0, 6),
    //         d if d <= 16 => (16.0, 7),
    //         _ => return Err(AfeError::WindowPeriodTooLong),
    //     };
    //     let period_clk: Time = 1.0 / self.clock;
    //     let period_clk_div: Time = period_clk * clk_div.0;
    //     let counter: f32 = (period / period_clk_div).value;
    //     let counter_max_value: u16 = counter.round() as u16 - 1;
    //     let quantisation: Time = period / counter;
    // }

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
}

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
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
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
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
}
