//! This module contains the TIA low level functions.

use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::{
    capacitance::picofarad,
    electrical_resistance::kiloohm,
    electrical_resistance::megaohm,
    f32::{Capacitance, ElectricalResistance},
};

use crate::{device::AFE4404, errors::AfeError, modes::LedMode};

use super::values::CapacitorValue;
use super::values::ResistorValue;

impl<I2C, MODE> AFE4404<I2C, MODE>
where
    I2C: I2c<SevenBitAddress>,
    MODE: LedMode,
{
    /// Converts an `ElectricalResistance` into a tuple of `ElectricalResistance` rounded to the closest actual value and register value.
    pub(crate) fn from_resistor(
        resistor: ElectricalResistance,
    ) -> Result<(ElectricalResistance, u8), AfeError<I2C::Error>> {
        Ok(match resistor.get::<kiloohm>() {
            r if r < 10.0 => return Err(AfeError::ResistorValueOutsideAllowedRange),
            r if r < 18.0 => (ElectricalResistance::new::<kiloohm>(10.0), 5), // (resistor value, register value).
            r if r < 38.0 => (ElectricalResistance::new::<kiloohm>(25.0), 4),
            r if r < 75.0 => (ElectricalResistance::new::<kiloohm>(50.0), 3),
            r if r < 175.0 => (ElectricalResistance::new::<kiloohm>(100.0), 2),
            r if r < 375.0 => (ElectricalResistance::new::<kiloohm>(250.0), 1),
            r if r < 750.0 => (ElectricalResistance::new::<kiloohm>(500.0), 0),
            r if r < 1500.0 => (ElectricalResistance::new::<megaohm>(1.0), 6),
            r if r <= 2000.0 => (ElectricalResistance::new::<megaohm>(2.0), 7),
            _ => return Err(AfeError::ResistorValueOutsideAllowedRange),
        })
    }

    /// Converts a register value into an `ElectricalResistance`.
    pub(crate) fn into_resistor(
        reg_value: u8,
        reg_addr: u8,
    ) -> Result<ElectricalResistance, AfeError<I2C::Error>> {
        Ok(match reg_value {
            5 => ElectricalResistance::new::<kiloohm>(10.0),
            4 => ElectricalResistance::new::<kiloohm>(25.0),
            3 => ElectricalResistance::new::<kiloohm>(50.0),
            2 => ElectricalResistance::new::<kiloohm>(100.0),
            1 => ElectricalResistance::new::<kiloohm>(250.0),
            0 => ElectricalResistance::new::<kiloohm>(500.0),
            6 => ElectricalResistance::new::<megaohm>(1.0),
            7 => ElectricalResistance::new::<megaohm>(2.0),
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr }),
        })
    }

    /// Converts a `Capacitance` into a tuple of `Capacitance` rounded to the closest actual value and register value.
    pub(crate) fn from_capacitor(
        capacitor: Capacitance,
    ) -> Result<(Capacitance, u8), AfeError<I2C::Error>> {
        Ok(match capacitor.get::<picofarad>() {
            c if c < 2.5 => return Err(AfeError::CapacitorValueOutsideAllowedRange),
            c if c < 3.75 => (Capacitance::new::<picofarad>(2.5), 1), // (capacitor value, register value).
            c if c < 6.25 => (Capacitance::new::<picofarad>(5.0), 0),
            c if c < 8.75 => (Capacitance::new::<picofarad>(7.5), 3),
            c if c < 13.75 => (Capacitance::new::<picofarad>(10.0), 2),
            c if c < 18.75 => (Capacitance::new::<picofarad>(17.5), 5),
            c if c < 21.25 => (Capacitance::new::<picofarad>(20.0), 4),
            c if c < 23.75 => (Capacitance::new::<picofarad>(22.5), 7),
            c if c <= 25.0 => (Capacitance::new::<picofarad>(25.0), 6),
            _ => return Err(AfeError::CapacitorValueOutsideAllowedRange),
        })
    }

    /// Converts a register value into a `Capacitance`.
    pub(crate) fn into_capacitor(
        reg_value: u8,
        reg_addr: u8,
    ) -> Result<Capacitance, AfeError<I2C::Error>> {
        Ok(match reg_value {
            1 => Capacitance::new::<picofarad>(2.5),
            0 => Capacitance::new::<picofarad>(5.0),
            3 => Capacitance::new::<picofarad>(7.5),
            2 => Capacitance::new::<picofarad>(10.0),
            5 => Capacitance::new::<picofarad>(17.5),
            4 => Capacitance::new::<picofarad>(20.0),
            7 => Capacitance::new::<picofarad>(22.5),
            6 => Capacitance::new::<picofarad>(25.0),
            _ => return Err(AfeError::InvalidRegisterValue { reg_addr }),
        })
    }

    /// Sets the tia resistor1 value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the resistor value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a resistor value outside the range 10-2000 kOhm will result in an error.
    pub fn set_tia_resistor1(
        &mut self,
        resistor: ElectricalResistance,
    ) -> Result<ElectricalResistance, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = Self::from_resistor(resistor)?;

        let separate_resistor: bool =
            (value.1 != r20h_prev.tia_gain_sep()) || (r21h_prev.tia_cf() != r20h_prev.tia_cf_sep());

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_resistor))?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_gain(value.1))?;

        Ok(value.0)
    }

    /// Sets the tia resistor1 value given a `ResistorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_tia_resistor1_enum(
        &mut self,
        resistor: ResistorValue,
    ) -> Result<ResistorValue, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = resistor as u8;

        let separate_resistor: bool =
            (value != r20h_prev.tia_gain_sep()) || (r21h_prev.tia_cf() != r20h_prev.tia_cf_sep());

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_resistor))?;
        self.registers.r21h.write(r21h_prev.with_tia_gain(value))?;

        Ok(resistor)
    }

    /// Sets the tia resistor2 value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the resistor value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a resistor value outside the range 10-2000 kOhm will result in an error.
    pub fn set_tia_resistor2(
        &mut self,
        resistor: ElectricalResistance,
    ) -> Result<ElectricalResistance, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = Self::from_resistor(resistor)?;

        let separate_resistor: bool =
            (r21h_prev.tia_gain() != value.1) || (r21h_prev.tia_cf() != r20h_prev.tia_cf_sep());

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_resistor)
                .with_tia_gain_sep(value.1),
        )?;

        Ok(value.0)
    }

    /// Sets the tia resistor2 value given a `ResistorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_tia_resistor2_enum(
        &mut self,
        resistor: ResistorValue,
    ) -> Result<ResistorValue, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = resistor as u8;

        let separate_resistor: bool =
            (r21h_prev.tia_gain() != value) || (r21h_prev.tia_cf() != r20h_prev.tia_cf_sep());

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_resistor)
                .with_tia_gain_sep(value),
        )?;

        Ok(resistor)
    }

    /// Gets the tia resistor1 value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_resistor1(&mut self) -> Result<ElectricalResistance, AfeError<I2C::Error>> {
        let r21h_prev = self.registers.r21h.read()?;

        let value = Self::into_resistor(r21h_prev.tia_gain(), 0x21)?;

        Ok(value)
    }

    /// Gets the tia resistor1 value as a `ResistorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_tia_resistor1_enum(&mut self) -> Result<ResistorValue, AfeError<I2C::Error>> {
        let r21h_prev = self.registers.r21h.read()?;

        let value: ResistorValue = ResistorValue::from_u8(r21h_prev.tia_gain());

        Ok(value)
    }

    /// Gets the tia resistor2 value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_resistor2(&mut self) -> Result<ElectricalResistance, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;

        let value = Self::into_resistor(r20h_prev.tia_gain_sep(), 0x20)?;

        Ok(value)
    }

    /// Gets the tia resistor2 value as a `ResistorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_tia_resistor2_enum(&mut self) -> Result<ResistorValue, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;

        let value: ResistorValue = ResistorValue::from_u8(r20h_prev.tia_gain_sep());

        Ok(value)
    }

    /// Sets the tia capacitor1 value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the capacitor value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a capacitor value outside the range 2.5-25 pF will result in an error.
    pub fn set_tia_capacitor1(
        &mut self,
        capacitor: Capacitance,
    ) -> Result<Capacitance, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = Self::from_capacitor(capacitor)?;

        let separate_capacitor: bool = (r21h_prev.tia_gain() != r20h_prev.tia_gain_sep())
            || (value.1 != r20h_prev.tia_cf_sep());

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_capacitor))?;
        self.registers.r21h.write(r21h_prev.with_tia_cf(value.1))?;

        Ok(value.0)
    }

    /// Sets the tia capacitor1 value given a `CapacitorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_tia_capacitor1_enum(
        &mut self,
        capacitor: CapacitorValue,
    ) -> Result<CapacitorValue, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = capacitor as u8;

        let separate_capacitor: bool =
            (r21h_prev.tia_gain() != r20h_prev.tia_gain_sep()) || (value != r20h_prev.tia_cf_sep());

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_capacitor))?;
        self.registers.r21h.write(r21h_prev.with_tia_cf(value))?;

        Ok(capacitor)
    }

    /// Sets the tia capacitor2 value.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the capacitor value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a capacitor value outside the range 2.5-25 pF will result in an error.
    pub fn set_tia_capacitor2(
        &mut self,
        capacitor: Capacitance,
    ) -> Result<Capacitance, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = Self::from_capacitor(capacitor)?;

        let separate_capacitor: bool =
            (r21h_prev.tia_gain() != r20h_prev.tia_gain_sep()) || (r21h_prev.tia_cf() != value.1);

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_capacitor))?;
        self.registers.r21h.write(r21h_prev.with_tia_cf(value.1))?;

        Ok(value.0)
    }

    /// Sets the tia capacitor2 value given a `CapacitorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn set_tia_capacitor2_enum(
        &mut self,
        capacitor: CapacitorValue,
    ) -> Result<CapacitorValue, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let value = capacitor as u8;

        let separate_capacitor: bool =
            (r21h_prev.tia_gain() != r20h_prev.tia_gain_sep()) || (r21h_prev.tia_cf() != value);

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_capacitor))?;
        self.registers.r21h.write(r21h_prev.with_tia_cf(value))?;

        Ok(capacitor)
    }

    /// Gets the tia capacitor1 value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_capacitor1(&mut self) -> Result<Capacitance, AfeError<I2C::Error>> {
        let r21h_prev = self.registers.r21h.read()?;

        let value = Self::into_capacitor(r21h_prev.tia_cf(), 0x21)?;

        Ok(value)
    }

    /// Gets the tia capacitor1 value as a `CapacitorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_tia_capacitor1_enum(&mut self) -> Result<CapacitorValue, AfeError<I2C::Error>> {
        let r21h_prev = self.registers.r21h.read()?;

        let value = CapacitorValue::from_u8(r21h_prev.tia_cf());

        Ok(value)
    }

    /// Gets the tia capacitor2 value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error or if the [`AFE4404`] contains invalid data.
    pub fn get_tia_capacitor2(&mut self) -> Result<Capacitance, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;

        let value = Self::into_capacitor(r20h_prev.tia_cf_sep(), 0x20)?;

        Ok(value)
    }

    /// Gets the tia capacitor2 value as a `CapacitorValue`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_tia_capacitor2_enum(&mut self) -> Result<CapacitorValue, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;

        let value = CapacitorValue::from_u8(r20h_prev.tia_cf_sep());

        Ok(value)
    }
}
