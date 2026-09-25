//! # The type of the option's strategy ( Call or Put)
use clap::ValueEnum;
#[derive(Clone, PartialEq, Debug, ValueEnum)]
pub enum CallPutCategory {
    Call,
    Put,
}

#[derive(Clone, PartialEq, Debug, ValueEnum)]
pub enum LongShortCategory {
    Long,
    Short,
}
