use std::cmp::max;

use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::{AFE4404,
            R01h, R02h, R03h, R04h, R05h, R06h, R07h, R08h, R09h,
            R0Ah, R0Bh, R0Ch, R0Dh, R0Eh, R0Fh, R10h, R11h, R12h,
            R13h, R14h, R15h, R16h, R17h, R18h, R19h, R1Ah, R1Bh,
            R1Ch, R1Dh, R36h, R37h, R39h};

#[derive(Copy, Clone, Default)]
struct TimingRegisters {
    led_st: u16,
    led_end: u16,
    sample_st: u16,
    sample_end: u16,
    reset_st: u16,
    reset_end: u16,
    conv_st: u16,
    conv_end: u16,
}

impl<I2C> AFE4404<I2C>
    where
        I2C: I2c<SevenBitAddress>, {
    #![allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::too_many_lines)]
    /// Set the LEDs timings.
    ///
    /// All timings are expressed in microseconds.
    ///
    /// # Notes
    ///
    /// When working with two LEDs set led3 to (0, 0).
    /// This function automatically sets the adc sample and conversion phases.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting wrong timing values will result in an error.
    pub fn set_timing_window(&mut self, period: f32, led1: (f32, f32), led2: (f32, f32), led3: (f32, f32)) -> Result<[f32; 7], ()> {
        let r1eh_prev = self
            .registers
            .r1Eh
            .read()
            .expect("Failed to read register 1Eh.");

        let clk = 4_000_000;

        let clk_div = (period * (clk as f32) / 65536e6).ceil() as u8;
        let clk_div: (f32, u8) = match clk_div {
            1 => (1.0, 0),    // (division ratio, register value).
            2 => (2.0, 4),
            d if d <= 4 => (4.0, 5),
            d if d <= 8 => (8.0, 6),
            d if d <= 16 => (16.0, 7),
            _ => return Err(()),
        };
        let period_clk = 1e6 / (clk as f32);
        let period_clk_div = period_clk * clk_div.0;
        let counter = period / period_clk_div;
        let counter_max_value = counter.round() as u16;
        let quantisation = period / counter;

        let time_led_st_to_sample_st = f32::max(25.0, 0.2 * led2.1 - led2.0);
        let time_led_end_to_reset_st = 2.0 * period_clk_div;
        let time_reset = 6.0 * period_clk_div;
        let time_reset_end_to_conv_st = 2.0 * period_clk_div;
        let time_conv = ((r1eh_prev.numav() as f32 + 2.0) * 200.0).mul_add(period_clk, 15.0);

        let mut values: [TimingRegisters; 4] = [TimingRegisters::default(); 4];
        let amb = (0.0, 0.0);
        for (i, &phase) in [led2, led3, led1, amb].iter().enumerate() {
            // TODO: check phases order.
            let time_reset_st = phase.1 + time_led_end_to_reset_st;
            let time_reset_end = time_reset_st + time_reset;
            let time_conv_st = time_reset_end + time_reset_end_to_conv_st;

            values[i] = TimingRegisters {
                led_st: (phase.0 / quantisation).round() as u16,
                led_end: (phase.1 / quantisation).round() as u16,
                sample_st: ((phase.0 + time_led_st_to_sample_st) / quantisation).round() as u16,
                sample_end: (phase.1 / quantisation).round() as u16,
                reset_st: (time_reset_st / quantisation).round() as u16,
                reset_end: (time_reset_end / quantisation).round() as u16,
                conv_st: (time_conv_st / quantisation).round() as u16,
                conv_end: ((time_conv_st + time_conv) / quantisation).round() as u16,
            };
        }

        // Write led2 registers.
        self.registers
            .r09h
            .write(
                R09h::new()
                    .with_led2ledstc(values[0].led_st)
            )
            .expect("Failed to write register 09h");
        self.registers
            .r0Ah
            .write(
                R0Ah::new()
                    .with_led2ledendc(values[0].led_end)
            )
            .expect("Failed to write register 0Ah");
        self.registers
            .r01h
            .write(
                R01h::new()
                    .with_led2stc(values[0].sample_st)
            )
            .expect("Failed to write register 01h");
        self.registers
            .r02h
            .write(
                R02h::new()
                    .with_led2endc(values[0].sample_end)
            )
            .expect("Failed to write register 02h");
        self.registers
            .r0Dh
            .write(
                R0Dh::new()
                    .with_led2convst(values[0].conv_st)
            )
            .expect("Failed to write register 0Dh");
        self.registers
            .r0Eh
            .write(
                R0Eh::new()
                    .with_led2convend(values[0].conv_end)
            )
            .expect("Failed to write register 0Eh");
        self.registers
            .r15h
            .write(
                R15h::new()
                    .with_adcrststct0(values[0].reset_st)
            )
            .expect("Failed to write register 15h");
        self.registers
            .r16h
            .write(
                R16h::new()
                    .with_adcrstendct0(values[0].reset_end)
            )
            .expect("Failed to write register 16h");

        // Write ambient2 or led3 registers.
        self.registers
            .r36h
            .write(
                R36h::new()
                    .with_led3ledstc(values[1].led_st)
            )
            .expect("Failed to write register 36h");
        self.registers
            .r37h
            .write(
                R37h::new()
                    .with_led3ledendc(values[1].led_end)
            )
            .expect("Failed to write register 37h");
        self.registers
            .r05h
            .write(
                R05h::new()
                    .with_aled2stc_or_led3stc(values[1].sample_st)
            )
            .expect("Failed to write register 05h");
        self.registers
            .r06h
            .write(
                R06h::new()
                    .with_aled2endc_or_led3endc(values[1].sample_end)
            )
            .expect("Failed to write register 06h");
        self.registers
            .r0Fh
            .write(
                R0Fh::new()
                    .with_aled2convst_or_led3convst(values[1].conv_st)
            )
            .expect("Failed to write register 0Fh");
        self.registers
            .r10h
            .write(
                R10h::new()
                    .with_aled2convend_or_led3convend(values[1].conv_end)
            )
            .expect("Failed to write register 10h");
        self.registers
            .r17h
            .write(
                R17h::new()
                    .with_adcrststct1(values[1].reset_st)
            )
            .expect("Failed to write register 17h");
        self.registers
            .r18h
            .write(
                R18h::new()
                    .with_adcrstendct1(values[1].reset_end)
            )
            .expect("Failed to write register 18h");

        // Write led1 registers.
        self.registers
            .r03h
            .write(
                R03h::new()
                    .with_led1ledstc(values[2].led_st)
            )
            .expect("Failed to write register 03h");
        self.registers
            .r04h
            .write(
                R04h::new()
                    .with_led1ledendc(values[2].led_end)
            )
            .expect("Failed to write register 04h");
        self.registers
            .r07h
            .write(
                R07h::new()
                    .with_led1stc(values[2].sample_st)
            )
            .expect("Failed to write register 07h");
        self.registers
            .r08h
            .write(
                R08h::new()
                    .with_led1endc(values[2].sample_end)
            )
            .expect("Failed to write register 08h");
        self.registers
            .r11h
            .write(
                R11h::new()
                    .with_led1convst(values[2].conv_st)
            )
            .expect("Failed to write register 11h");
        self.registers
            .r12h
            .write(
                R12h::new()
                    .with_led1convend(values[2].conv_end)
            )
            .expect("Failed to write register 12h");
        self.registers
            .r19h
            .write(
                R19h::new()
                    .with_adcrststct2(values[2].reset_st)
            )
            .expect("Failed to write register 19h");
        self.registers
            .r1Ah
            .write(
                R1Ah::new()
                    .with_adcrstendct2(values[2].reset_end)
            )
            .expect("Failed to write register 1Ah");

        // Write ambient1 registers.
        self.registers
            .r0Bh
            .write(
                R0Bh::new()
                    .with_aled1stc(values[1].sample_st)
            )
            .expect("Failed to write register 0Bh");
        self.registers
            .r0Ch
            .write(
                R0Ch::new()
                    .with_aled1endc(values[1].sample_end)
            )
            .expect("Failed to write register 0Ch");
        self.registers
            .r13h
            .write(
                R13h::new()
                    .with_aled1convst(values[1].conv_st)
            )
            .expect("Failed to write register 13h");
        self.registers
            .r14h
            .write(
                R14h::new()
                    .with_aled1convend(values[1].conv_end)
            )
            .expect("Failed to write register 14h");
        self.registers
            .r1Bh
            .write(
                R1Bh::new()
                    .with_adcrststct3(values[1].reset_st)
            )
            .expect("Failed to write register 1Bh");
        self.registers
            .r1Ch
            .write(
                R1Ch::new()
                    .with_adcrstendct3(values[1].reset_end)
            )
            .expect("Failed to write register 1Ch");

        self.registers
            .r1Dh
            .write(
                R1Dh::new()
                    .with_prpct(counter_max_value)
            )
            .expect("Failed to write register 1Dh");
        self.registers
            .r39h
            .write(
                R39h::new()
                    .with_clkdiv_prf(clk_div.1)
            )
            .expect("Failed to write register 39h");

        Ok([counter_max_value, values[2].led_st, values[2].led_end, values[0].led_st, values[0].led_end, values[1].led_st, values[1].led_end].map(|v| f32::from(v) * quantisation))
    }
}