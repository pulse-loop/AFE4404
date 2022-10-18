use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_potential::volt;
use uom::si::f32::ElectricPotential;

use crate::{errors::AfeError, R00h, AFE4404};

pub enum ReadingMode {
    ThreeLeds,
    TwoLeds,
}

#[derive(Debug)]
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
    pub fn read(&mut self, mode: ReadingMode) -> Result<Readings, AfeError<I2C::Error>> {
        let r2ah_prev = self.registers.r2Ah.read()?;
        let r2bh_prev = self.registers.r2Bh.read()?;
        let r2ch_prev = self.registers.r2Ch.read()?;
        let r2dh_prev = self.registers.r2Dh.read()?;
        let r2eh_prev = self.registers.r2Eh.read()?;
        let r2fh_prev = self.registers.r2Fh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        let mut values: [ElectricPotential; 6] = Default::default();
        for (i, &register_value) in [
            r2ah_prev.led2val(),
            r2bh_prev.aled2val_or_led3val(),
            r2ch_prev.led1val(),
            r2dh_prev.aled1val(),
            r2eh_prev.led2_minus_aled2val(),
            r2fh_prev.led1_minus_aled1val(),
        ]
            .iter()
            .enumerate()
        {
            let sign_extension_bits = ((register_value & 0x00FF_FFFF) >> 21) as u8;
            let signed_value = match sign_extension_bits {
                0b000 => register_value as i32, // The value is positive.
                0b111 => (register_value | 0xFF00_0000) as i32, // Extend the sign of the negative value.
                _ => return Err(AfeError::AdcReadingOutsideAllowedRange),
            };
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

    pub fn reset(&mut self) -> Result<(), AfeError<I2C::Error>> {
        self.registers.r00h.write(R00h::new().with_sw_reset(true))
    }

    pub fn set_clock_source(&mut self, internal: bool) -> Result<(), AfeError<I2C::Error>> {
        let r23h_prev = self.registers.r23h.read()?;

        self.registers
            .r23h
            .write(r23h_prev.with_osc_enable(internal))?;

        Ok(())
    }

    pub fn start_sampling(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r1eh_prev = self.registers.r1Eh.read()?;

        self.registers.r1Eh.write(r1eh_prev.with_timeren(true))?;

        Ok(())
    }

    pub fn stop_sampling(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r1eh_prev = self.registers.r1Eh.read()?;

        self.registers.r1Eh.write(r1eh_prev.with_timeren(false))?;

        Ok(())
    }

    pub fn enable_clock_out(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r29h_prev = self.registers.r29h.read()?;

        self.registers.r29h.write(r29h_prev.with_enable_clkout(true))?;

        Ok(())
    }
}