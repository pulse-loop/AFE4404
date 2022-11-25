/// Represents the dynamic blocks of the [`AFE4404<I2C>`].]
#[derive(Clone, Copy)]
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

/// Represents the power state of a dynamic block.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// The block is powered on.
    Enabled,
    /// The block is powered off.
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