//! The main errors that can occur

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StrategyError {
    #[error("Invalid spot price: {0}. Must be strictly positive.")]
    InvalidSpot(f64),

    #[error("Invalid strike price: {0}. Must be strictly positive.")]
    InvalidStrike(f64),

    #[error("Invalid premium: {0}. Must be strictly positive.")]
    InvalidPremium(f64),

    #[error(
        "Spread configuration error: lower strike ({strike_down}) must be strictly less than upper strike ({strike_up})"
    )]
    InvalidSpreadStrikes { strike_down: f64, strike_up: f64 },

    #[error(
        "Strangle configuration error: put strike ({put_strike}) must be less than call strike ({call_strike})"
    )]
    InvalidStrangleStrikes { put_strike: f64, call_strike: f64 },

    #[error(
        "Iron Condor strike ordering violation: expected K1 < K2 < K3 < K4, got K1={k1}, K2={k2}, K3={k3}, K4={k4}"
    )]
    InvalidIronCondorStrikes { k1: f64, k2: f64, k3: f64, k4: f64 },

    #[error(
        "Iron Butterfly strike ordering violation: expected K1 < K_ATM < K3, got K1={k1}, ATM={k_atm}, K3={k3}"
    )]
    InvalidIronButterflyStrikes { k1: f64, k_atm: f64, k3: f64 },
}
