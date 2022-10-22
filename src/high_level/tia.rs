use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::f32::{Capacitance, ElectricalResistance};
use uom::si::{
    capacitance::picofarad, electrical_resistance::kiloohm, electrical_resistance::megaohm,
};

use crate::errors::AfeError;
use crate::AFE4404;

#[derive(Debug)]
pub struct ResistorConfiguration {
    pub resistor1: ElectricalResistance,
    pub resistor2: ElectricalResistance,
}

#[derive(Debug)]
pub struct CapacitorConfiguration {
    pub capacitor1: Capacitance,
    pub capacitor2: Capacitance,
}

impl<I2C> AFE4404<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Set the tia resistors value.
    ///
    /// `resistor1` is used during sample LED1 and sample Ambient1 phases,
    /// `resistor2` is used during sample LED2 and sample Ambient2 or LED3 phases.
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
        configuration: &ResistorConfiguration,
    ) -> Result<ResistorConfiguration, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let mut values: [(ElectricalResistance, u8); 2] = Default::default();
        for (i, &resistor) in [configuration.resistor1, configuration.resistor2]
            .iter()
            .enumerate()
        {
            values[i] = match resistor.get::<kiloohm>() {
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
            };
        }

        let separate_resistor: bool = (values[0] != values[1]) || r20h_prev.ensepgain();

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_resistor)
                .with_tia_gain_sep(values[1].1),
        )?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_gain(values[0].1))?;

        Ok(ResistorConfiguration {
            resistor1: values[0].0,
            resistor2: values[1].0,
        })
    }

    /// Get the tia resistors value.
    ///
    /// `resistor1` is used during sample LED1 and sample Ambient1 phases,
    /// `resistor2` is used during sample LED2 and sample Ambient2 or LED3 phases.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_tia_resistors(&mut self) -> Result<ResistorConfiguration, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let mut values: [ElectricalResistance; 2] = Default::default();
        for (i, &resistor_reg) in [r21h_prev.tia_gain(), r20h_prev.tia_gain_sep()]
            .iter()
            .enumerate()
        {
            values[i] = match resistor_reg {
                5 => ElectricalResistance::new::<kiloohm>(10.0),
                4 => ElectricalResistance::new::<kiloohm>(25.0),
                3 => ElectricalResistance::new::<kiloohm>(50.0),
                2 => ElectricalResistance::new::<kiloohm>(100.0),
                1 => ElectricalResistance::new::<kiloohm>(250.0),
                0 => ElectricalResistance::new::<kiloohm>(500.0),
                6 => ElectricalResistance::new::<megaohm>(1.0),
                7 => ElectricalResistance::new::<megaohm>(2.0),
                _ => Default::default(),
            }
        }

        Ok(ResistorConfiguration {
            resistor1: values[0],
            resistor2: values[1],
        })
    }

    /// Set the tia capacitors value.
    ///
    /// `capacitor1` is used during sample LED1 and sample Ambient1 phases,
    /// `capacitor2` is used during sample LED2 and sample Ambient2 or LED3 phases.
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
        configuration: &CapacitorConfiguration,
    ) -> Result<CapacitorConfiguration, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let mut value: [(Capacitance, u8); 2] = Default::default();
        for (i, &capacitor) in [configuration.capacitor1, configuration.capacitor2]
            .iter()
            .enumerate()
        {
            value[i] = match capacitor.get::<picofarad>() {
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
            }
        }

        let separate_capacitor: bool = (value[0] != value[1]) || r20h_prev.ensepgain();

        self.registers.r20h.write(
            r20h_prev
                .with_ensepgain(separate_capacitor)
                .with_tia_cf_sep(value[1].1),
        )?;
        self.registers
            .r21h
            .write(r21h_prev.with_tia_cf(value[0].1))?;

        Ok(CapacitorConfiguration {
            capacitor1: value[0].0,
            capacitor2: value[1].0,
        })
    }

    /// Set the tia capacitors value.
    ///
    /// `capacitor1` is used during sample LED1 and sample Ambient1 phases,
    /// `capacitor2` is used during sample LED2 and sample Ambient2 or LED3 phases.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    pub fn get_tia_capacitors(&mut self) -> Result<CapacitorConfiguration, AfeError<I2C::Error>> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let mut values: [Capacitance; 2] = Default::default();
        for (i, &capacitor_reg) in [r21h_prev.tia_cf(), r20h_prev.tia_cf_sep()]
            .iter()
            .enumerate()
        {
            values[i] = match capacitor_reg {
                1 => Capacitance::new::<picofarad>(2.5),
                0 => Capacitance::new::<picofarad>(5.0),
                3 => Capacitance::new::<picofarad>(7.5),
                2 => Capacitance::new::<picofarad>(10.0),
                5 => Capacitance::new::<picofarad>(17.5),
                4 => Capacitance::new::<picofarad>(20.0),
                7 => Capacitance::new::<picofarad>(22.5),
                6 => Capacitance::new::<picofarad>(25.0),
                _ => Default::default(),
            }
        }

        Ok(CapacitorConfiguration {
            capacitor1: values[0],
            capacitor2: values[1],
        })
    }
}
