//! # Strategies used when we have a trend

use crate::payoffs::domain_types::{OptionKind, Position, Premium, Spot, Strike};
use crate::payoffs::errors::StrategyError;
use crate::payoffs::leg::OptionLeg;

/// The basic structure of a Spread
#[derive(Debug, PartialEq, Clone)]
struct VerticalSpread {
    legs: [OptionLeg; 2],
}

impl VerticalSpread {
    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        if spot.0 <= 0.0 {
            return Err(StrategyError::InvalidSpot(spot.0));
        }

        Ok(self
            .legs
            .iter()
            .map(|leg| leg.payoff(spot).unwrap())
            .sum::<f64>())
    }

    pub fn credit(&self) -> f64 {
        self.legs.iter().map(|leg| leg.credit()).sum::<f64>()
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        Ok(self
            .legs
            .iter()
            .map(|leg| leg.pnl(spot).unwrap())
            .sum::<f64>())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BullCallSpread(VerticalSpread);

impl BullCallSpread {
    pub fn build(
        strike_up: Strike,
        strike_down: Strike,
        premium_up: Premium,
        premium_down: Premium,
        position: Position,
    ) -> Result<Self, StrategyError> {
        if strike_up.0 <= strike_down.0 {
            return Err(StrategyError::InvalidSpreadStrikes {
                strike_up: strike_up.0,
                strike_down: strike_down.0,
            });
        }

        let short_position = match position {
            Position::Long => Position::Short,
            Position::Short => Position::Long,
        };

        let long_leg = OptionLeg::build(OptionKind::Call, position, strike_down, premium_down)?;

        let short_leg = OptionLeg::build(OptionKind::Call, short_position, strike_up, premium_up)?;

        Ok(Self(VerticalSpread {
            legs: [long_leg, short_leg],
        }))
    }

    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.payoff(spot)
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.pnl(spot)
    }
}

pub struct BullPutSpread(VerticalSpread);

impl BullPutSpread {
    pub fn build(
        strike_up: Strike,
        strike_down: Strike,
        premium_up: Premium,
        premium_down: Premium,
        position: Position,
    ) -> Result<Self, StrategyError> {
        if strike_up.0 <= strike_down.0 {
            return Err(StrategyError::InvalidSpreadStrikes {
                strike_up: strike_up.0,
                strike_down: strike_down.0,
            });
        }

        let short_position = match position {
            Position::Long => Position::Short,
            Position::Short => Position::Long,
        };

        let long_leg = OptionLeg::build(OptionKind::Put, position, strike_up, premium_up)?;

        let short_leg =
            OptionLeg::build(OptionKind::Put, short_position, strike_down, premium_down)?;

        Ok(Self(VerticalSpread {
            legs: [long_leg, short_leg],
        }))
    }

    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.payoff(spot)
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.pnl(spot)
    }
}
pub struct BearCallSpread(VerticalSpread);

impl BearCallSpread {
    pub fn build(
        strike_up: Strike,
        strike_down: Strike,
        premium_up: Premium,
        premium_down: Premium,
        position: Position,
    ) -> Result<Self, StrategyError> {
        if strike_up.0 <= strike_down.0 {
            return Err(StrategyError::InvalidSpreadStrikes {
                strike_up: strike_up.0,
                strike_down: strike_down.0,
            });
        }

        let short_position = match position {
            Position::Long => Position::Short,
            Position::Short => Position::Long,
        };

        let long_leg = OptionLeg::build(OptionKind::Call, position, strike_up, premium_up)?;

        let short_leg =
            OptionLeg::build(OptionKind::Call, short_position, strike_down, premium_down)?;

        Ok(Self(VerticalSpread {
            legs: [long_leg, short_leg],
        }))
    }

    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.payoff(spot)
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.pnl(spot)
    }
}

struct BearPutSpread(VerticalSpread);

impl BearPutSpread {
    pub fn build(
        strike_up: Strike,
        strike_down: Strike,
        premium_up: Premium,
        premium_down: Premium,
        position: Position,
    ) -> Result<Self, StrategyError> {
        if strike_up.0 <= strike_down.0 {
            return Err(StrategyError::InvalidSpreadStrikes {
                strike_up: strike_up.0,
                strike_down: strike_down.0,
            });
        }

        let short_position = match position {
            Position::Long => Position::Short,
            Position::Short => Position::Long,
        };

        let long_leg = OptionLeg::build(OptionKind::Put, position, strike_up, premium_up)?;

        let short_leg = OptionLeg::build(OptionKind::Put, short_position, strike_down, premium_down)?;

        Ok(Self(VerticalSpread {
            legs: [long_leg, short_leg],
        }))
    }

    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.payoff(spot)
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.pnl(spot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[test]
    fn test_bull_spread() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(7.0);
        let premium_down = Premium(10.0);
        let position = Position::Long;
        let kind = OptionKind::Call;

        let bull_spread =
            BullCallSpread::build(strike_up, strike_down, premium_up, premium_down, position);

        assert!(bull_spread.is_ok());

        let bull_spread = bull_spread.unwrap();

        assert_eq!(bull_spread.0.legs[0].strike, strike_down);
        assert_eq!(bull_spread.0.legs[1].strike, strike_up);
        assert_eq!(bull_spread.0.legs[0].premium, premium_down);
        assert_eq!(bull_spread.0.legs[1].premium, premium_up);
    }

    #[test]
    fn test_bull_spread_invalid_strike() {
        let strike_up = Strike(-100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;
        let kind = OptionKind::Call;

        let bull_spread =
            BullCallSpread::build(strike_up, strike_down, premium_up, premium_down, position);

        assert!(bull_spread.is_err());
    }

    #[test]
    fn test_bull_spread_invalid_premium() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(-10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;
        let kind = OptionKind::Call;

        let bull_spread =
            BullCallSpread::build(strike_up, strike_down, premium_up, premium_down, position);

        assert!(bull_spread.is_err());
    }

    #[fixture]
    fn bull_spread() -> BullCallSpread {
        let strike_up = Strike(110.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        BullCallSpread::build(strike_up, strike_down, premium_up, premium_down, position).unwrap()
    }

    #[rstest]
    #[case(Spot(100.), 30.0)]
    #[case(Spot(70.0), 0.)]
    #[case(Spot(120.0), 40.)]
    fn test_bull_spread_payoff(
        bull_spread: BullCallSpread,
        #[case] spot: Spot,
        #[case] expected: f64,
    ) {
        assert_eq!(bull_spread.payoff(spot).unwrap(), expected);
    }

    #[test]
    fn test_build_bear_spread() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        let bear_spread =
            BearCallSpread::build(strike_up, strike_down, premium_up, premium_down, position);

        assert!(bear_spread.is_ok());

        let bear_spread = bear_spread.unwrap();

        assert_eq!(bear_spread.0.legs[0].strike, strike_up);
        assert_eq!(bear_spread.0.legs[1].strike, strike_down);
        assert_eq!(bear_spread.0.legs[0].premium, premium_up);
        assert_eq!(bear_spread.0.legs[1].premium, premium_down);
    }

    #[test]
    fn test_build_bear_spread_invalid_premium() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(-10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;
        let kind = OptionKind::Call;

        let bear_spread =
            BearCallSpread::build(strike_up, strike_down, premium_up, premium_down, position);

        assert!(bear_spread.is_err());
    }

    #[test]
    fn test_build_bear_spread_invalid_strike() {
        let strike_up = Strike(-100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;
        let kind = OptionKind::Call;

        let bear_spread =
            BearCallSpread::build(strike_up, strike_down, premium_up, premium_down, position);

        assert!(bear_spread.is_err());
    }

    #[fixture]
    fn bear_call() -> BearCallSpread {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        BearCallSpread::build(strike_up, strike_down, premium_up, premium_down, position).unwrap()
    }

    #[rstest]
    #[case(Spot(100.), -30.0)]
    #[case(Spot(70.0), 0.)]
    #[case(Spot(120.0), -30.)]
    fn test_bear_call_spread_payoff(
        bear_call: BearCallSpread,
        #[case] spot: Spot,
        #[case] expected: f64,
    ) {
        let payoff_res = bear_call.payoff(spot).unwrap();
        let pnl_res = bear_call.pnl(spot).unwrap();
        eprintln!(
            "Spot: {:?} | Payoff obtenu: {} | PnL obtenu: {} | Attendu: {}",
            spot, payoff_res, pnl_res, expected
        );
        assert_eq!(bear_call.payoff(spot).unwrap(), expected);
    }

    #[test]
    fn test_build_bull_put_spread() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        let bull_put_spread = BullPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        );

        assert!(bull_put_spread.is_ok());

        let bull_put_spread = bull_put_spread.unwrap();

        assert_eq!(bull_put_spread.0.legs[0].strike, strike_up);
        assert_eq!(bull_put_spread.0.legs[1].strike, strike_down);
        assert_eq!(bull_put_spread.0.legs[0].premium, premium_up);
        assert_eq!(bull_put_spread.0.legs[1].premium, premium_down);
    }

    #[test]
    fn test_build_bull_put_spread_invalid_premium() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(-7.0);
        let position = Position::Long;

        let bull_put_spread = BullPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        );

        assert!(bull_put_spread.is_err());
    }

    #[test]
    fn test_build_bull_put_spread_invalid_strike() {
        let strike_up = Strike(-100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(7.0);
        let premium_down = Premium(10.0);
        let position = Position::Long;

        let bull_put_spread = BullPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        );

        assert!(bull_put_spread.is_err());
    }

    #[fixture]
    fn bull_put_spread() -> BullPutSpread {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(7.0);
        let premium_down = Premium(10.0);
        let position = Position::Long;

        BullPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        ).unwrap()
    }

    #[rstest]
    #[case(Spot(100.), 0.0)]
    #[case(Spot(70.0), 30.)]
    #[case(Spot(120.0), 0.)]
    fn test_bull_put_spread_payoff(bull_put_spread: BullPutSpread, #[case] spot: Spot, #[case] expected: f64) {
        assert_eq!(bull_put_spread.payoff(spot).unwrap(), expected);
    }

    #[test]
    fn test_bear_put_spread_build() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        let bear_put_spread = BearPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        );

        assert!(bear_put_spread.is_ok());

        let bear_put_spread = bear_put_spread.unwrap();

        assert_eq!(bear_put_spread.0.legs[0].strike, strike_up);
        assert_eq!(bear_put_spread.0.legs[1].strike, strike_down);
        assert_eq!(bear_put_spread.0.legs[0].premium, premium_up);
        assert_eq!(bear_put_spread.0.legs[1].premium, premium_down);
    }

    #[test]
    fn test_build_bear_put_spread_build_invalid_premium() {
        let strike_up = Strike(100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(-10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        let bear_put_spread = BearPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        );

        assert!(bear_put_spread.is_err());
    }

    #[test]
    fn test_build_bear_put_spread_build_invalid_strike() {
        let strike_up = Strike(-100.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        let bear_put_spread = BearPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        );

        assert!(bear_put_spread.is_err());
    }

    #[fixture]
    fn bear_put_spread() -> BearPutSpread {
        let strike_up = Strike(110.0);
        let strike_down = Strike(70.0);
        let premium_up = Premium(10.0);
        let premium_down = Premium(7.0);
        let position = Position::Long;

        BearPutSpread::build(
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            position,
        ).unwrap()
    }

    #[rstest]
    #[case(Spot(100.), 10.0)]
    #[case(Spot(70.0), 40.0)]
    #[case(Spot(120.0), 0.0)]
    fn test_bear_put_spread_payoff(bear_put_spread: BearPutSpread, #[case] spot: Spot, #[case] expected: f64) {
        assert_eq!(bear_put_spread.payoff(spot).unwrap(), expected);
    }
}
