//! This module contains all the valid values of the TIA resistors and capacitors.

use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::errors::AfeError;

/// Represents the possible values of the feedback resistors of the TIA inside the [`AFE4404`].
///
/// # Notes
///
/// The values are encoded as inside the [`AFE4404`] registers.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ResistorValue<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// A 10kΩ resistor.
    R10k = 5,
    /// A 25kΩ resistor.
    R25k = 4,
    /// A 50kΩ resistor.
    R50k = 3,
    /// A 100kΩ resistor.
    R100k = 2,
    /// A 250kΩ resistor.
    R250k = 1,
    /// A 500kΩ resistor.
    R500k = 0,
    /// A 1MΩ resistor.
    R1M = 6,
    /// A 2MΩ resistor.
    R2M = 7,
    /// Phantomdata.
    _Unreachable(core::marker::PhantomData<I2C>, core::convert::Infallible) = u8::MAX,
}

impl<I2C> TryFrom<u8> for ResistorValue<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    type Error = AfeError<I2C::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResistorValue::R500k),
            1 => Ok(ResistorValue::R250k),
            2 => Ok(ResistorValue::R100k),
            3 => Ok(ResistorValue::R50k),
            4 => Ok(ResistorValue::R25k),
            5 => Ok(ResistorValue::R10k),
            6 => Ok(ResistorValue::R1M),
            7 => Ok(ResistorValue::R2M),
            _ => Err(AfeError::ResistorValueOutsideAllowedRange),
        }
    }
}

impl<I2C> TryInto<u8> for ResistorValue<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    type Error = AfeError<I2C::Error>;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            ResistorValue::R500k => Ok(0),
            ResistorValue::R250k => Ok(1),
            ResistorValue::R100k => Ok(2),
            ResistorValue::R50k => Ok(3),
            ResistorValue::R25k => Ok(4),
            ResistorValue::R10k => Ok(5),
            ResistorValue::R1M => Ok(6),
            ResistorValue::R2M => Ok(7),
            ResistorValue::_Unreachable(_, _) => Err(AfeError::ResistorValueOutsideAllowedRange),
        }
    }
}

/// Represents the possible values of the feedback capacitors of the TIA inside the [`AFE4404`].
///
/// # Notes
///
/// The values are encoded as inside the [`AFE4404`] registers.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum CapacitorValue<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    /// A 2.5pF capacitor.
    C2p5 = 1,
    /// A 5pF capacitor.
    C5p0 = 0,
    /// A 7.5pF capacitor.
    C7p5 = 3,
    /// A 10pF capacitor.
    C10p0 = 2,
    /// A 17.5pF capacitor.
    C17p5 = 5,
    /// A 20pF capacitor.
    C20p0 = 4,
    /// A 22.5pF capacitor.
    C22p5 = 7,
    /// A 25pF capacitor.
    C25p0 = 6,
    /// Phantomdata.
    _Unreachable(core::marker::PhantomData<I2C>, core::convert::Infallible) = u8::MAX,
}

impl<I2C> TryFrom<u8> for CapacitorValue<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    type Error = AfeError<I2C::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CapacitorValue::C5p0),
            1 => Ok(CapacitorValue::C2p5),
            2 => Ok(CapacitorValue::C10p0),
            3 => Ok(CapacitorValue::C7p5),
            4 => Ok(CapacitorValue::C20p0),
            5 => Ok(CapacitorValue::C17p5),
            6 => Ok(CapacitorValue::C25p0),
            7 => Ok(CapacitorValue::C22p5),
            _ => Err(AfeError::CapacitorValueOutsideAllowedRange),
        }
    }
}

impl<I2C> TryInto<u8> for CapacitorValue<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    type Error = AfeError<I2C::Error>;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            CapacitorValue::C5p0 => Ok(0),
            CapacitorValue::C2p5 => Ok(1),
            CapacitorValue::C10p0 => Ok(2),
            CapacitorValue::C7p5 => Ok(3),
            CapacitorValue::C20p0 => Ok(4),
            CapacitorValue::C17p5 => Ok(5),
            CapacitorValue::C25p0 => Ok(6),
            CapacitorValue::C22p5 => Ok(7),
            CapacitorValue::_Unreachable(_, _) => Err(AfeError::CapacitorValueOutsideAllowedRange),
        }
    }
}
