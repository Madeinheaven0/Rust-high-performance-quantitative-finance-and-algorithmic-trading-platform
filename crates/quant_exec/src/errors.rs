use thiserror::Error;

use quant_core::errors::StrategyError;

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("Dimension error: The number of elements ({strategies_len}) doesn't match the number of spots ({spots_len})")]
    MismatchedLengths {
        strategies_len: usize,
        spots_len: usize,
    },
    
    #[error("The engine has no elements")]
    EmptyElements,
    
    #[error("The engine has no spots")]
    EmptySpots,
    
    #[error(transparent)]
    Strategy(#[from] StrategyError)
}