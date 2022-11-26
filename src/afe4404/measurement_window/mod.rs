//! This module contains the measurement window related functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::f32::Time;

use super::AFE4404;
use crate::{
    afe4404::{ThreeLedsMode, TwoLedsMode},
    errors::AfeError,
    register_structs::{
        R01h, R02h, R03h, R04h, R05h, R06h, R07h, R08h, R09h, R0Ah, R0Bh, R0Ch, R0Dh, R0Eh, R0Fh,
        R10h, R11h, R12h, R13h, R14h, R15h, R16h, R17h, R18h, R19h, R1Ah, R1Bh, R1Ch, R1Dh, R32h,
        R33h, R36h, R37h, R39h,
    },
};

pub use configuration::{
    ActiveTiming, AmbientTiming, LedTiming, MeasurementWindowConfiguration, PowerDownTiming,
};

mod configuration;

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    #![allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_lossless,
        clippy::too_many_lines
    )]

    /// Sets the LEDs timings.
    ///
    /// # Notes
    ///
    /// This function automatically enables the timer engine.
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a window periond too long for the current clock frequency will result in an error.
    pub fn set_timing_window(
        &mut self,
        configuration: &MeasurementWindowConfiguration<ThreeLedsMode>,
    ) -> Result<MeasurementWindowConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        struct QuantisedValues {
            led_st: u16,
            led_end: u16,
            sample_st: u16,
            sample_end: u16,
            reset_st: u16,
            reset_end: u16,
            conv_st: u16,
            conv_end: u16,
        }

        let r1eh_prev = self.registers.r1Eh.read()?;

        let clk_div = ((*configuration.period() * self.clock).value / 65536.0).ceil() as u8;
        let clk_div: (f32, u8) = match clk_div {
            1 => (1.0, 0), // (division ratio, register value).
            2 => (2.0, 4),
            d if d <= 4 => (4.0, 5),
            d if d <= 8 => (8.0, 6),
            d if d <= 16 => (16.0, 7),
            _ => return Err(AfeError::WindowPeriodTooLong),
        };
        let period_clk: Time = 1.0 / self.clock;
        let period_clk_div: Time = period_clk * clk_div.0;
        let counter: f32 = (*configuration.period() / period_clk_div).value;
        let counter_max_value: u16 = counter.round() as u16 - 1;
        let quantisation: Time = *configuration.period() / counter;

        let active_values: Vec<QuantisedValues> = [
            *configuration.active_timing_configuration().led2(),
            *configuration.active_timing_configuration().led3(),
            *configuration.active_timing_configuration().led1(),
            (*configuration.active_timing_configuration().ambient()).into(),
        ]
        .iter()
        .map(|timing| QuantisedValues {
            led_st: (timing.led_st / quantisation).value.round() as u16,
            led_end: (timing.led_end / quantisation).value.round() as u16,
            sample_st: (timing.sample_st / quantisation).value.round() as u16,
            sample_end: (timing.sample_end / quantisation).value.round() as u16,
            reset_st: (timing.reset_st / quantisation).value.round() as u16,
            reset_end: (timing.reset_end / quantisation).value.round() as u16,
            conv_st: (timing.conv_st / quantisation).value.round() as u16,
            conv_end: (timing.conv_end / quantisation).value.round() as u16,
        })
        .collect();

        let power_down_values = [
            (configuration.inactive_timing_configuration().power_down_st / quantisation)
                .value
                .round() as u16,
            (configuration.inactive_timing_configuration().power_down_end / quantisation)
                .value
                .round() as u16,
        ];

        // Enable timer engine.
        self.registers
            .r1Dh
            .write(R1Dh::new().with_prpct(counter_max_value))?;
        self.registers
            .r39h
            .write(R39h::new().with_clkdiv_prf(clk_div.1))?;
        self.registers.r1Eh.write(r1eh_prev.with_timeren(true))?;

        // Write led2 registers.
        self.registers
            .r09h
            .write(R09h::new().with_led2ledstc(active_values[0].led_st))?;
        self.registers
            .r0Ah
            .write(R0Ah::new().with_led2ledendc(active_values[0].led_end))?;
        self.registers
            .r01h
            .write(R01h::new().with_led2stc(active_values[0].sample_st))?;
        self.registers
            .r02h
            .write(R02h::new().with_led2endc(active_values[0].sample_end))?;
        self.registers
            .r15h
            .write(R15h::new().with_adcrststct0(active_values[0].reset_st))?;
        self.registers
            .r16h
            .write(R16h::new().with_adcrstendct0(active_values[0].reset_end))?;
        self.registers
            .r0Dh
            .write(R0Dh::new().with_led2convst(active_values[0].conv_st))?;
        self.registers
            .r0Eh
            .write(R0Eh::new().with_led2convend(active_values[0].conv_end))?;

        // Write led3 registers.
        self.registers
            .r36h
            .write(R36h::new().with_led3ledstc(active_values[1].led_st))?;
        self.registers
            .r37h
            .write(R37h::new().with_led3ledendc(active_values[1].led_end))?;
        self.registers
            .r05h
            .write(R05h::new().with_aled2stc_or_led3stc(active_values[1].sample_st))?;
        self.registers
            .r06h
            .write(R06h::new().with_aled2endc_or_led3endc(active_values[1].sample_end))?;
        self.registers
            .r17h
            .write(R17h::new().with_adcrststct1(active_values[1].reset_st))?;
        self.registers
            .r18h
            .write(R18h::new().with_adcrstendct1(active_values[1].reset_end))?;
        self.registers
            .r0Fh
            .write(R0Fh::new().with_aled2convst_or_led3convst(active_values[1].conv_st))?;
        self.registers
            .r10h
            .write(R10h::new().with_aled2convend_or_led3convend(active_values[1].conv_end))?;

        // Write led1 registers.
        self.registers
            .r03h
            .write(R03h::new().with_led1ledstc(active_values[2].led_st))?;
        self.registers
            .r04h
            .write(R04h::new().with_led1ledendc(active_values[2].led_end))?;
        self.registers
            .r07h
            .write(R07h::new().with_led1stc(active_values[2].sample_st))?;
        self.registers
            .r08h
            .write(R08h::new().with_led1endc(active_values[2].sample_end))?;
        self.registers
            .r19h
            .write(R19h::new().with_adcrststct2(active_values[2].reset_st))?;
        self.registers
            .r1Ah
            .write(R1Ah::new().with_adcrstendct2(active_values[2].reset_end))?;
        self.registers
            .r11h
            .write(R11h::new().with_led1convst(active_values[2].conv_st))?;
        self.registers
            .r12h
            .write(R12h::new().with_led1convend(active_values[2].conv_end))?;

        // Write ambient registers.
        self.registers
            .r0Bh
            .write(R0Bh::new().with_aled1stc(active_values[3].sample_st))?;
        self.registers
            .r0Ch
            .write(R0Ch::new().with_aled1endc(active_values[3].sample_end))?;
        self.registers
            .r1Bh
            .write(R1Bh::new().with_adcrststct3(active_values[3].reset_st))?;
        self.registers
            .r1Ch
            .write(R1Ch::new().with_adcrstendct3(active_values[3].reset_end))?;
        self.registers
            .r13h
            .write(R13h::new().with_aled1convst(active_values[3].conv_st))?;
        self.registers
            .r14h
            .write(R14h::new().with_aled1convend(active_values[3].conv_end))?;

        // Write dynamic power down registers.
        self.registers
            .r32h
            .write(R32h::new().with_pdncyclestc(power_down_values[0]))?;
        self.registers
            .r33h
            .write(R33h::new().with_pdncycleendc(power_down_values[1]))?;

        Ok(MeasurementWindowConfiguration::<ThreeLedsMode>::new(
            (counter_max_value + 1) as f32 * quantisation,
            ActiveTiming::<ThreeLedsMode>::new(
                LedTiming {
                    led_st: active_values[2].led_st as f32 * quantisation,
                    led_end: active_values[2].led_end as f32 * quantisation,
                    sample_st: active_values[2].sample_st as f32 * quantisation,
                    sample_end: active_values[2].sample_end as f32 * quantisation,
                    reset_st: active_values[2].reset_st as f32 * quantisation,
                    reset_end: active_values[2].reset_end as f32 * quantisation,
                    conv_st: active_values[2].conv_st as f32 * quantisation,
                    conv_end: active_values[2].conv_end as f32 * quantisation,
                },
                LedTiming {
                    led_st: active_values[0].led_st as f32 * quantisation,
                    led_end: active_values[0].led_end as f32 * quantisation,
                    sample_st: active_values[0].sample_st as f32 * quantisation,
                    sample_end: active_values[0].sample_end as f32 * quantisation,
                    reset_st: active_values[0].reset_st as f32 * quantisation,
                    reset_end: active_values[0].reset_end as f32 * quantisation,
                    conv_st: active_values[0].conv_st as f32 * quantisation,
                    conv_end: active_values[0].conv_end as f32 * quantisation,
                },
                LedTiming {
                    led_st: active_values[1].led_st as f32 * quantisation,
                    led_end: active_values[1].led_end as f32 * quantisation,
                    sample_st: active_values[1].sample_st as f32 * quantisation,
                    sample_end: active_values[1].sample_end as f32 * quantisation,
                    reset_st: active_values[1].reset_st as f32 * quantisation,
                    reset_end: active_values[1].reset_end as f32 * quantisation,
                    conv_st: active_values[1].conv_st as f32 * quantisation,
                    conv_end: active_values[1].conv_end as f32 * quantisation,
                },
                AmbientTiming {
                    sample_st: active_values[3].sample_st as f32 * quantisation,
                    sample_end: active_values[3].sample_end as f32 * quantisation,
                    reset_st: active_values[3].reset_st as f32 * quantisation,
                    reset_end: active_values[3].reset_end as f32 * quantisation,
                    conv_st: active_values[3].conv_st as f32 * quantisation,
                    conv_end: active_values[3].conv_end as f32 * quantisation,
                },
            ),
            PowerDownTiming {
                power_down_st: power_down_values[0] as f32 * quantisation,
                power_down_end: power_down_values[1] as f32 * quantisation,
            },
        ))
    }

    /// Gets the LEDs timings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    #[allow(clippy::similar_names)]
    pub fn get_timing_window(
        &mut self,
    ) -> Result<MeasurementWindowConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let r01h_prev = self.registers.r01h.read()?;
        let r02h_prev = self.registers.r02h.read()?;
        let r03h_prev = self.registers.r03h.read()?;
        let r04h_prev = self.registers.r04h.read()?;
        let r05h_prev = self.registers.r05h.read()?;
        let r06h_prev = self.registers.r06h.read()?;
        let r07h_prev = self.registers.r07h.read()?;
        let r08h_prev = self.registers.r08h.read()?;
        let r09h_prev = self.registers.r09h.read()?;
        let r0ah_prev = self.registers.r0Ah.read()?;
        let r0bh_prev = self.registers.r0Bh.read()?;
        let r0ch_prev = self.registers.r0Ch.read()?;
        let r0dh_prev = self.registers.r0Dh.read()?;
        let r0eh_prev = self.registers.r0Eh.read()?;
        let r0fh_prev = self.registers.r0Fh.read()?;
        let r10h_prev = self.registers.r10h.read()?;
        let r11h_prev = self.registers.r11h.read()?;
        let r12h_prev = self.registers.r12h.read()?;
        let r13h_prev = self.registers.r13h.read()?;
        let r14h_prev = self.registers.r14h.read()?;
        let r15h_prev = self.registers.r15h.read()?;
        let r16h_prev = self.registers.r16h.read()?;
        let r17h_prev = self.registers.r17h.read()?;
        let r18h_prev = self.registers.r18h.read()?;
        let r19h_prev = self.registers.r19h.read()?;
        let r1ah_prev = self.registers.r1Ah.read()?;
        let r1bh_prev = self.registers.r1Bh.read()?;
        let r1ch_prev = self.registers.r1Ch.read()?;
        let r1dh_prev = self.registers.r1Dh.read()?;
        let r32h_prev = self.registers.r32h.read()?;
        let r33h_prev = self.registers.r33h.read()?;
        let r36h_prev = self.registers.r36h.read()?;
        let r37h_prev = self.registers.r37h.read()?;
        let r39h_prev = self.registers.r39h.read()?;

        let clk_div: f32 = match r39h_prev.clkdiv_prf() {
            0 => 1.0,
            4 => 2.0,
            5 => 4.0,
            6 => 8.0,
            7 => 16.0,
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr: 0x39 }),
        };
        let period_clk_div = clk_div / self.clock;
        let period = (r1dh_prev.prpct() + 1) as f32 * period_clk_div;
        let quantisation = period_clk_div;

        Ok(MeasurementWindowConfiguration::<ThreeLedsMode>::new(
            period,
            ActiveTiming::<ThreeLedsMode>::new(
                LedTiming {
                    led_st: r03h_prev.led1ledstc() as f32 * quantisation,
                    led_end: r04h_prev.led1ledendc() as f32 * quantisation,
                    sample_st: r07h_prev.led1stc() as f32 * quantisation,
                    sample_end: r08h_prev.led1endc() as f32 * quantisation,
                    reset_st: r19h_prev.adcrststct2() as f32 * quantisation,
                    reset_end: r1ah_prev.adcrstendct2() as f32 * quantisation,
                    conv_st: r11h_prev.led1convst() as f32 * quantisation,
                    conv_end: r12h_prev.led1convend() as f32 * quantisation,
                },
                LedTiming {
                    led_st: r09h_prev.led2ledstc() as f32 * quantisation,
                    led_end: r0ah_prev.led2ledendc() as f32 * quantisation,
                    sample_st: r01h_prev.led2stc() as f32 * quantisation,
                    sample_end: r02h_prev.led2endc() as f32 * quantisation,
                    reset_st: r15h_prev.adcrststct0() as f32 * quantisation,
                    reset_end: r16h_prev.adcrstendct0() as f32 * quantisation,
                    conv_st: r0dh_prev.led2convst() as f32 * quantisation,
                    conv_end: r0eh_prev.led2convend() as f32 * quantisation,
                },
                LedTiming {
                    led_st: r36h_prev.led3ledstc() as f32 * quantisation,
                    led_end: r37h_prev.led3ledendc() as f32 * quantisation,
                    sample_st: r05h_prev.aled2stc_or_led3stc() as f32 * quantisation,
                    sample_end: r06h_prev.aled2endc_or_led3endc() as f32 * quantisation,
                    reset_st: r17h_prev.adcrststct1() as f32 * quantisation,
                    reset_end: r18h_prev.adcrstendct1() as f32 * quantisation,
                    conv_st: r0fh_prev.aled2convst_or_led3convst() as f32 * quantisation,
                    conv_end: r10h_prev.aled2convend_or_led3convend() as f32 * quantisation,
                },
                AmbientTiming {
                    sample_st: r0bh_prev.aled1stc() as f32 * quantisation,
                    sample_end: r0ch_prev.aled1endc() as f32 * quantisation,
                    reset_st: r1bh_prev.adcrststct3() as f32 * quantisation,
                    reset_end: r1ch_prev.adcrstendct3() as f32 * quantisation,
                    conv_st: r13h_prev.aled1convst() as f32 * quantisation,
                    conv_end: r14h_prev.aled1convend() as f32 * quantisation,
                },
            ),
            PowerDownTiming::new(
                r32h_prev.pdncyclestc() as f32 * quantisation,
                r33h_prev.pdncycleendc() as f32 * quantisation,
            ),
        ))
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    #![allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_lossless,
        clippy::too_many_lines
    )]

    /// Sets the LEDs timings.
    ///
    /// # Notes
    ///
    /// This function automatically enables the timer engine.
    /// After calling this function, a wait time of `tCHANNEL` should be applied before high-accuracy readings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a window periond too long for the current clock frequency will result in an error.
    pub fn set_timing_window(
        &mut self,
        configuration: &MeasurementWindowConfiguration<TwoLedsMode>,
    ) -> Result<MeasurementWindowConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        struct QuantisedValues {
            led_st: u16,
            led_end: u16,
            sample_st: u16,
            sample_end: u16,
            reset_st: u16,
            reset_end: u16,
            conv_st: u16,
            conv_end: u16,
        }

        let r1eh_prev = self.registers.r1Eh.read()?;

        let clk_div = ((*configuration.period() * self.clock).value / 65536.0).ceil() as u8;
        let clk_div: (f32, u8) = match clk_div {
            1 => (1.0, 0), // (division ratio, register value).
            2 => (2.0, 4),
            d if d <= 4 => (4.0, 5),
            d if d <= 8 => (8.0, 6),
            d if d <= 16 => (16.0, 7),
            _ => return Err(AfeError::WindowPeriodTooLong),
        };
        let period_clk: Time = 1.0 / self.clock;
        let period_clk_div: Time = period_clk * clk_div.0;
        let counter: f32 = (*configuration.period() / period_clk_div).value;
        let counter_max_value: u16 = counter.round() as u16 - 1;
        let quantisation: Time = *configuration.period() / counter;

        let active_values: Vec<QuantisedValues> = [
            *configuration.active_timing_configuration().led2(),
            (*configuration.active_timing_configuration().ambient2()).into(),
            *configuration.active_timing_configuration().led1(),
            (*configuration.active_timing_configuration().ambient1()).into(),
        ]
        .iter()
        .map(|timing| QuantisedValues {
            led_st: (timing.led_st / quantisation).value.round() as u16,
            led_end: (timing.led_end / quantisation).value.round() as u16,
            sample_st: (timing.sample_st / quantisation).value.round() as u16,
            sample_end: (timing.sample_end / quantisation).value.round() as u16,
            reset_st: (timing.reset_st / quantisation).value.round() as u16,
            reset_end: (timing.reset_end / quantisation).value.round() as u16,
            conv_st: (timing.conv_st / quantisation).value.round() as u16,
            conv_end: (timing.conv_end / quantisation).value.round() as u16,
        })
        .collect();

        let power_down_values = [
            (configuration.inactive_timing_configuration().power_down_st / quantisation)
                .value
                .round() as u16,
            (configuration.inactive_timing_configuration().power_down_end / quantisation)
                .value
                .round() as u16,
        ];

        // Enable timer engine.
        self.registers
            .r1Dh
            .write(R1Dh::new().with_prpct(counter_max_value))?;
        self.registers
            .r39h
            .write(R39h::new().with_clkdiv_prf(clk_div.1))?;
        self.registers.r1Eh.write(r1eh_prev.with_timeren(true))?;

        // Write led2 registers.
        self.registers
            .r09h
            .write(R09h::new().with_led2ledstc(active_values[0].led_st))?;
        self.registers
            .r0Ah
            .write(R0Ah::new().with_led2ledendc(active_values[0].led_end))?;
        self.registers
            .r01h
            .write(R01h::new().with_led2stc(active_values[0].sample_st))?;
        self.registers
            .r02h
            .write(R02h::new().with_led2endc(active_values[0].sample_end))?;
        self.registers
            .r15h
            .write(R15h::new().with_adcrststct0(active_values[0].reset_st))?;
        self.registers
            .r16h
            .write(R16h::new().with_adcrstendct0(active_values[0].reset_end))?;
        self.registers
            .r0Dh
            .write(R0Dh::new().with_led2convst(active_values[0].conv_st))?;
        self.registers
            .r0Eh
            .write(R0Eh::new().with_led2convend(active_values[0].conv_end))?;

        // Write ambient2 registers.
        self.registers
            .r05h
            .write(R05h::new().with_aled2stc_or_led3stc(active_values[1].sample_st))?;
        self.registers
            .r06h
            .write(R06h::new().with_aled2endc_or_led3endc(active_values[1].sample_end))?;
        self.registers
            .r17h
            .write(R17h::new().with_adcrststct1(active_values[1].reset_st))?;
        self.registers
            .r18h
            .write(R18h::new().with_adcrstendct1(active_values[1].reset_end))?;
        self.registers
            .r0Fh
            .write(R0Fh::new().with_aled2convst_or_led3convst(active_values[1].conv_st))?;
        self.registers
            .r10h
            .write(R10h::new().with_aled2convend_or_led3convend(active_values[1].conv_end))?;

        // Write led1 registers.
        self.registers
            .r03h
            .write(R03h::new().with_led1ledstc(active_values[2].led_st))?;
        self.registers
            .r04h
            .write(R04h::new().with_led1ledendc(active_values[2].led_end))?;
        self.registers
            .r07h
            .write(R07h::new().with_led1stc(active_values[2].sample_st))?;
        self.registers
            .r08h
            .write(R08h::new().with_led1endc(active_values[2].sample_end))?;
        self.registers
            .r19h
            .write(R19h::new().with_adcrststct2(active_values[2].reset_st))?;
        self.registers
            .r1Ah
            .write(R1Ah::new().with_adcrstendct2(active_values[2].reset_end))?;
        self.registers
            .r11h
            .write(R11h::new().with_led1convst(active_values[2].conv_st))?;
        self.registers
            .r12h
            .write(R12h::new().with_led1convend(active_values[2].conv_end))?;

        // Write ambient1 registers.
        self.registers
            .r0Bh
            .write(R0Bh::new().with_aled1stc(active_values[3].sample_st))?;
        self.registers
            .r0Ch
            .write(R0Ch::new().with_aled1endc(active_values[3].sample_end))?;
        self.registers
            .r1Bh
            .write(R1Bh::new().with_adcrststct3(active_values[3].reset_st))?;
        self.registers
            .r1Ch
            .write(R1Ch::new().with_adcrstendct3(active_values[3].reset_end))?;
        self.registers
            .r13h
            .write(R13h::new().with_aled1convst(active_values[3].conv_st))?;
        self.registers
            .r14h
            .write(R14h::new().with_aled1convend(active_values[3].conv_end))?;

        // Write dynamic power down registers.
        self.registers
            .r32h
            .write(R32h::new().with_pdncyclestc(power_down_values[0]))?;
        self.registers
            .r33h
            .write(R33h::new().with_pdncycleendc(power_down_values[1]))?;

        Ok(MeasurementWindowConfiguration::<TwoLedsMode>::new(
            (counter_max_value + 1) as f32 * quantisation,
            ActiveTiming::<TwoLedsMode>::new(
                LedTiming {
                    led_st: active_values[2].led_st as f32 * quantisation,
                    led_end: active_values[2].led_end as f32 * quantisation,
                    sample_st: active_values[2].sample_st as f32 * quantisation,
                    sample_end: active_values[2].sample_end as f32 * quantisation,
                    reset_st: active_values[2].reset_st as f32 * quantisation,
                    reset_end: active_values[2].reset_end as f32 * quantisation,
                    conv_st: active_values[2].conv_st as f32 * quantisation,
                    conv_end: active_values[2].conv_end as f32 * quantisation,
                },
                LedTiming {
                    led_st: active_values[0].led_st as f32 * quantisation,
                    led_end: active_values[0].led_end as f32 * quantisation,
                    sample_st: active_values[0].sample_st as f32 * quantisation,
                    sample_end: active_values[0].sample_end as f32 * quantisation,
                    reset_st: active_values[0].reset_st as f32 * quantisation,
                    reset_end: active_values[0].reset_end as f32 * quantisation,
                    conv_st: active_values[0].conv_st as f32 * quantisation,
                    conv_end: active_values[0].conv_end as f32 * quantisation,
                },
                AmbientTiming {
                    sample_st: active_values[3].sample_st as f32 * quantisation,
                    sample_end: active_values[3].sample_end as f32 * quantisation,
                    reset_st: active_values[3].reset_st as f32 * quantisation,
                    reset_end: active_values[3].reset_end as f32 * quantisation,
                    conv_st: active_values[3].conv_st as f32 * quantisation,
                    conv_end: active_values[3].conv_end as f32 * quantisation,
                },
                AmbientTiming {
                    sample_st: active_values[1].sample_st as f32 * quantisation,
                    sample_end: active_values[1].sample_end as f32 * quantisation,
                    reset_st: active_values[1].reset_st as f32 * quantisation,
                    reset_end: active_values[1].reset_end as f32 * quantisation,
                    conv_st: active_values[1].conv_st as f32 * quantisation,
                    conv_end: active_values[1].conv_end as f32 * quantisation,
                },
            ),
            PowerDownTiming {
                power_down_st: power_down_values[0] as f32 * quantisation,
                power_down_end: power_down_values[1] as f32 * quantisation,
            },
        ))
    }

    /// Gets the LEDs timings.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    #[allow(clippy::similar_names)]
    pub fn get_timing_window(
        &mut self,
    ) -> Result<MeasurementWindowConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r01h_prev = self.registers.r01h.read()?;
        let r02h_prev = self.registers.r02h.read()?;
        let r03h_prev = self.registers.r03h.read()?;
        let r04h_prev = self.registers.r04h.read()?;
        let r05h_prev = self.registers.r05h.read()?;
        let r06h_prev = self.registers.r06h.read()?;
        let r07h_prev = self.registers.r07h.read()?;
        let r08h_prev = self.registers.r08h.read()?;
        let r09h_prev = self.registers.r09h.read()?;
        let r0ah_prev = self.registers.r0Ah.read()?;
        let r0bh_prev = self.registers.r0Bh.read()?;
        let r0ch_prev = self.registers.r0Ch.read()?;
        let r0dh_prev = self.registers.r0Dh.read()?;
        let r0eh_prev = self.registers.r0Eh.read()?;
        let r0fh_prev = self.registers.r0Fh.read()?;
        let r10h_prev = self.registers.r10h.read()?;
        let r11h_prev = self.registers.r11h.read()?;
        let r12h_prev = self.registers.r12h.read()?;
        let r13h_prev = self.registers.r13h.read()?;
        let r14h_prev = self.registers.r14h.read()?;
        let r15h_prev = self.registers.r15h.read()?;
        let r16h_prev = self.registers.r16h.read()?;
        let r17h_prev = self.registers.r17h.read()?;
        let r18h_prev = self.registers.r18h.read()?;
        let r19h_prev = self.registers.r19h.read()?;
        let r1ah_prev = self.registers.r1Ah.read()?;
        let r1bh_prev = self.registers.r1Bh.read()?;
        let r1ch_prev = self.registers.r1Ch.read()?;
        let r1dh_prev = self.registers.r1Dh.read()?;
        let r32h_prev = self.registers.r32h.read()?;
        let r33h_prev = self.registers.r33h.read()?;
        let r39h_prev = self.registers.r39h.read()?;

        let clk_div: f32 = match r39h_prev.clkdiv_prf() {
            0 => 1.0,
            4 => 2.0,
            5 => 4.0,
            6 => 8.0,
            7 => 16.0,
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr: 0x39 }),
        };
        let period_clk_div = clk_div / self.clock;
        let period = (r1dh_prev.prpct() + 1) as f32 * period_clk_div;
        let quantisation = period_clk_div;

        Ok(MeasurementWindowConfiguration::<TwoLedsMode>::new(
            period,
            ActiveTiming::<TwoLedsMode>::new(
                LedTiming {
                    led_st: r03h_prev.led1ledstc() as f32 * quantisation,
                    led_end: r04h_prev.led1ledendc() as f32 * quantisation,
                    sample_st: r07h_prev.led1stc() as f32 * quantisation,
                    sample_end: r08h_prev.led1endc() as f32 * quantisation,
                    reset_st: r19h_prev.adcrststct2() as f32 * quantisation,
                    reset_end: r1ah_prev.adcrstendct2() as f32 * quantisation,
                    conv_st: r11h_prev.led1convst() as f32 * quantisation,
                    conv_end: r12h_prev.led1convend() as f32 * quantisation,
                },
                LedTiming {
                    led_st: r09h_prev.led2ledstc() as f32 * quantisation,
                    led_end: r0ah_prev.led2ledendc() as f32 * quantisation,
                    sample_st: r01h_prev.led2stc() as f32 * quantisation,
                    sample_end: r02h_prev.led2endc() as f32 * quantisation,
                    reset_st: r15h_prev.adcrststct0() as f32 * quantisation,
                    reset_end: r16h_prev.adcrstendct0() as f32 * quantisation,
                    conv_st: r0dh_prev.led2convst() as f32 * quantisation,
                    conv_end: r0eh_prev.led2convend() as f32 * quantisation,
                },
                AmbientTiming {
                    sample_st: r0bh_prev.aled1stc() as f32 * quantisation,
                    sample_end: r0ch_prev.aled1endc() as f32 * quantisation,
                    reset_st: r1bh_prev.adcrststct3() as f32 * quantisation,
                    reset_end: r1ch_prev.adcrstendct3() as f32 * quantisation,
                    conv_st: r13h_prev.aled1convst() as f32 * quantisation,
                    conv_end: r14h_prev.aled1convend() as f32 * quantisation,
                },
                AmbientTiming {
                    sample_st: r05h_prev.aled2stc_or_led3stc() as f32 * quantisation,
                    sample_end: r06h_prev.aled2endc_or_led3endc() as f32 * quantisation,
                    reset_st: r17h_prev.adcrststct1() as f32 * quantisation,
                    reset_end: r18h_prev.adcrstendct1() as f32 * quantisation,
                    conv_st: r0fh_prev.aled2convst_or_led3convst() as f32 * quantisation,
                    conv_end: r10h_prev.aled2convend_or_led3convend() as f32 * quantisation,
                },
            ),
            PowerDownTiming::new(
                r32h_prev.pdncyclestc() as f32 * quantisation,
                r33h_prev.pdncycleendc() as f32 * quantisation,
            ),
        ))
    }
}
