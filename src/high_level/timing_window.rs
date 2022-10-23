use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::{f32::Time, time::{second, microsecond}};

use crate::{
    errors::AfeError, R01h, R02h, R03h, R04h, R05h, R06h, R07h, R08h, R09h, R0Ah, R0Bh, R0Ch, R0Dh,
    R0Eh, R0Fh, R10h, R11h, R12h, R13h, R14h, R15h, R16h, R17h, R18h, R19h, R1Ah, R1Bh, R1Ch, R1Dh,
    R32h, R33h, R36h, R37h, R39h, AFE4404,
};

#[derive(Debug)]
pub struct MeasurementWindowConfiguration {
    pub period: Time,
    pub active_timing_configuration: ActiveTimingConfiguration,
    pub inactive_timing: PowerDownTiming,
}

impl Default for MeasurementWindowConfiguration {
    fn default() -> Self {
        MeasurementWindowConfiguration {
            period: Time::new::<microsecond>(10_000.0),
            active_timing_configuration: ActiveTimingConfiguration::ThreeLeds {
                led2: LedTiming {
                    led_st: Time::new::<microsecond>(0.0),
                    led_end: Time::new::<microsecond>(99.75),
                    sample_st: Time::new::<microsecond>(25.0),
                    sample_end: Time::new::<microsecond>(99.75),
                    reset_st: Time::new::<microsecond>(100.25),
                    reset_end: Time::new::<microsecond>(101.75),
                    conv_st: Time::new::<microsecond>(102.25),
                    conv_end: Time::new::<microsecond>(367.0),
                },
                led3: LedTiming {
                    led_st: Time::new::<microsecond>(100.25),
                    led_end: Time::new::<microsecond>(200.0),
                    sample_st: Time::new::<microsecond>(125.25),
                    sample_end: Time::new::<microsecond>(200.0),
                    reset_st: Time::new::<microsecond>(367.5),
                    reset_end: Time::new::<microsecond>(369.0),
                    conv_st: Time::new::<microsecond>(369.5),
                    conv_end: Time::new::<microsecond>(634.25),
                },
                led1: LedTiming {
                    led_st: Time::new::<microsecond>(200.5),
                    led_end: Time::new::<microsecond>(300.25),
                    sample_st: Time::new::<microsecond>(225.5),
                    sample_end: Time::new::<microsecond>(300.25),
                    reset_st: Time::new::<microsecond>(634.75),
                    reset_end: Time::new::<microsecond>(636.25),
                    conv_st: Time::new::<microsecond>(636.75),
                    conv_end: Time::new::<microsecond>(901.5),
                },
                ambient: AmbientTiming {
                    sample_st: Time::new::<microsecond>(325.75),
                    sample_end: Time::new::<microsecond>(400.5),
                    reset_st: Time::new::<microsecond>(902.0),
                    reset_end: Time::new::<microsecond>(903.5),
                    conv_st: Time::new::<microsecond>(904.0),
                    conv_end: Time::new::<microsecond>(1168.75),
                },
            },
            inactive_timing: PowerDownTiming {
                power_down_st: Time::new::<microsecond>(1368.75),
                power_down_end: Time::new::<microsecond>(9799.75),
            },
        }
    }
}

#[derive(Debug)]
pub enum ActiveTimingConfiguration {
    ThreeLeds {
        led2: LedTiming,
        led3: LedTiming,
        led1: LedTiming,
        ambient: AmbientTiming,
    },
    TwoLeds {
        led2: LedTiming,
        ambient2: AmbientTiming,
        led1: LedTiming,
        ambient1: AmbientTiming,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct LedTiming {
    pub led_st: Time,
    pub led_end: Time,
    pub sample_st: Time,
    pub sample_end: Time,
    pub reset_st: Time,
    pub reset_end: Time,
    pub conv_st: Time,
    pub conv_end: Time,
}

#[derive(Clone, Copy, Debug)]
pub struct AmbientTiming {
    pub sample_st: Time,
    pub sample_end: Time,
    pub reset_st: Time,
    pub reset_end: Time,
    pub conv_st: Time,
    pub conv_end: Time,
}

impl From<AmbientTiming> for LedTiming {
    fn from(other: AmbientTiming) -> Self {
        Self {
            led_st: Time::new::<second>(0.0),
            led_end: Time::new::<second>(0.0),
            sample_st: other.sample_st,
            sample_end: other.sample_end,
            reset_st: other.reset_st,
            reset_end: other.reset_end,
            conv_st: other.conv_st,
            conv_end: other.conv_end,
        }
    }
}

#[derive(Debug)]
pub struct PowerDownTiming {
    pub power_down_st: Time,
    pub power_down_end: Time,
}

impl<I2C> AFE4404<I2C>
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

    /// Set the LEDs timings.
    ///
    ///
    /// # Notes
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_timing_window(
        &mut self,
        configuration: &MeasurementWindowConfiguration,
    ) -> Result<MeasurementWindowConfiguration, AfeError<I2C::Error>> {
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

        let clk_div = ((configuration.period * self.clock).value / 65536.0).ceil() as u8;
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
        let counter: f32 = (configuration.period / period_clk_div).value;
        let counter_max_value: u16 = counter.round() as u16 - 1;
        let quantisation: Time = configuration.period / counter;

        let values: Vec<QuantisedValues> = match configuration.active_timing_configuration {
            ActiveTimingConfiguration::ThreeLeds {
                led1,
                led2,
                led3,
                ambient,
            } => [led2, led3, led1, ambient.into()],
            ActiveTimingConfiguration::TwoLeds {
                led1,
                led2,
                ambient1,
                ambient2,
            } => [led2, ambient2.into(), led1, ambient1.into()],
        }
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
            (configuration.inactive_timing.power_down_st / quantisation)
                .value
                .round() as u16,
            (configuration.inactive_timing.power_down_end / quantisation)
                .value
                .round() as u16,
        ];

        self.registers
            .r1Dh
            .write(R1Dh::new().with_prpct(counter_max_value))?;
        self.registers
            .r39h
            .write(R39h::new().with_clkdiv_prf(clk_div.1))?;

        // Write led2 registers.
        self.registers
            .r09h
            .write(R09h::new().with_led2ledstc(values[0].led_st))?;
        self.registers
            .r0Ah
            .write(R0Ah::new().with_led2ledendc(values[0].led_end))?;
        self.registers
            .r01h
            .write(R01h::new().with_led2stc(values[0].sample_st))?;
        self.registers
            .r02h
            .write(R02h::new().with_led2endc(values[0].sample_end))?;
        self.registers
            .r15h
            .write(R15h::new().with_adcrststct0(values[0].reset_st))?;
        self.registers
            .r16h
            .write(R16h::new().with_adcrstendct0(values[0].reset_end))?;
        self.registers
            .r0Dh
            .write(R0Dh::new().with_led2convst(values[0].conv_st))?;
        self.registers
            .r0Eh
            .write(R0Eh::new().with_led2convend(values[0].conv_end))?;

        // Write ambient2 or led3 registers.
        self.registers
            .r36h
            .write(R36h::new().with_led3ledstc(values[1].led_st))?;
        self.registers
            .r37h
            .write(R37h::new().with_led3ledendc(values[1].led_end))?;
        self.registers
            .r05h
            .write(R05h::new().with_aled2stc_or_led3stc(values[1].sample_st))?;
        self.registers
            .r06h
            .write(R06h::new().with_aled2endc_or_led3endc(values[1].sample_end))?;
        self.registers
            .r17h
            .write(R17h::new().with_adcrststct1(values[1].reset_st))?;
        self.registers
            .r18h
            .write(R18h::new().with_adcrstendct1(values[1].reset_end))?;
        self.registers
            .r0Fh
            .write(R0Fh::new().with_aled2convst_or_led3convst(values[1].conv_st))?;
        self.registers
            .r10h
            .write(R10h::new().with_aled2convend_or_led3convend(values[1].conv_end))?;

        // Write led1 registers.
        self.registers
            .r03h
            .write(R03h::new().with_led1ledstc(values[2].led_st))?;
        self.registers
            .r04h
            .write(R04h::new().with_led1ledendc(values[2].led_end))?;
        self.registers
            .r07h
            .write(R07h::new().with_led1stc(values[2].sample_st))?;
        self.registers
            .r08h
            .write(R08h::new().with_led1endc(values[2].sample_end))?;
        self.registers
            .r19h
            .write(R19h::new().with_adcrststct2(values[2].reset_st))?;
        self.registers
            .r1Ah
            .write(R1Ah::new().with_adcrstendct2(values[2].reset_end))?;
        self.registers
            .r11h
            .write(R11h::new().with_led1convst(values[2].conv_st))?;
        self.registers
            .r12h
            .write(R12h::new().with_led1convend(values[2].conv_end))?;

        // Write ambient1 registers.
        self.registers
            .r0Bh
            .write(R0Bh::new().with_aled1stc(values[3].sample_st))?;
        self.registers
            .r0Ch
            .write(R0Ch::new().with_aled1endc(values[3].sample_end))?;
        self.registers
            .r1Bh
            .write(R1Bh::new().with_adcrststct3(values[3].reset_st))?;
        self.registers
            .r1Ch
            .write(R1Ch::new().with_adcrstendct3(values[3].reset_end))?;
        self.registers
            .r13h
            .write(R13h::new().with_aled1convst(values[3].conv_st))?;
        self.registers
            .r14h
            .write(R14h::new().with_aled1convend(values[3].conv_end))?;

        // Write dynamic power down registers.
        self.registers
            .r32h
            .write(R32h::new().with_pdncyclestc(power_down_values[0]))?;
        self.registers
            .r33h
            .write(R33h::new().with_pdncycleendc(power_down_values[1]))?;

        Ok(MeasurementWindowConfiguration {
            period: (counter_max_value + 1) as f32 * quantisation,
            active_timing_configuration: if values[1].led_st == 0 && values[1].led_end == 0 {
                ActiveTimingConfiguration::TwoLeds {
                    led2: LedTiming {
                        led_st: values[0].led_st as f32 * quantisation,
                        led_end: values[0].led_end as f32 * quantisation,
                        sample_st: values[0].sample_st as f32 * quantisation,
                        sample_end: values[0].sample_end as f32 * quantisation,
                        reset_st: values[0].reset_st as f32 * quantisation,
                        reset_end: values[0].reset_end as f32 * quantisation,
                        conv_st: values[0].conv_st as f32 * quantisation,
                        conv_end: values[0].conv_end as f32 * quantisation,
                    },
                    ambient2: AmbientTiming {
                        sample_st: values[1].sample_st as f32 * quantisation,
                        sample_end: values[1].sample_end as f32 * quantisation,
                        reset_st: values[1].reset_st as f32 * quantisation,
                        reset_end: values[1].reset_end as f32 * quantisation,
                        conv_st: values[1].conv_st as f32 * quantisation,
                        conv_end: values[1].conv_end as f32 * quantisation,
                    },
                    led1: LedTiming {
                        led_st: values[2].led_st as f32 * quantisation,
                        led_end: values[2].led_end as f32 * quantisation,
                        sample_st: values[2].sample_st as f32 * quantisation,
                        sample_end: values[2].sample_end as f32 * quantisation,
                        reset_st: values[2].reset_st as f32 * quantisation,
                        reset_end: values[2].reset_end as f32 * quantisation,
                        conv_st: values[2].conv_st as f32 * quantisation,
                        conv_end: values[2].conv_end as f32 * quantisation,
                    },
                    ambient1: AmbientTiming {
                        sample_st: values[3].sample_st as f32 * quantisation,
                        sample_end: values[3].sample_end as f32 * quantisation,
                        reset_st: values[3].reset_st as f32 * quantisation,
                        reset_end: values[3].reset_end as f32 * quantisation,
                        conv_st: values[3].conv_st as f32 * quantisation,
                        conv_end: values[3].conv_end as f32 * quantisation,
                    },
                }
            } else {
                ActiveTimingConfiguration::ThreeLeds {
                    led2: LedTiming {
                        led_st: values[0].led_st as f32 * quantisation,
                        led_end: values[0].led_end as f32 * quantisation,
                        sample_st: values[0].sample_st as f32 * quantisation,
                        sample_end: values[0].sample_end as f32 * quantisation,
                        reset_st: values[0].reset_st as f32 * quantisation,
                        reset_end: values[0].reset_end as f32 * quantisation,
                        conv_st: values[0].conv_st as f32 * quantisation,
                        conv_end: values[0].conv_end as f32 * quantisation,
                    },
                    led3: LedTiming {
                        led_st: values[1].led_st as f32 * quantisation,
                        led_end: values[1].led_end as f32 * quantisation,
                        sample_st: values[1].sample_st as f32 * quantisation,
                        sample_end: values[1].sample_end as f32 * quantisation,
                        reset_st: values[1].reset_st as f32 * quantisation,
                        reset_end: values[1].reset_end as f32 * quantisation,
                        conv_st: values[1].conv_st as f32 * quantisation,
                        conv_end: values[1].conv_end as f32 * quantisation,
                    },
                    led1: LedTiming {
                        led_st: values[2].led_st as f32 * quantisation,
                        led_end: values[2].led_end as f32 * quantisation,
                        sample_st: values[2].sample_st as f32 * quantisation,
                        sample_end: values[2].sample_end as f32 * quantisation,
                        reset_st: values[2].reset_st as f32 * quantisation,
                        reset_end: values[2].reset_end as f32 * quantisation,
                        conv_st: values[2].conv_st as f32 * quantisation,
                        conv_end: values[2].conv_end as f32 * quantisation,
                    },
                    ambient: AmbientTiming {
                        sample_st: values[3].sample_st as f32 * quantisation,
                        sample_end: values[3].sample_end as f32 * quantisation,
                        reset_st: values[3].reset_st as f32 * quantisation,
                        reset_end: values[3].reset_end as f32 * quantisation,
                        conv_st: values[3].conv_st as f32 * quantisation,
                        conv_end: values[3].conv_end as f32 * quantisation,
                    },
                }
            },
            inactive_timing: PowerDownTiming {
                power_down_st: power_down_values[0] as f32 * quantisation,
                power_down_end: power_down_values[1] as f32 * quantisation,
            },
        })
    }

    pub fn get_timing_window(
        &mut self,
    ) -> Result<MeasurementWindowConfiguration, AfeError<I2C::Error>> {
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
            _ => Default::default(),
        };
        let period_clk_div = clk_div / self.clock;
        let period = (r1dh_prev.prpct() + 1) as f32 * period_clk_div;
        let quantisation = period_clk_div;

        Ok(MeasurementWindowConfiguration {
            period,
            active_timing_configuration: if r36h_prev.led3ledstc() == 0
                && r37h_prev.led3ledendc() == 0
            {
                ActiveTimingConfiguration::TwoLeds {
                    led2: LedTiming {
                        led_st: r09h_prev.led2ledstc() as f32 * quantisation,
                        led_end: r0ah_prev.led2ledendc() as f32 * quantisation,
                        sample_st: r01h_prev.led2stc() as f32 * quantisation,
                        sample_end: r02h_prev.led2endc() as f32 * quantisation,
                        reset_st: r15h_prev.adcrststct0() as f32 * quantisation,
                        reset_end: r16h_prev.adcrstendct0() as f32 * quantisation,
                        conv_st: r0dh_prev.led2convst() as f32 * quantisation,
                        conv_end: r0eh_prev.led2convend() as f32 * quantisation,
                    },
                    ambient2: AmbientTiming {
                        sample_st: r05h_prev.aled2stc_or_led3stc() as f32 * quantisation,
                        sample_end: r06h_prev.aled2endc_or_led3endc() as f32 * quantisation,
                        reset_st: r17h_prev.adcrststct1() as f32 * quantisation,
                        reset_end: r18h_prev.adcrstendct1() as f32 * quantisation,
                        conv_st: r0fh_prev.aled2convst_or_led3convst() as f32 * quantisation,
                        conv_end: r10h_prev.aled2convend_or_led3convend() as f32 * quantisation,
                    },
                    led1: LedTiming {
                        led_st: r03h_prev.led1ledstc() as f32 * quantisation,
                        led_end: r04h_prev.led1ledendc() as f32 * quantisation,
                        sample_st: r07h_prev.led1stc() as f32 * quantisation,
                        sample_end: r08h_prev.led1endc() as f32 * quantisation,
                        reset_st: r19h_prev.adcrststct2() as f32 * quantisation,
                        reset_end: r1ah_prev.adcrstendct2() as f32 * quantisation,
                        conv_st: r11h_prev.led1convst() as f32 * quantisation,
                        conv_end: r12h_prev.led1convend() as f32 * quantisation,
                    },
                    ambient1: AmbientTiming {
                        sample_st: r0bh_prev.aled1stc() as f32 * quantisation,
                        sample_end: r0ch_prev.aled1endc() as f32 * quantisation,
                        reset_st: r1bh_prev.adcrststct3() as f32 * quantisation,
                        reset_end: r1ch_prev.adcrstendct3() as f32 * quantisation,
                        conv_st: r13h_prev.aled1convst() as f32 * quantisation,
                        conv_end: r14h_prev.aled1convend() as f32 * quantisation,
                    },
                }
            } else {
                ActiveTimingConfiguration::ThreeLeds {
                    led2: LedTiming {
                        led_st: r09h_prev.led2ledstc() as f32 * quantisation,
                        led_end: r0ah_prev.led2ledendc() as f32 * quantisation,
                        sample_st: r01h_prev.led2stc() as f32 * quantisation,
                        sample_end: r02h_prev.led2endc() as f32 * quantisation,
                        reset_st: r15h_prev.adcrststct0() as f32 * quantisation,
                        reset_end: r16h_prev.adcrstendct0() as f32 * quantisation,
                        conv_st: r0dh_prev.led2convst() as f32 * quantisation,
                        conv_end: r0eh_prev.led2convend() as f32 * quantisation,
                    },
                    led3: LedTiming {
                        led_st: r36h_prev.led3ledstc() as f32 * quantisation,
                        led_end: r37h_prev.led3ledendc() as f32 * quantisation,
                        sample_st: r05h_prev.aled2stc_or_led3stc() as f32 * quantisation,
                        sample_end: r06h_prev.aled2endc_or_led3endc() as f32 * quantisation,
                        reset_st: r17h_prev.adcrststct1() as f32 * quantisation,
                        reset_end: r18h_prev.adcrstendct1() as f32 * quantisation,
                        conv_st: r0fh_prev.aled2convst_or_led3convst() as f32 * quantisation,
                        conv_end: r10h_prev.aled2convend_or_led3convend() as f32 * quantisation,
                    },
                    led1: LedTiming {
                        led_st: r03h_prev.led1ledstc() as f32 * quantisation,
                        led_end: r04h_prev.led1ledendc() as f32 * quantisation,
                        sample_st: r07h_prev.led1stc() as f32 * quantisation,
                        sample_end: r08h_prev.led1endc() as f32 * quantisation,
                        reset_st: r19h_prev.adcrststct2() as f32 * quantisation,
                        reset_end: r1ah_prev.adcrstendct2() as f32 * quantisation,
                        conv_st: r11h_prev.led1convst() as f32 * quantisation,
                        conv_end: r12h_prev.led1convend() as f32 * quantisation,
                    },
                    ambient: AmbientTiming {
                        sample_st: r0bh_prev.aled1stc() as f32 * quantisation,
                        sample_end: r0ch_prev.aled1endc() as f32 * quantisation,
                        reset_st: r1bh_prev.adcrststct3() as f32 * quantisation,
                        reset_end: r1ch_prev.adcrstendct3() as f32 * quantisation,
                        conv_st: r13h_prev.aled1convst() as f32 * quantisation,
                        conv_end: r14h_prev.aled1convend() as f32 * quantisation,
                    },
                }
            },
            inactive_timing: PowerDownTiming {
                power_down_st: r32h_prev.pdncyclestc() as f32 * quantisation,
                power_down_end: r33h_prev.pdncycleendc() as f32 * quantisation,
            },
        })
    }
}
