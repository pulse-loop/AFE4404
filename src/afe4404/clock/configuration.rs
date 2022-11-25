/// Represents the clock mode of the [`AFE4404`].
#[derive(Debug, Clone, Copy)]
pub enum ClockConfiguration {
    /// The clock is driven by the internal oscillator at 4 MHz.
    Internal,
    /// The clock is driven by the internal oscillator at 4 MHz and propagated to the `CLK` pin.
    InternalToOutput {
        /// The division factor of the clock output.
        division_ratio: u8,
    },
    /// The clock is driven by an external oscillator.
    External,
}
