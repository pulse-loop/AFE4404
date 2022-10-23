use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum AfeError<I2CError: embedded_hal::i2c::Error> {
    #[error("I2C error")]
    I2CError(#[from] I2CError),
    #[error("incorrect I2C answer length (expected: {}, received: {})", .expected, .received)]
    IncorrectAnswerLength { expected: usize, received: usize },
    #[error("the requested LED current falls outside the allowed range")]
    LedCurrentOutsideAllowedRange,
    #[error("the requested resistor value falls outside the allowed range")]
    ResistorValueOutsideAllowedRange,
    #[error("the requested capacitor value falls outside the allowed range")]
    CapacitorValueOutsideAllowedRange,
    #[error("the ADC reading falls outside the allowed range")]
    AdcReadingOutsideAllowedRange,
    #[error("the requested window period is too long for the current clock frequency")]
    WindowPeriodTooLong,
    #[error("the requested internal clock is not 4MHz")]
    IncorrectInternalClock,
}
