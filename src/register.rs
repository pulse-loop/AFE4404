//! This module contains the register communication via I2C functions.

use alloc::sync::Arc;
use core::cell::RefCell;

use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::{errors::AfeError, RegisterWritable};

/// Represents a register inside the AFE4404.
pub(crate) struct Register<I2C, BF> {
    _p: core::marker::PhantomData<BF>,
    reg_addr: u8,
    phy_addr: SevenBitAddress,
    i2c: Arc<RefCell<I2C>>,
}

impl<I2C, BF> Register<I2C, BF>
where
    I2C: I2c,
    BF: RegisterWritable,
{
    /// Creates a new [`Register<I2C, BF>`] given a physical and memory address, associated to the specified I2C interface.
    pub(crate) fn new(reg_addr: u8, phy_addr: SevenBitAddress, i2c: Arc<RefCell<I2C>>) -> Self {
        Self {
            _p: core::marker::PhantomData::default(),
            reg_addr,
            phy_addr,
            i2c,
        }
    }

    /// Reads the contents of this [`Register<I2C, BF>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if an I2C transaction fails.
    pub(crate) fn read(&mut self) -> Result<BF, AfeError<I2C::Error>> {
        // Enable register reading flag for configuration registers.
        if self.reg_addr < 0x2a || (self.reg_addr > 0x2f && self.reg_addr < 0x3f) {
            self.i2c
                .borrow_mut()
                .write(self.phy_addr, [0, 0, 0, 1].as_slice())?;
        }

        let output_buffer = [self.reg_addr];
        let mut receive_buffer: [u8; 3] = [0, 0, 0];

        self.i2c.borrow_mut().write(self.phy_addr, &output_buffer)?;

        self.i2c
            .borrow_mut()
            .read(self.phy_addr, &mut receive_buffer)?;

        // Disable register reading flag for configuration registers.
        if self.reg_addr < 0x2a || (self.reg_addr > 0x2f && self.reg_addr < 0x3f) {
            self.i2c
                .borrow_mut()
                .write(self.phy_addr, [0, 0, 0, 0].as_slice())?;
        }

        Ok(BF::from_reg_bytes(receive_buffer))
    }

    /// Writes a new value to the specified register.
    ///
    /// # Errors
    ///
    /// This function will return an error if if an I2C transaction fails.
    pub(crate) fn write(&mut self, value: BF) -> Result<(), AfeError<I2C::Error>> {
        let mut buffer: [u8; 4] = [self.reg_addr, 0, 0, 0];
        buffer[1..=3].copy_from_slice(&value.into_reg_bytes());

        self.i2c
            .borrow_mut()
            .write(self.phy_addr, buffer.as_slice())?;

        Ok(())
    }
}
