use thiserror::Error;

#[derive(Error, Debug)]
pub enum PriceError {
    #[error("The spot price must be greater than zero. The spot price is {0}")]
    SpotPriceNegative(f64),
    #[error("The strike prices must be greater than zero")]
    StrikePriceNegative,
    #[error("Enter a valid number for the spot price")]
    SpotPriceInvalid,
    #[error("Enter a valid number for the strike price")]
    StrikePriceInvalid,
    #[error("The strike ({0}) price must be less than the strike up ({1})")]
    StrikeConfigurationError(f64, f64),
    #[error("At least one prime price is less or equal than 0")]
    PrimePriceError,
}
