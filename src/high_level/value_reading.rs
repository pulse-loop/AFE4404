use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_potential::volt;
use uom::si::f32::ElectricPotential;

use crate::{R00h, AFE4404};

pub enum ReadingMode {
    ThreeLeds,
    TwoLeds,
}

pub enum Readings {
    ThreeLeds {
        led2: ElectricPotential,
        led3: ElectricPotential,
        led1: ElectricPotential,
        ambient: ElectricPotential,
        led1_minus_ambient: ElectricPotential,
    },
    TwoLeds {
        led2: ElectricPotential,
        ambient2: ElectricPotential,
        led1: ElectricPotential,
        ambient1: ElectricPotential,
        led2_minus_ambient2: ElectricPotential,
        led1_minus_ambient1: ElectricPotential,
    },
}

impl<I2C> AFE4404<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    ///
    ///
    /// # Notes
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn read(&mut self, mode: ReadingMode) -> Result<Readings, ()> {
        const fn extend_sign(n: u32) -> i32 {
            let mut result = n as i32;
            if (result & 0x0080_0000) != 0 {
                result = !((result ^ 0x00FF_FFFF) + 1) + 1;
            }
            result
        }

        let r2Ah_prev = self.registers.r2Ah.read()?;
        let r2Bh_prev = self.registers.r2Bh.read()?;
        let r2Ch_prev = self.registers.r2Ch.read()?;
        let r2Dh_prev = self.registers.r2Dh.read()?;
        let r2Eh_prev = self.registers.r2Eh.read()?;
        let r2Fh_prev = self.registers.r2Fh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        let mut values: [ElectricPotential; 6] = Default::default();
        for (i, &register_value) in [
            r2Ah_prev.led2val(),
            r2Bh_prev.aled2val_or_led3val(),
            r2Ch_prev.led1val(),
            r2Dh_prev.aled1val(),
            r2Eh_prev.led2_minus_aled2val(),
            r2Fh_prev.led1_minus_aled1val(),
        ]
        .iter()
        .enumerate()
        {
            let signed_value = extend_sign(register_value);
            if signed_value < -0x0020_0000 {
                return Err(()); // Lower than negative full-scale.
            } else if signed_value > 0x001F_FFFF {
                return Err(()); // Higher than positive full-scale.
            }
            values[i] = signed_value as f32 * quantisation;
        }

        Ok(match mode {
            ReadingMode::ThreeLeds => Readings::ThreeLeds {
                led2: values[0],
                led3: values[1],
                led1: values[2],
                ambient: values[3],
                led1_minus_ambient: values[5],
            },
            ReadingMode::TwoLeds => Readings::TwoLeds {
                led2: values[0],
                ambient2: values[1],
                led1: values[2],
                ambient1: values[3],
                led2_minus_ambient2: values[4],
                led1_minus_ambient1: values[5],
            },
        })
    }

    pub fn reset(&mut self) {
        self.registers
            .r00h
            .write(R00h::new().with_sw_reset(true))
            .expect("Failed to write register 00h.");
    }

    pub fn enable_register_reading(&mut self) {
        self.registers
            .r00h
            .write(R00h::new().with_reg_read(true))
            .expect("Failed to write register 00h.");
    }

    pub fn disable_register_reading(&mut self) {
        let r00h_prev = self
            .registers
            .r00h
            .read()
            .expect("Failed to read register 00h.");

        self.registers
            .r00h
            .write(r00h_prev.with_reg_read(false))
            .expect("Failed to write register 00h.");
    }

    pub fn start_sampling(&mut self) {}

    pub fn stop_sampling(&mut self) {}
}
