use uom::si::f32::{ElectricalResistance, Capacitance};

/// Represents the feedback resistors of the TIA inside the [`AFE4404`].
#[derive(Debug)]
pub struct ResistorConfiguration {
    /// The resistor used during sample LED1 and sample Ambient1 phases.
    pub resistor1: ElectricalResistance,
    /// The resistor used during sample LED2 and sample Ambient2 or LED3 phases.
    pub resistor2: ElectricalResistance,
}

/// Represents the feedback capacitors of the TIA inside the [`AFE4404`].
#[derive(Debug)]
pub struct CapacitorConfiguration {
    /// The capacitor used during sample LED1 and sample Ambient1 phases.
    pub capacitor1: Capacitance,
    /// The capacitor used during sample LED2 and sample Ambient2 or LED3 phases.
    pub capacitor2: Capacitance,
}