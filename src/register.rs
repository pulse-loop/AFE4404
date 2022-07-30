use embedded_hal::{
    i2c::{
        blocking::I2c,
        SevenBitAddress,
    }
};

/// Represents a register inside the AFE4404.
struct Register<'a, I2C, BF> {
    reg_addr: u8,
    phy_addr: SevenBitAddress,
    i2c: &'a mut I2C,
}

impl<'a, I2C, BF> Register<'a, I2C, BF>
    where I2C: I2c {
    /// Creates a new register from a register address, a physical address and an I2C interface.
    ///
    /// # Arguments
    ///
    /// * `reg_addr`: The address of the register.
    /// * `phy_addr`: The physical I2C address of the AFE4404.
    /// * `i2c`: An I2C interface.
    ///
    /// returns: Register<I2C>
    fn new(reg_addr: u8, phy_addr: SevenBitAddress, i2c: &'a mut I2C) -> Self {
        Register {
            reg_addr,
            phy_addr,
            i2c,
        }
    }

    /// Reads the content of the register.
    ///
    /// returns: Result<[u8; 3], ()>
    fn read(&mut self) -> Result<[u8; 3], ()> {
        // TODO: Error types.
        let output_buffer = [self.reg_addr];
        let receive_buffer: &mut [u8] = &mut [];

        if self.i2c.write_read(self.phy_addr, &output_buffer, receive_buffer).is_err() {
            Err(())
        } else if receive_buffer.len() == 3 {
            let mut value: [u8; 3] = [0, 0, 0];
            value.copy_from_slice(&(receive_buffer[0..2]));
            Ok(value)
        } else {
            Err(())
        }
    }


    /// Writes a 24 bit value in the register.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to be written.
    ///
    /// returns: Result<(), ()>
    fn write(&mut self, value: [u8; 3]) -> Result<(), ()> {
        // TODO: Error and Ok types.
        let mut buffer: [u8; 4] = [self.reg_addr, 0, 0, 0];
        buffer[1..3].copy_from_slice(&value);
        if self.i2c.write(self.phy_addr, buffer.as_slice()).is_err() {
            Err(())
        } else {
            Ok(())
        }
    }
}