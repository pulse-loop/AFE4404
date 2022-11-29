//! This module contains the TIA related functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;

use super::ThreeLedsMode;
use super::TwoLedsMode;
use super::AFE4404;
use crate::errors::AfeError;

pub use configuration::{CapacitorConfiguration, ResistorConfiguration};

mod configuration;
mod low_level;

impl<I2C> AFE4404<I2C, ThreeLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the tia resistors value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the resistors value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a resistor value outside the range 10-2000 kOhm will result in an error.
    pub fn set_tia_resistors(
        &mut self,
        configuration: &ResistorConfiguration<ThreeLedsMode>,
    ) -> Result<ResistorConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let values = [
            Self::from_resistor(*configuration.resistor1())?,
            Self::from_resistor(*configuration.resistor2())?,
        ];

        let separate_resistor: bool =
            (values[0] != values[1]) || (r21h_prev.tia_cf() != r20h_prev.tia_cf_sep());

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_resistor)
                .with_tia_gain_sep(values[1].1),
        )?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_gain(values[0].1))?;

        Ok(ResistorConfiguration::<ThreeLedsMode>::new(
            values[0].0,
            values[1].0,
        ))
    }

    /// Gets the tia resistors value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_resistors(
        &mut self,
    ) -> Result<ResistorConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        Ok(ResistorConfiguration::<ThreeLedsMode>::new(
            self.get_tia_resistor1()?,
            self.get_tia_resistor2()?,
        ))
    }

    /// Sets the tia capacitors value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the capacitors value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a capacitor value outside the range 2.5-25 pF will result in an error.
    pub fn set_tia_capacitors(
        &mut self,
        configuration: &CapacitorConfiguration<ThreeLedsMode>,
    ) -> Result<CapacitorConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let values = [
            Self::from_capacitor(*configuration.capacitor1())?,
            Self::from_capacitor(*configuration.capacitor2())?,
        ];

        let separate_capacitor: bool =
            (values[0] != values[1]) || (r21h_prev.tia_gain() != r20h_prev.tia_gain_sep());

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_capacitor)
                .with_tia_cf_sep(values[1].1),
        )?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_cf(values[0].1))?;

        Ok(CapacitorConfiguration::<ThreeLedsMode>::new(
            values[0].0,
            values[1].0,
        ))
    }

    /// Gets the tia capacitors value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_capacitors(
        &mut self,
    ) -> Result<CapacitorConfiguration<ThreeLedsMode>, AfeError<I2C::Error>> {
        Ok(CapacitorConfiguration::<ThreeLedsMode>::new(
            self.get_tia_capacitor1()?,
            self.get_tia_capacitor2()?,
        ))
    }
}

impl<I2C> AFE4404<I2C, TwoLedsMode>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Sets the tia resistors value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the resistors value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a resistor value outside the range 10-2000 kOhm will result in an error.
    pub fn set_tia_resistors(
        &mut self,
        configuration: &ResistorConfiguration<TwoLedsMode>,
    ) -> Result<ResistorConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let values = [
            Self::from_resistor(*configuration.resistor1())?,
            Self::from_resistor(*configuration.resistor2())?,
        ];

        let separate_resistor: bool =
            (values[0] != values[1]) || (r21h_prev.tia_cf() != r20h_prev.tia_cf_sep());

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_resistor)
                .with_tia_gain_sep(values[1].1),
        )?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_gain(values[0].1))?;

        Ok(ResistorConfiguration::<TwoLedsMode>::new(
            values[0].0,
            values[1].0,
        ))
    }

    /// Gets the tia resistors value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_resistors(
        &mut self,
    ) -> Result<ResistorConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        Ok(ResistorConfiguration::<TwoLedsMode>::new(
            self.get_tia_resistor1()?,
            self.get_tia_resistor2()?,
        ))
    }

    /// Sets the tia capacitors value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the capacitors value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a capacitor value outside the range 2.5-25 pF will result in an error.
    pub fn set_tia_capacitors(
        &mut self,
        configuration: &CapacitorConfiguration<TwoLedsMode>,
    ) -> Result<CapacitorConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let values = [
            Self::from_capacitor(*configuration.capacitor1())?,
            Self::from_capacitor(*configuration.capacitor2())?,
        ];

        let separate_capacitor: bool =
            (values[0] != values[1]) || (r21h_prev.tia_gain() != r20h_prev.tia_gain_sep());

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_capacitor)
                .with_tia_cf_sep(values[1].1),
        )?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_cf(values[0].1))?;

        Ok(CapacitorConfiguration::<TwoLedsMode>::new(
            values[0].0,
            values[1].0,
        ))
    }

    /// Gets the tia capacitors value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_capacitors(
        &mut self,
    ) -> Result<CapacitorConfiguration<TwoLedsMode>, AfeError<I2C::Error>> {
        Ok(CapacitorConfiguration::<TwoLedsMode>::new(
            self.get_tia_capacitor1()?,
            self.get_tia_capacitor2()?,
        ))
    }
}
