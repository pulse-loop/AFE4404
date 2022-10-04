use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;

use crate::{AFE4404, R00h};

impl<I2C> AFE4404<I2C>
    where
        I2C: I2c<SevenBitAddress>, {
    pub fn read(&mut self) -> [u32; 4] {
        let r2Ah_prev = self
            .registers
            .r2Ah
            .read()
            .expect("Failed to read register 2Ah.");
        let r2Bh_prev = self
            .registers
            .r2Bh
            .read()
            .expect("Failed to read register 2Bh.");
        let r2Ch_prev = self
            .registers
            .r2Ch
            .read()
            .expect("Failed to read register 2Ch.");
        let r2Dh_prev = self
            .registers
            .r2Dh
            .read()
            .expect("Failed to read register 2Dh.");
        // TODO: Conversion.
    }

    pub fn reset(&mut self) {
        self.registers
            .r00h
            .write(
                R00h::new()
                    .with_sw_reset(true)
            )
            .expect("Failed to write register 00h.");
    }

    pub fn enable_register_reading(&mut self) {
        self.registers
            .r00h
            .write(
                R00h::new()
                    .with_reg_read(true)
            )
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
            .write(
                r00h_prev.with_reg_read(false)
            )
            .expect("Failed to write register 00h.");
    }

    pub fn start_sampling(&mut self) {}

    pub fn stop_sampling(&mut self) {}
}