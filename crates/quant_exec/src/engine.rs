//! # The engine of the platform
use rayon::prelude::*;
use tracing::instrument;
use tracing::{debug, error, info};

use crate::errors::EngineError;
use quant_core::errors;
use quant_core::payoffs::{domain_types, leg, range_bound, spread, volatility_strategies};

pub struct OptionEngine {
    pub legs: Vec<leg::OptionLeg>,
    pub spots: Vec<domain_types::Spot>,
}

impl OptionEngine {
    #[instrument(skip(legs, spots), fields(legs_len = legs.len(), spots_len = spots.len()))]
    pub fn build(
        legs: &[leg::OptionLeg],
        spots: &[domain_types::Spot],
    ) -> Result<Self, EngineError> {
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
            });
        }

        info!("The OptionEngine is created");
        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        Ok(self
            .legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.payoff(spot.to_owned()).unwrap())
            .collect())
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        Ok(self
            .legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.pnl(spot.to_owned()).unwrap())
            .collect())
    }
}

pub struct SpreadEngine<T: spread::SpreadStrategy> {
    pub spreads: Vec<T>,
    pub spots: Vec<domain_types::Spot>,
}

impl<T: spread::SpreadStrategy + Sync + Send> SpreadEngine<T> {
    pub fn build(spreads: Vec<T>, spots: Vec<domain_types::Spot>) -> Result<Self, EngineError> {
        if spreads.is_empty() {
            error!("The list of spreads is empty");
            return Err(EngineError::EmptyElements);
        }
        if spots.is_empty() {
            error!("The list of spots is empty");
            return Err(EngineError::EmptySpots);
        }

        if spreads.len() != spots.len() {
            error!("The list of spreads and spots are not equal");
            debug!(
                "Spread's len: {}, spots len: {}",
                spreads.len(),
                spots.len()
            );
            return Err(EngineError::MismatchedLengths {
                strategies_len: spreads.len(),
                spots_len: spots.len(),
            });
        }

        Ok(Self { spreads, spots })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.spreads
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(spread, spot)| spread.payoff(spot.to_owned()))
            .collect()
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.spreads
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(spread, spot)| spread.pnl(spot.to_owned()))
            .collect()
    }
}

pub struct VolatilityEngine<T: volatility_strategies::VolatilityStrategy> {
    pub legs: Vec<T>,
    pub spots: Vec<domain_types::Spot>,
}

impl<T: volatility_strategies::VolatilityStrategy + Sync + Send> VolatilityEngine<T> {
    pub fn build(legs: Vec<T>, spots: Vec<domain_types::Spot>) -> Result<Self, EngineError> {
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
            });
        }

        Ok(Self { legs, spots })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.payoff(spot.to_owned()))
            .collect()
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.pnl(spot.to_owned()))
            .collect()
    }
}

pub struct RangeEngine<T: range_bound::RangeStrategy> {
    pub legs: Vec<T>,
    pub spots: Vec<domain_types::Spot>,
}

impl<T: range_bound::RangeStrategy + Sync + Send> RangeEngine<T> {
    pub fn build(legs: Vec<T>, spots: Vec<domain_types::Spot>) -> Result<Self, EngineError> {
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
            });
        }

        Ok(Self { legs, spots })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.payoff(spot.to_owned()))
            .collect()
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .zip(self.spots.par_iter())
            .map(|(leg, spot)| leg.pnl(spot.to_owned()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain_types::{OptionKind, Premium, Side, Spot};
    use quant_core::payoffs::domain_types::Strike;

    #[test]
    fn test_build_option_engine() {
        let leg1 =
            leg::OptionLeg::build(OptionKind::Call, Side::Long, Strike(100.), Premium(3.)).unwrap();
        let leg2 =
            leg::OptionLeg::build(OptionKind::Call, Side::Long, Strike(100.), Premium(3.)).unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = Spot(90.);

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
            domain_types::Side::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.),
        )
            .unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Side::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.),
        )
            .unwrap();
        let leg3 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Side::Long,
            domain_types::Strike(107.),
            domain_types::Premium(3.),
        )
            .unwrap();

        let spot1 = domain_types::Spot(108.);
        let spot2 = domain_types::Spot(90.);

        let spreads = vec![leg1, leg2, leg3];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots);

        assert!(engine.is_err());
    }

    #[test]
    fn test_compute_payoffs() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Side::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.),
        )
            .unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Side::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.),
        )
            .unwrap();

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
    fn test_compute_pnls() {
        let leg1 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Side::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.),
        )
            .unwrap();
        let leg2 = leg::OptionLeg::build(
            domain_types::OptionKind::Call,
            domain_types::Side::Long,
            domain_types::Strike(100.),
            domain_types::Premium(3.),
        )
            .unwrap();

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

    #[test]
    fn test_spread_engine_empty_elements() {
        let spot1 = domain_types::Spot(100.);
        let spots = vec![spot1];
        let empty_spreads: Vec<spread::BullCallSpread> = vec![];

        // Utilisation de .is_err() pour éviter d'exiger Debug sur l'Engine
        assert!(SpreadEngine::build(empty_spreads, spots).is_err());
    }

    #[test]
    fn test_volatility_engine_empty_spots() {
        let empty_spots: Vec<Spot> = vec![];
        let empty_legs: Vec<volatility_strategies::Straddle> = vec![];

        assert!(VolatilityEngine::build(empty_legs, empty_spots).is_err());
    }

    #[test]
    fn test_range_engine_mismatched_lengths() {
        let spot1 = Spot(100.);
        let spot2 = Spot(110.);
        let spots = vec![spot1, spot2];

        let legs: Vec<range_bound::IronCondor> = vec![];
        assert!(RangeEngine::build(legs, spots).is_err());
    }
}