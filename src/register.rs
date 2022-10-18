use core::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};

use crate::{errors::AfeError, RegisterWritable};

/// Represents a register inside the AFE4404.
pub(crate) struct Register<I2C, BF> {
    _p: PhantomData<BF>,
    reg_addr: u8,
    phy_addr: SevenBitAddress,
    i2c: Rc<RefCell<I2C>>,
}

impl<I2C, BF> Register<I2C, BF>
    where
        I2C: I2c,
        BF: RegisterWritable,
{
    /// Creates a new register from a register address, a physical address and an I2C interface.
    ///
    /// # Arguments
    ///
    /// * `reg_addr`: The address of the register.
    /// * `phy_addr`: The physical I2C address of the AFE4404.
    /// * `i2c`: An I2C interface.
    ///
    /// returns: Register<I2C>
    pub(crate) fn new(reg_addr: u8, phy_addr: SevenBitAddress, i2c: Rc<RefCell<I2C>>) -> Self {
        Self {
            _p: std::marker::PhantomData::default(),
            reg_addr,
            phy_addr,
            i2c,
        }
    }

    /// Reads the content of the register.
    ///
    /// returns: Result<[u8; 3], ()>
    pub(crate) fn read(&mut self) -> Result<BF, AfeError<I2C::Error>> {
        // Enable register reading for configuration registers.
        if self.reg_addr < 0x2a || self.reg_addr > 0x2f {
            self.i2c.borrow_mut().write(self.phy_addr, [0, 0, 1].as_slice())?;
        }

        let output_buffer = [self.reg_addr];
        let mut receive_buffer: [u8; 3] = [0, 0, 0];

        self.i2c
            .borrow_mut()
            .write_read(self.phy_addr, &output_buffer, &mut receive_buffer)?;

        println!("Reading: {:?} from register {:#02x}", receive_buffer, self.reg_addr);

        // Disable register reading for configuration registers.
        if self.reg_addr < 0x2a || self.reg_addr > 0x2f {
            self.i2c.borrow_mut().write(self.phy_addr, [0, 0, 0].as_slice())?;
        }

        Ok(BF::from_reg_bytes(receive_buffer))
    }

    // TODO: Check all documentation for correct types.

    /// Writes a 24 bit value in the register.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to be written.
    ///
    /// returns: Result<(), ()>
    pub(crate) fn write(&mut self, value: BF) -> Result<(), AfeError<I2C::Error>> {
        let mut buffer: [u8; 4] = [self.reg_addr, 0, 0, 0];
        buffer[1..=3].copy_from_slice(
            &value
                .into_reg_bytes()
                .iter()
                .map(|byte| byte.reverse_bits())
                .collect::<Vec<u8>>()[..],
        );

        println!("Writing: {:?} to register {:#02x}", buffer, self.reg_addr);
        self.i2c
            .borrow_mut()
            .write(self.phy_addr, buffer.as_slice())?;

        Ok(())
    }
}
