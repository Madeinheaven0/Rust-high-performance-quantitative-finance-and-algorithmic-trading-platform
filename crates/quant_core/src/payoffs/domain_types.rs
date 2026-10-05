//! # The fundamentals type used in the quant_core crate

use crate::errors::StrategyError;
use clap::ValueEnum;

/// The option's type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum OptionKind {
    Call,
    Put,
}

/// The Position of the option
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum Position {
    Long,
    Short,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Spot(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Strike(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Premium(pub f64);
