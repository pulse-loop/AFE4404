use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::electric_potential::volt;
use uom::si::f32::ElectricPotential;

use crate::{
    afe4404::{LedMode, ThreeLedsMode, TwoLedsMode},
    errors::AfeError,
    AFE4404,
};

#[derive(Debug)]
pub struct Readings<MODE: LedMode> {
    led1: ElectricPotential,
    led2: ElectricPotential,
    ambient1: ElectricPotential,
    ambient2_or_led3: ElectricPotential,
    led1_minus_ambient1: ElectricPotential,
    led2_minus_ambient2: ElectricPotential,
    mode: core::marker::PhantomData<MODE>,
}

impl Readings<ThreeLedsMode> {
    pub(crate) fn new(
        led1: ElectricPotential,
        led2: ElectricPotential,
        led3: ElectricPotential,
        ambient: ElectricPotential,
        led1_minus_ambient: ElectricPotential,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1: ambient,
            ambient2_or_led3: led3,
            led1_minus_ambient1: led1_minus_ambient,
            led2_minus_ambient2: ElectricPotential::new::<volt>(0.0),
            mode: core::marker::PhantomData,
        }
    }
    pub fn led1(&self) -> &ElectricPotential {
        &self.led1
    }
    pub fn led2(&self) -> &ElectricPotential {
        &self.led2
    }
    pub fn led3(&self) -> &ElectricPotential {
        &self.ambient2_or_led3
    }
    pub fn ambient(&self) -> &ElectricPotential {
        &self.ambient1
    }
    pub fn led1_minus_ambient(&self) -> &ElectricPotential {
        &self.led1_minus_ambient1
    }
}

impl Readings<TwoLedsMode> {
    pub(crate) fn new(
        led1: ElectricPotential,
        led2: ElectricPotential,
        ambient1: ElectricPotential,
        ambient2: ElectricPotential,
        led1_minus_ambient1: ElectricPotential,
        led2_minus_ambient2: ElectricPotential,
    ) -> Self {
        Self {
            led1,
            led2,
            ambient1,
            ambient2_or_led3: ambient2,
            led1_minus_ambient1,
            led2_minus_ambient2,
            mode: std::marker::PhantomData,
        }
    }
    pub fn led1(&self) -> &ElectricPotential {
        &self.led1
    }
    pub fn led2(&self) -> &ElectricPotential {
        &self.led2
    }
    pub fn ambient1(&self) -> &ElectricPotential {
        &self.ambient1
    }
    pub fn ambient2(&self) -> &ElectricPotential {
        &self.ambient2_or_led3
    }
    pub fn led1_minus_ambient1(&self) -> &ElectricPotential {
        &self.led1_minus_ambient1
    }
    pub fn led2_minus_ambient2(&self) -> &ElectricPotential {
        &self.led2_minus_ambient2
    }
}

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Returns an array of raw readings from the frontend.
    ///
    /// # Errors
    ///
    /// This function will return an error in case of failure of an I2C operation.
    #[allow(clippy::similar_names)]
    fn get_raw_readings(&mut self) -> Result<[ElectricPotential; 6], AfeError<I2C::Error>> {
        let r2ah_prev = self.registers.r2Ah.read()?;
        let r2bh_prev = self.registers.r2Bh.read()?;
        let r2ch_prev = self.registers.r2Ch.read()?;
        let r2dh_prev = self.registers.r2Dh.read()?;
        let r2fh_prev = self.registers.r2Fh.read()?;

        let quantisation: ElectricPotential = ElectricPotential::new::<volt>(1.2) / 2_097_151.0;

        let mut values: [ElectricPotential; 6] = Default::default();

        // We are converting a 22 bit reading (stored in a 32 bit register) to a 32 bit float.
        // Since the 32 bit float has a 23 bits, we allow a precision loss.
        // We also allow wraps since we take the sign into account.
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_wrap)]
        for (i, &register_value) in [
            r2ch_prev.led1val(),
            r2ah_prev.led2val(),
            r2bh_prev.aled2val_or_led3val(),
            r2dh_prev.aled1val(),
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

        Ok(values)
    }
}

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Read the sampled values.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid untill next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    pub fn read(&mut self) -> Result<Readings<ThreeLedsMode>, AfeError<I2C::Error>> {
        let values = self.get_raw_readings()?;

        Ok(Readings::<ThreeLedsMode>::new(
            values[0], values[1], values[2], values[3], values[4],
        ))
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Read the sampled values.
    ///
    /// # Notes
    ///
    /// Call this function after an `ADC_RDY` pulse, data will remain valid untill next `ADC_RDY` pulse.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// This function returns an error if the ADC reading falls outside the allowed range.
    #[allow(clippy::similar_names)]
    pub fn read(&mut self) -> Result<Readings<TwoLedsMode>, AfeError<I2C::Error>> {
        let values = self.get_raw_readings()?;

        Ok(Readings::<TwoLedsMode>::new(
            values[0], values[1], values[2], values[3], values[4], values[5],
        ))
    }
}
