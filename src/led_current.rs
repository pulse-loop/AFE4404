use embedded_hal::i2c::blocking::I2c;
use embedded_hal::i2c::SevenBitAddress;
use crate::{AFE4404, R22h, R23h};

impl<I2C> AFE4404<I2C>
    where I2C: I2c<SevenBitAddress> {

    pub fn set_leds_current(&mut self, led1: f32, led2: f32, led3: f32) -> Result<[f32; 3], ()> {
        let r23h_prev = self.registers.r23h.read().expect("Failed to read register 23h.");

        let high_current: bool = led1 > 50.0 || led2 > 50.0 || led3 > 50.0;
        let range = if high_current { 100.0 } else { 50.0 };
        let quantisation = range / 64.0;

        let values = [
            (led1 / quantisation).round() as u8,
            (led2 / quantisation).round() as u8,
            (led3 / quantisation).round() as u8,
        ];

        self.registers.r22h.write(R22h::init(values[0], values[1], values[2]));
        self.registers.r23h.write(R23h::o)

        return Ok(values.map(|v| (v as f32) * quantisation));
    }
}