use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AnalyticsError {
    #[error("At least 2 steps of time but there is {0} steps")]
    InvalidStepNumber(usize),
    #[error("At least 1 asset but there are {0} assets")]
    InvalidAssetNumber(usize),
    #[error("No NaN values is allowed")]
    NanValue,
    #[error("No negative values is allowed")]
    NegativeValue,
    #[error("The confidence value is between O and 1 but you put {0}")]
    InvalidRangeValue(f64),
    #[error("No infinite value")]
    NoInfiniteValue,
    #[error("Invalid Price")]
    InvalidPrice,
    #[error("dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error(
        "insufficient observations ({available}) for unbiased estimation with {required} degrees of freedom"
    )]
    InsufficientObservations { available: usize, required: usize },
}
