use embedded_hal::i2c;
use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum AfeError<I2C: embedded_hal::i2c::blocking::I2c> {
    #[error("I2C error")]
    I2CError(I2C::Error),
    #[error("incorrect I2C answer length (expected: {}, received: {})", .expected, .received)]
    IncorrectAnswerLength { expected: usize, received: usize },
}
