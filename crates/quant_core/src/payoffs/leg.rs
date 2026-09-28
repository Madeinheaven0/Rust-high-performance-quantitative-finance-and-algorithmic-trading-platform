//! # The fundamental constituent cell.

use crate::payoffs::domain_types::{OptionKind, Position, Premium, Spot, Strike};
use crate::payoffs::errors::StrategyError;

/// Describe the structure of an option
#[derive(Debug, PartialEq, Clone)]
pub struct OptionLeg {
    pub kind: OptionKind,
    pub position: Position,
    pub strike: Strike,
    pub premium: Premium,
}

impl OptionLeg {
    /// Build and compute payoff and pnl
    ///
    /// ### Errors
    ///
    /// Return an error if `strike`, `premium` or `spot` are invalid
    pub fn build(
        kind: OptionKind,
        position: Position,
        strike: Strike,
        premium: Premium,
    ) -> Result<Self, StrategyError> {
        if strike.0 <= 0.0 {
            return Err(StrategyError::InvalidStrike(strike.0));
        }

        if premium.0 <= 0.0 {
            return Err(StrategyError::InvalidPremium(premium.0));
        }

        Ok(Self {
            kind,
            position,
            strike,
            premium,
        })
    }

    #[inline(always)]
    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        if spot.0 <= 0.0 {
            return Err(StrategyError::InvalidSpot(spot.0));
        }

        let intrinsic = match self.kind {
            OptionKind::Call => (spot.0 - self.strike.0).max(0.),
            OptionKind::Put => (self.strike.0 - spot.0).max(0.),
        };

        let coefficient = match self.position {
            Position::Long => 1.0,
            Position::Short => -1.0,
        };

        Ok(intrinsic * coefficient)
    }
    #[inline(always)]
    pub fn credit(&self) -> f64 {
        match self.position {
            Position::Long => -self.premium.0,
            Position::Short => self.premium.0,
        }
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        let intrinsic = self.payoff(spot)?;

        let coefficient = match self.position {
            Position::Long => 1.0,
            Position::Short => -1.0,
        };

        Ok((intrinsic * coefficient) + self.credit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[test]
    fn test_build_option_leg() {
        let strike = Strike(80.0);
        let premium = Premium(5.0);
        let kind = OptionKind::Call;
        let position = Position::Long;

        let option_leg = OptionLeg::build(kind, position, strike, premium);

        assert!(option_leg.is_ok());

        let option_leg = option_leg.unwrap();
        assert_eq!(option_leg.kind, OptionKind::Call);
        assert_eq!(option_leg.position, Position::Long);
        assert_eq!(option_leg.strike, Strike(80.0));
        assert_eq!(option_leg.premium, Premium(5.0));
    }

    #[test]
    #[should_panic(expected = "InvalidStrike")]
    fn test_build_option_leg_invalid_strikes() {
        let strike = Strike(-80.0);
        let premium = Premium(5.0);
        let kind = OptionKind::Call;
        let position = Position::Long;

        let _option_leg = OptionLeg::build(kind, position, strike, premium).unwrap();
    }

    #[test]
    #[should_panic(expected = "InvalidPremium")]
    fn test_build_option_leg_invalid_premium() {
        let strike = Strike(80.0);
        let premium = Premium(-5.0);
        let kind = OptionKind::Call;
        let position = Position::Long;

        let _option_leg = OptionLeg::build(kind, position, strike, premium).unwrap();
    }

    #[fixture]
    fn option_leg() -> OptionLeg {
        let kind = OptionKind::Call;
        let position = Position::Long;
        let strike = Strike(80.0);
        let premium = Premium(5.0);
        OptionLeg::build(kind, position, strike, premium).unwrap()
    }

    #[rstest]
    #[case(Spot(80.0), 0.0)]
    #[case(Spot(100.0), 20.0)]
    #[case(Spot(50.0), 0.0)]
    fn test_payoff(option_leg: OptionLeg, #[case] spot: Spot, #[case] expected: f64) {
        assert_eq!(option_leg.payoff(spot).unwrap(), expected);
    }

    #[rstest]
    #[case(Spot(80.0), -5.0)]
    #[case(Spot(100.0), 15.0)]
    fn test_pnl(option_leg: OptionLeg, #[case] spot: Spot, #[case] expected: f64) {
        assert_eq!(option_leg.pnl(spot).unwrap(), expected);
    }

    #[test]
    fn test_payoff_invalid_spot() {
        let spot = Spot(-80.0);
        let strike = Strike(80.0);
        let premium = Premium(5.0);
        let kind = OptionKind::Call;
        let position = Position::Long;

        let option_leg = OptionLeg::build(kind, position, strike, premium).unwrap();

        let payoff = option_leg.payoff(spot);
        assert!(payoff.is_err());
    }

    #[test]
    fn test_pnl_invalid_spot() {
        let spot = Spot(-80.0);
        let strike = Strike(80.0);
        let premium = Premium(5.0);
        let kind = OptionKind::Call;
        let position = Position::Long;

        let option_leg = OptionLeg::build(kind, position, strike, premium).unwrap();

        let pnl = option_leg.pnl(spot);
        assert!(pnl.is_err());
    }
}
