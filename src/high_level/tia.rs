use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use uom::si::f32::{Capacitance, ElectricalResistance};
use uom::si::{capacitance::picofarad, electrical_resistance::kiloohm};

use crate::AFE4404;

impl<I2C> AFE4404<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// Set the tia gain with resistors.
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
        resistor1: ElectricalResistance,
        resistor2: ElectricalResistance,
    ) -> Result<[u16; 2], ()> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let mut values: [(u16, u8); 2] = [(0, 0); 2];
        for (i, &resistor) in [resistor1, resistor2].iter().enumerate() {
            values[i] = match resistor.get::<kiloohm>() {
                r if r < 10.0 => return Err(()),
                r if r < 18.0 => (10, 5), // (resistor value, register value).
                r if r < 38.0 => (25, 4),
                r if r < 75.0 => (50, 3),
                r if r < 175.0 => (100, 2),
                r if r < 375.0 => (250, 1),
                r if r < 750.0 => (500, 0),
                r if r < 1500.0 => (1000, 6),
                r if r <= 2000.0 => (2000, 7),
                _ => return Err(()),
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

        Ok([values[0].0, values[1].0])
    }

    /// Sets the tia capacitors of this [`AFE4404<I2C>`].
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
    /// Setting a capacitor value outside the range 2500-25000 fF will result in an error.
    pub fn set_tia_capacitors(
        &mut self,
        capacitor1: Capacitance,
        capacitor2: Capacitance,
    ) -> Result<[u16; 2], ()> {
        let r20h_prev = self.registers.r20h.read()?;
        let r21h_prev = self.registers.r21h.read()?;

        let mut value: [(u16, u8); 2] = [(0, 0); 2];
        for (i, &capacitor) in [capacitor1, capacitor2].iter().enumerate() {
            value[i] = match capacitor.get::<picofarad>() {
                c if c < 2.5 => return Err(()),
                c if c < 3.75 => (2500, 1), // (capacitor value, register value).
                c if c < 6.25 => (5000, 0),
                c if c < 8.75 => (7500, 3),
                c if c < 13.75 => (10000, 2),
                c if c < 18.75 => (17500, 5),
                c if c < 21.25 => (20000, 4),
                c if c < 23.75 => (22500, 7),
                c if c <= 25.0 => (25000, 6),
                _ => return Err(()),
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

        Ok([value[0].0, value[1].0])
    }
}
