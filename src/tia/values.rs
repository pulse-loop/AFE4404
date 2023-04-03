//! This module contains all the valid values of the TIA resistors and capacitors.

/// Represents the possible values of the feedback resistors of the TIA inside the [`AFE4404`].
///
/// # Notes
///
/// The values are encoded as inside the [`AFE4404`] registers.
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ResistorValue {
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
}

impl ResistorValue {
    pub(crate) fn from_u8(value: u8) -> ResistorValue {
        match value {
            0 => ResistorValue::R500k,
            1 => ResistorValue::R250k,
            2 => ResistorValue::R100k,
            3 => ResistorValue::R50k,
            4 => ResistorValue::R25k,
            5 => ResistorValue::R10k,
            6 => ResistorValue::R1M,
            7 => ResistorValue::R2M,
            _ => unreachable!(),
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
pub enum CapacitorValue {
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
}

impl CapacitorValue {
    pub(crate) fn from_u8(value: u8) -> CapacitorValue {
        match value {
            0 => CapacitorValue::C5p0,
            1 => CapacitorValue::C2p5,
            2 => CapacitorValue::C10p0,
            3 => CapacitorValue::C7p5,
            4 => CapacitorValue::C20p0,
            5 => CapacitorValue::C17p5,
            6 => CapacitorValue::C25p0,
            7 => CapacitorValue::C22p5,
            _ => unreachable!(),
        }
    }
}
