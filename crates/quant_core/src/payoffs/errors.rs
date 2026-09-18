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
    #[error("The first strike K1 ({0}) price must be less than the second strike price K2 ({1})")]
    StrikeConfigurationError(f64, f64),
}
