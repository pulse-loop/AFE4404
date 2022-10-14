use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_potential::volt;
use uom::si::f32::ElectricPotential;

use crate::{R00h, AFE4404, errors::AfeError};

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
    pub fn read(&mut self, mode: ReadingMode) -> Result<Readings, AfeError<I2C::Error>> {
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
        self.registers
            .r00h
            .write(R00h::new().with_sw_reset(true))
    }

    pub fn enable_register_reading(&mut self) -> Result<(), AfeError<I2C::Error>> {
        self.registers
            .r00h
            .write(R00h::new().with_reg_read(true))
    }

    pub fn disable_register_reading(&mut self) -> Result<(), AfeError<I2C::Error>> {
        let r00h_prev = self
            .registers
            .r00h
            .read()?;

        self.registers
            .r00h
            .write(r00h_prev.with_reg_read(false))
    }

    pub fn start_sampling(&mut self) {}

    pub fn stop_sampling(&mut self) {}
}
