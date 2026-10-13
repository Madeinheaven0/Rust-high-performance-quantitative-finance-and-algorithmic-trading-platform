//! # The engine of the platform
use rayon::prelude::*;
use tracing::instrument;
use tracing::{info, error, debug};

use crate::errors::EngineError;
use quant_core::errors;
use quant_core::payoffs::{domain_types, leg, spread, volatility_strategies, range_bound};
use quant_core::payoffs::leg::OptionLeg;

pub struct OptionEngine {
    pub legs: Vec<leg::OptionLeg>,
    pub spots: Vec<domain_types::Spot>,
}

impl OptionEngine {
    #[instrument(skip(legs, spots), fields(legs_len = legs.len(), spots_len = spots.len()))]
    pub fn build(legs: &[OptionLeg], spots: &[domain_types::Spot]) -> Result<Self, EngineError> {
        if legs.is_empty() {
            error!("The list of legs is empty");
            return Err(EngineError::EmptyElements);
        }
        if spots.is_empty() {
            error!("The list of spots is empty");
            return Err(EngineError::EmptySpots);
        }

        if legs.len() != spots.len() {
            error!("The list of legs and spots are not equal");
            debug!("Legs len: {}, spots len: {}", legs.len(), spots.len());
            return Err(EngineError::MismatchedLengths {
                strategies_len: legs.len(),
                spots_len: spots.len(),
            })
        }

        info!("The OptionEngine is created");
        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        Ok(
            self.legs
                .par_iter()
                .zip(self.spots.par_iter())
                .map(|(leg, spot)|
                    {leg.payoff(spot.to_owned()).unwrap()}).collect()
        )
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        Ok(
            self.legs
                .par_iter()
                .zip(self.spots.par_iter())
                .map(|(leg, spot)|
                    {leg.pnl(spot.to_owned()).unwrap()}).collect()
        )
    }
}

pub struct SpreadEngine<T: spread::SpreadStrategy> {
    spreads: Vec<T>,
    spots: Vec<domain_types::Spot>,
}

impl<T: spread::SpreadStrategy + Sync + Send + Clone> SpreadEngine<T> {
    pub fn build(spreads: &[T], spots: &[domain_types::Spot]) -> Result<Self, EngineError> {
        if spreads.is_empty() {
            error!("The list of legs is empty");
            return Err(EngineError::EmptyElements);
        }
        if spots.is_empty() {
            error!("The list of spots is empty");
            return Err(EngineError::EmptySpots);
        }

        if spreads.len() != spots.len() {
            error!("The list of legs and spots are not equal");
            debug!("Spread's len: {}, spots len: {}", spreads.len(), spots.len());
            return Err(EngineError::MismatchedLengths {
                strategies_len: spreads.len(),
                spots_len: spots.len(),
            })
        }

        Ok(Self {
            spreads: spreads.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self
            .spreads
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(spread, spot)|  spread.payoff(spot.to_owned()))
            .collect()
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self
            .spreads
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(spread, spot)| spread.pnl(spot.to_owned()))
            .collect()
    }
}

pub struct VolatilityEngine<T: volatility_strategies::VolatilityStrategy> {
    legs: Vec<T>,
    spots: Vec<domain_types::Spot>,
}

impl<T: volatility_strategies::VolatilityStrategy + Sync + Send + Clone> VolatilityEngine<T> {
    pub fn build(legs: &[T], spots: &[domain_types::Spot]) -> Result<Self, EngineError> {
        if legs.is_empty() {
            error!("The list of legs is empty");
            return Err(EngineError::EmptyElements);
        }
        if spots.is_empty() {
            error!("The list of spots is empty");
            return Err(EngineError::EmptySpots);
        }

        if legs.len() != spots.len() {
            error!("The list of legs and spots are not equal");
            debug!("Legs len: {}, spots len: {}", legs.len(), spots.len());
            return Err(EngineError::MismatchedLengths {
                strategies_len: legs.len(),
                spots_len: spots.len(),
            })
        }

        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self
            .legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.payoff(spot.to_owned()))
            .collect()
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self
            .legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.pnl(spot.to_owned()))
            .collect()
    }
}

pub struct RangeEngine<T: range_bound::RangeStrategy> {
    legs: Vec<T>,
    spots: Vec<domain_types::Spot>,
}

impl<T: range_bound::RangeStrategy + Sync + Send + Clone> RangeEngine<T> {
    pub fn build(legs: &[T], spots: &[domain_types::Spot]) -> Result<Self, EngineError> {
        if legs.is_empty() {
            error!("The list of legs is empty");
            return Err(EngineError::EmptyElements);
        }
        if spots.is_empty() {
            error!("The list of spots is empty");
            return Err(EngineError::EmptySpots);
        }

        if legs.len() != spots.len() {
            error!("The list of legs and spots are not equal");
            debug!("Legs len: {}, spots len: {}", legs.len(), spots.len());
            return Err(EngineError::MismatchedLengths {
                strategies_len: legs.len(),
                spots_len: spots.len(),
            })
        }

        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self
            .legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)|  leg.payoff(spot.to_owned()))
            .collect()
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self
            .legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)|  leg.payoff(spot.to_owned()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use quant_core::payoffs::domain_types::Strike;

    #[test]
    fn test_build_option_engine() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots);

        assert!(engine.is_ok());

        let engine = engine.unwrap();

        assert_ne!(engine.legs.is_empty(), true);
        assert_ne!(engine.spots.is_empty(), true);
        assert_eq!(engine.legs.len(), 2);
        assert_eq!(engine.spots.len(), engine.legs.len());
    }

    #[test]
    fn test_build_option_engine_invalid_lengths() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg3 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(107.),
            domain_types::Premium(3.)
        ).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2, leg3];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots);

        assert!(engine.is_err());
    }

    #[test]
    fn test_compute_payoffs_is_ok() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots).unwrap();

        let payoffs = engine.payoffs();

        assert!(payoffs.is_ok());
    }

    #[test]
    fn test_compute_payoffs() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots).unwrap();
        let payoffs = engine.payoffs().unwrap();

        assert_eq!(payoffs[0], 8.);
        assert_eq!(payoffs[1], 0.);
    }

    #[test]
    fn test_compute_pnls_is_ok() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots).unwrap();

        let pnls = engine.pnls();

        assert!(pnls.is_ok());
    }

    #[test]
    fn test_compute_pnls() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Position::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.)
        ).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots).unwrap();

        let pnls = engine.pnls().unwrap();

        assert_eq!(pnls.len(), 2);
        assert_eq!(pnls[0], 5.);
        assert_eq!(pnls[1], -3.);
    }
}