//! # The engine of the platform
use rayon::prelude::*;
use tracing::instrument;
use tracing::{error, info};

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

        info!("The OptionEngine is created");
        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    /// Computes the payoffs for all strategies across all spot prices in parallel (Cartesian product).
    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.payoff(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }

    /// Computes the PnLs for all strategies across all spot prices in parallel (Cartesian product).
    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.pnl(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }
}

pub struct SpreadEngine<T: spread::SpreadStrategy> {
    pub spreads: Vec<T>,
    pub spots: Vec<domain_types::Spot>,
}

impl<T: spread::SpreadStrategy + Sync + Send + Clone> SpreadEngine<T> {
    pub fn build(spreads: &[T], spots: &[domain_types::Spot]) -> Result<Self, EngineError> {
        if spreads.is_empty() {
            error!("The list of spreads is empty");
            return Err(EngineError::EmptyElements);
        }
        if spots.is_empty() {
            error!("The list of spots is empty");
            return Err(EngineError::EmptySpots);
        }

        Ok(Self {
            spreads: spreads.to_vec(),
            spots: spots.to_vec(),
        })
    }

    /// Computes the payoffs for all strategies across all spot prices in parallel (Cartesian product).
    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.spreads
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.payoff(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }


    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.spreads
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.pnl(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }
}

pub struct VolatilityEngine<T: volatility_strategies::VolatilityStrategy> {
    pub legs: Vec<T>,
    pub spots: Vec<domain_types::Spot>,
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

        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.payoff(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }

    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.pnl(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }
}

pub struct RangeEngine<T: range_bound::RangeStrategy> {
    pub legs: Vec<T>,
    pub spots: Vec<domain_types::Spot>,
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

        Ok(Self {
            legs: legs.to_vec(),
            spots: spots.to_vec(),
        })
    }

    pub fn payoffs(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.payoff(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
    }


    pub fn pnls(&self) -> Result<Vec<f64>, errors::StrategyError> {
        self.legs
            .par_iter()
            .map(|leg| {
                self.spots
                    .iter()
                    .map(|spot| leg.pnl(spot.to_owned()))
                    .collect::<Result<Vec<f64>, errors::StrategyError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, errors::StrategyError>>()
            .map(|matrix| matrix.into_iter().flatten().collect())
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
            OptionKind::Call,
            Side::Long,
            Strike(100.),
            Premium(3.),
        )
            .unwrap();
        let leg2 = leg::OptionLeg::build(
            OptionKind::Call,
            Side::Long,
            Strike(100.),
            Premium(3.),
        )
            .unwrap();
        let leg3 = leg::OptionLeg::build(
            OptionKind::Call,
            Side::Long,
            Strike(107.),
            Premium(3.),
        )
            .unwrap();

        let spot1 = Spot(108.);
        let spot2 = Spot(90.);

        let spreads = vec![leg1, leg2, leg3];
        let spots = vec![spot1, spot2];

        let engine = OptionEngine::build(&spreads, &spots);

        assert!(engine.is_err());
    }

    #[test]
    fn test_compute_payoffs() {
        let leg1 = leg::OptionLeg::build(
            OptionKind::Call,
            Side::Long,
            Strike(100.),
            Premium(3.),
        )
            .unwrap();
        let leg2 = leg::OptionLeg::build(
            OptionKind::Call,
            Side::Long,
            Strike(100.),
            Premium(3.),
        )
            .unwrap();

        let spot1 = Spot(108.);
        let spot2 = Spot(90.);

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
            OptionKind::Call,
            Side::Long,
            Strike(100.),
            Premium(3.),
        )
            .unwrap();
        let leg2 = leg::OptionLeg::build(
            OptionKind::Call,
            Side::Long,
            Strike(100.),
            Premium(3.),
        )
            .unwrap();

        let spot1 = Spot(108.);
        let spot2 = Spot(90.);

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

        assert!(SpreadEngine::build(&empty_spreads, &spots).is_err());
    }

    #[test]
    fn test_volatility_engine_empty_spots() {
        let empty_spots: Vec<Spot> = vec![];
        let empty_legs: Vec<volatility_strategies::Straddle> = vec![];

        assert!(VolatilityEngine::build(&empty_legs, &empty_spots).is_err());
    }

    #[test]
    fn test_range_engine_mismatched_lengths() {
        let spot1 = Spot(100.);
        let spot2 = Spot(110.);
        let spots = vec![spot1, spot2];

        let legs: Vec<range_bound::IronCondor> = vec![];
        assert!(RangeEngine::build(&legs, &spots).is_err());
    }
}