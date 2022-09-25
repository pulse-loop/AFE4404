use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::AFE4404;

impl<I2C> AFE4404<I2C>
    where
        I2C: I2c<SevenBitAddress>,
{
    /// Set the tia gain with resistors.
    ///
    /// The resistance is expressed in kiloohms.
    /// Resistor1 is used during sample LED1 and sample Ambient1 phases,
    /// resistor2 is used during sample LED2 and sample Ambient2 or LED3 phases.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the resistors value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a resistor value outside the range 10-2000 kOhm will result in an error.
    pub fn set_tia_resistors(&mut self, resistor1: u16, resistor2: u16) -> Result<[u16; 2], ()> {
        let r20h_prev = self
            .registers
            .r20h
            .read()
            .expect("Failed to read register 20h.");
        let r21h_prev = self
            .registers
            .r21h
            .read()
            .expect("Failed to read register 21h.");

        let mut value: [(u8, u16); 2] = [(0, 0), (0, 0)];
        for (i, &resistor) in [resistor1, resistor2].iter().enumerate() {
            value[i] = match resistor {
                r if r < 10 => return Err(()),
                r if r < 18 => (5, 10),
                r if r < 38 => (4, 25),
                r if r < 75 => (3, 50),
                r if r < 175 => (2, 100),
                r if r < 375 => (1, 250),
                r if r < 750 => (0, 500),
                r if r < 1500 => (6, 1000),
                r if r <= 2000 => (7, 2000),
                _ => return Err(()),
            }
        }

        let separate_resistor: bool = (value[0] != value[1]) || r20h_prev.ensepgain();

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_resistor).with_tia_gain_sep(value[1].0))
            .expect("Failed to write register 20h.");
        self.registers
            .r21h
            .write(r21h_prev.with_tia_gain(value[0].0))
            .expect("Failed to write register 21h.");

        Ok([value[0].1, value[1].1])
    }

    /// Set the tia bandwidth with capacitors.
    ///
    /// The capacitance is expressed in femtofarads.
    /// Capacitor1 is used during sample LED1 and sample Ambient1 phases,
    /// Capacitor2 is used during sample LED2 and sample Ambient2 or LED3 phases.
    ///
    /// # Notes
    ///
    /// This function automatically rounds the capacitors value to the closest actual value.
    ///
    /// # Errors
    ///
    /// This function returns an error if the I2C bus encounters an error.
    /// Setting a capacitor value outside the range 2500-25000 fF will result in an error.
    pub fn set_tia_capacitors(&mut self, capacitor1: u16, capacitor2: u16) -> Result<[u16; 2], ()> {
        let r20h_prev = self
            .registers
            .r20h
            .read()
            .expect("Failed to read register 20h.");
        let r21h_prev = self
            .registers
            .r21h
            .read()
            .expect("Failed to read register 21h.");

        let mut value: [(u8, u16); 2] = [(0, 0), (0, 0)];
        for (i, &capacitor) in [capacitor1, capacitor2].iter().enumerate() {
            value[i] = match capacitor {
                c if c < 2500 => return Err(()),
                c if c < 3750 => (1, 2500),
                c if c < 6250 => (0, 5000),
                c if c < 8750 => (3, 7500),
                c if c < 13750 => (2, 10000),
                c if c < 18750 => (5, 17500),
                c if c < 21250 => (4, 20000),
                c if c < 23750 => (7, 22500),
                c if c <= 25000 => (6, 25000),
                _ => return Err(()),
            }
        }

        let separate_capacitor: bool = (value[0] != value[1]) || r20h_prev.ensepgain();

        self.registers
            .r20h
            .write(r20h_prev.with_ensepgain(separate_capacitor).with_tia_cf_sep(value[1].0))
            .expect("Failed to write register 20h.");
        self.registers
            .r21h
            .write(r21h_prev.with_tia_cf(value[0].0))
            .expect("Failed to write register 21h.");

        Ok([value[0].1, value[1].1])
    }
}