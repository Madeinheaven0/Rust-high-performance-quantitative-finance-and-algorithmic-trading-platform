use rayon::prelude::*;

use quant_core::errors;
use quant_core::payoffs::{domain_types, leg, spread, volatility_strategies, range_bound};
use quant_core::payoffs::leg::OptionLeg;

pub struct OptionEngine {
    pub legs: Vec<leg::OptionLeg>,
    pub spots: Vec<domain_types::Spot>,
}

impl OptionEngine {
    pub fn build(legs: &[OptionLeg], spots: &[domain_types::Spot]) -> Self {
        OptionEngine {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        }
    }

    pub fn compute_payoff(&self) -> Result<Vec<f64>, errors::StrategyError> {
        Ok(
            self.legs
                .par_iter()
                .zip(self.spots.par_iter())
                .map(|(leg, spot)|
                    {leg.payoff(spot.to_owned()).unwrap()}).collect()
        )
    }

    pub fn compute_pnl(&self) -> Result<Vec<f64>, errors::StrategyError> {
        Ok(
            self.legs
                .par_iter()
                .zip(self.spots.par_iter())
                .map(|(leg, spot)|
                    {leg.payoff(spot.to_owned()).unwrap()}).collect()
        )
    }
}

pub struct SpreadEngine<T: spread::SpreadStrategy> {
    spreads: Vec<T>,
    spots: Vec<domain_types::Spot>,
}

impl<T: spread::SpreadStrategy + Sync + Send> SpreadEngine<T> {
    pub fn new(spreads: Vec<T>, spots: Vec<domain_types::Spot>) -> Self {
        Self {
            spreads,
            spots
        }
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.spreads.par_iter().zip(self.spots.par_iter()).map(|(spread, spot)|  spread.payoff(spot.to_owned())).collect()
    }
}
