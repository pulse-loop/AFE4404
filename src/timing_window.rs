use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::{f32::{Frequency, Time}, frequency::megahertz, time::{microsecond, second}};

use crate::{
    AFE4404, R01h, R02h, R03h, R04h, R05h, R06h, R07h, R08h, R09h, R0Ah, R0Bh, R0Ch, R0Dh, R0Eh, R0Fh,
    R10h, R11h, R12h, R13h, R14h, R15h, R16h, R17h, R18h, R19h, R1Ah, R1Bh, R1Ch, R1Dh, R1Eh, R36h,
    R37h, R39h,
};

pub struct MeasurementWindowConfiguration {
    period: Time,
    active_timing_configuration: ActiveTimingConfiguration,
    inactive_timing: PowerDownTiming,
}

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

struct LedTiming {
    led_st: Time,
    led_end: Time,
    sample_st: Time,
    sample_end: Time,
    reset_st: Time,
    reset_end: Time,
    conv_st: Time,
    conv_end: Time,
}

struct AmbientTiming {
    sample_st: Time,
    sample_end: Time,
    reset_st: Time,
    reset_end: Time,
    conv_st: Time,
    conv_end: Time,
}

struct PowerDownTiming {
    powerdown_st: Time,
    powerdown_end: Time,
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
    pub fn set_timing_window(&mut self, configuration: MeasurementWindowConfiguration) -> Result<(), ()> {
        let r1Eh_prev = self
            .registers
            .r1Eh
            .read()?;

        // TODO: manage external clock.
        let clk = Frequency::new::<megahertz>(4.0);

        let clk_div = ((configuration.period * clk).value / 65536.0).ceil() as u8;
        let clk_div: (f32, u8) = match clk_div {
            1 => (1.0, 0), // (division ratio, register value).
            2 => (2.0, 4),
            d if d <= 4 => (4.0, 5),
            d if d <= 8 => (8.0, 6),
            d if d <= 16 => (16.0, 7),
            _ => return Err(()),
        };
        let period_clk: Time = 1.0 / clk;
        let period_clk_div: Time = period_clk * clk_div.0;
        let counter = configuration.period / period_clk_div;
        let counter_max_value = counter.round() as u16 - 1;
        let quantisation = period / counter;

        /*values[i] = TimingRegisters {
            led_st: (curr_phase_st / quantisation).round() as u16,
            led_end: (time_led_end / quantisation).round() as u16,
            sample_st: ((curr_phase_st + time_led_st_to_sample_st) / quantisation).round() as u16,
            sample_end: (time_led_end / quantisation).round() as u16,
            reset_st: (time_reset_st / quantisation).round() as u16,
            reset_end: (time_reset_end / quantisation).round() as u16,
            conv_st: (time_conv_st / quantisation).round() as u16,
            conv_end: ((time_conv_st + time_conv) / quantisation).round() as u16,
        };*/

        // Enable timer engine.
        self.registers.r1Eh.write(R1Eh::new().with_timeren(true))?;

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
            .r0Dh
            .write(R0Dh::new().with_led2convst(values[0].conv_st))?;
        self.registers
            .r0Eh
            .write(R0Eh::new().with_led2convend(values[0].conv_end))?;
        self.registers
            .r15h
            .write(R15h::new().with_adcrststct0(values[0].reset_st))?;
        self.registers
            .r16h
            .write(R16h::new().with_adcrstendct0(values[0].reset_end))?;

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
            .r0Fh
            .write(R0Fh::new().with_aled2convst_or_led3convst(values[1].conv_st))?;
        self.registers
            .r10h
            .write(R10h::new().with_aled2convend_or_led3convend(values[1].conv_end))?;
        self.registers
            .r17h
            .write(R17h::new().with_adcrststct1(values[1].reset_st))?;
        self.registers
            .r18h
            .write(R18h::new().with_adcrstendct1(values[1].reset_end))?;

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
            .r11h
            .write(R11h::new().with_led1convst(values[2].conv_st))?;
        self.registers
            .r12h
            .write(R12h::new().with_led1convend(values[2].conv_end))?;
        self.registers
            .r19h
            .write(R19h::new().with_adcrststct2(values[2].reset_st))?;
        self.registers
            .r1Ah
            .write(R1Ah::new().with_adcrstendct2(values[2].reset_end))?;

        // Write ambient1 registers.
        self.registers
            .r0Bh
            .write(R0Bh::new().with_aled1stc(values[1].sample_st))?;
        self.registers
            .r0Ch
            .write(R0Ch::new().with_aled1endc(values[1].sample_end))?;
        self.registers
            .r13h
            .write(R13h::new().with_aled1convst(values[1].conv_st))?;
        self.registers
            .r14h
            .write(R14h::new().with_aled1convend(values[1].conv_end))?;
        self.registers
            .r1Bh
            .write(R1Bh::new().with_adcrststct3(values[1].reset_st))?;
        self.registers
            .r1Ch
            .write(R1Ch::new().with_adcrstendct3(values[1].reset_end))?;

        self.registers
            .r1Dh
            .write(R1Dh::new().with_prpct(counter_max_value))?;
        self.registers
            .r39h
            .write(R39h::new().with_clkdiv_prf(clk_div.1))?;

        Ok(())
    }
}
