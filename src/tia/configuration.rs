use uom::si::f32::{Capacitance, ElectricalResistance};

use crate::modes::{LedMode, ThreeLedsMode, TwoLedsMode};

/// Represents the feedback resistors of the TIA inside the [`AFE4404`].
#[derive(Copy, Clone, Debug)]
pub struct ResistorConfiguration<MODE: LedMode> {
    resistor1: ElectricalResistance,
    resistor2: ElectricalResistance,
    mode: core::marker::PhantomData<MODE>,
}

impl ResistorConfiguration<ThreeLedsMode> {
    /// Creates a new `ResistorConfiguration`.
    ///
    /// # Notes
    ///
    /// `resistor1` is used during sample LED1 and sample Ambient phases.
    /// `resistor2` is used during sample LED2 and sample LED3 phases.
    pub fn new(resistor1: ElectricalResistance, resistor2: ElectricalResistance) -> Self {
        Self {
            resistor1,
            resistor2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the resistor used during sample LED1 and sample Ambient phases.
    pub fn resistor1(&self) -> &ElectricalResistance {
        &self.resistor1
    }

    /// Gets an immutable reference of the resistor used during sample LED2 and sample LED3 phases.
    pub fn resistor2(&self) -> &ElectricalResistance {
        &self.resistor2
    }

    /// Gets a mutable reference of the resistor used during sample LED1 and sample Ambient phases.
    pub fn resistor1_mut(&mut self) -> &mut ElectricalResistance {
        &mut self.resistor1
    }

    /// Gets a mutable reference of the resistor used during sample LED2 and sample LED3 phases.
    pub fn resistor2_mut(&mut self) -> &mut ElectricalResistance {
        &mut self.resistor2
    }
}

impl ResistorConfiguration<TwoLedsMode> {
    /// Creates a new `ResistorConfiguration`.
    ///
    /// # Notes
    ///
    /// `resistor1` is used during sample LED1 and sample Ambient1 phases.
    /// `resistor2` is used during sample LED2 and sample Ambient2 phases.
    pub fn new(resistor1: ElectricalResistance, resistor2: ElectricalResistance) -> Self {
        Self {
            resistor1,
            resistor2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the resistor used during sample LED1 and sample Ambient1 phases.
    pub fn resistor1(&self) -> &ElectricalResistance {
        &self.resistor1
    }

    /// Gets an immutable reference of the resistor used during sample LED2 and sample Ambient2 phases.
    pub fn resistor2(&self) -> &ElectricalResistance {
        &self.resistor2
    }

    /// Gets a mutable reference of the resistor used during sample LED1 and sample Ambient1 phases.
    pub fn resistor1_mut(&mut self) -> &mut ElectricalResistance {
        &mut self.resistor1
    }

    /// Gets a mutable reference of the resistor used during sample LED2 and sample Ambient2 phases.
    pub fn resistor2_mut(&mut self) -> &mut ElectricalResistance {
        &mut self.resistor2
    }
}

/// Represents the feedback capacitors of the TIA inside the [`AFE4404`].
#[derive(Copy, Clone, Debug)]
pub struct CapacitorConfiguration<MODE: LedMode> {
    capacitor1: Capacitance,
    capacitor2: Capacitance,
    mode: core::marker::PhantomData<MODE>,
}

impl CapacitorConfiguration<ThreeLedsMode> {
    /// Creates a new `CapacitorConfiguration`.
    ///
    /// # Notes
    ///
    /// `capacitor1` is used during sample LED1 and sample Ambient phases.
    /// `capacitor2` is used during sample LED2 and sample LED3 phases.
    pub fn new(capacitor1: Capacitance, capacitor2: Capacitance) -> Self {
        Self {
            capacitor1,
            capacitor2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the capacitor used during sample LED1 and sample Ambient phases.
    pub fn capacitor1(&self) -> &Capacitance {
        &self.capacitor1
    }

    /// Gets an immutable reference of the capacitor used during sample LED2 and sample LED3 phases.
    pub fn capacitor2(&self) -> &Capacitance {
        &self.capacitor2
    }

    /// Gets a mutable reference of the capacitor used during sample LED1 and sample Ambient phases.
    pub fn capacitor1_mut(&mut self) -> &mut Capacitance {
        &mut self.capacitor1
    }

    /// Gets a mutable reference of the capacitor used during sample LED2 and sample LED3 phases.
    pub fn capacitor2_mut(&mut self) -> &mut Capacitance {
        &mut self.capacitor2
    }
}

impl CapacitorConfiguration<TwoLedsMode> {
    /// Creates a new `CapacitorConfiguration`.
    ///
    /// # Notes
    ///
    /// `capacitor1` is used during sample LED1 and sample Ambient1 phases.
    /// `capacitor2` is used during sample LED2 and sample Ambient2 phases.
    pub fn new(capacitor1: Capacitance, capacitor2: Capacitance) -> Self {
        Self {
            capacitor1,
            capacitor2,
            mode: core::marker::PhantomData,
        }
    }

    /// Gets an immutable reference of the capacitor used during sample LED1 and sample Ambient1 phases.
    pub fn capacitor1(&self) -> &Capacitance {
        &self.capacitor1
    }

    /// Gets an immutable reference of the capacitor used during sample LED2 and sample Ambient2 phases.
    pub fn capacitor2(&self) -> &Capacitance {
        &self.capacitor2
    }

    /// Gets a mutable reference of the capacitor used during sample LED1 and sample Ambient1 phases.
    pub fn capacitor1_mut(&mut self) -> &mut Capacitance {
        &mut self.capacitor1
    }

    /// Gets a mutable reference of the capacitor used during sample LED2 and sample Ambient2 phases.
    pub fn capacitor2_mut(&mut self) -> &mut Capacitance {
        &mut self.capacitor2
    }
}
