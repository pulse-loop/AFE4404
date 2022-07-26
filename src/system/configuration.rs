/// Represents the dynamic blocks inside the [`AFE4404`].
#[derive(Copy, Clone, Debug)]
pub struct DynamicConfiguration {
    /// Supply voltage for LEDs.
    pub transmitter: State,
    /// ADC.
    pub adc: State,
    /// TIA.
    pub tia: State,
    /// Rest of ADC.
    pub rest_of_adc: State,
}

/// Represents the state of a block.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// The block is enabled.
    Enabled,
    /// The block is disabled.
    Disabled,
}

impl From<bool> for State {
    fn from(val: bool) -> Self {
        // Attention: negative logic!
        if val {
            State::Disabled
        } else {
            State::Enabled
        }
    }
}

impl From<State> for bool {
    fn from(val: State) -> Self {
        val == State::Disabled
    }
}
