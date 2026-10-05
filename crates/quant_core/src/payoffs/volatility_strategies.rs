//!
//!

use crate::payoffs::domain_types::{OptionKind, Position, Premium, Spot, Strike};
use crate::errors::StrategyError;
use crate::payoffs::leg::OptionLeg;

///
pub struct Straddle {
    pub legs: [OptionLeg; 2],
}

impl Straddle {
    pub fn build(
        strike: Strike,
        call_premium: Premium,
        put_premium: Premium,
        position: Position,
    ) -> Result<Self, StrategyError> {
        if strike.0 <= 0. {
            return Err(StrategyError::InvalidStrike(strike.0));
        }

        if call_premium.0 <= 0. {
            return Err(StrategyError::InvalidPremium(call_premium.0));
        }

        if put_premium.0 <= 0. {
            return Err(StrategyError::InvalidPremium(put_premium.0));
        }

        match position {
            Position::Long => {
                let call_option =
                    OptionLeg::build(OptionKind::Call, position, strike, call_premium)?;

                let put_option = OptionLeg::build(OptionKind::Put, position, strike, put_premium)?;

                Ok(Self {
                    legs: [call_option, put_option],
                })
            }
            Position::Short => {
                let call_option =
                    OptionLeg::build(OptionKind::Call, position, strike, call_premium)?;

                let put_option = OptionLeg::build(OptionKind::Put, position, strike, put_premium)?;

                Ok(Self {
                    legs: [call_option, put_option],
                })
            }
        }
    }

    pub fn credit(&self) -> f64 {
        self.legs[0].credit() + self.legs[1].credit()
    }

    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        Ok(self.legs.iter().map(|leg| leg.payoff(spot).unwrap()).sum())
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        Ok(self.legs.iter().map(|leg| leg.pnl(spot).unwrap()).sum())
    }
}

pub struct Strangle {
    legs: [OptionLeg; 2],
}

impl Strangle {
    pub fn build(
        call_strike: Strike,
        call_premium: Premium,
        put_strike: Strike,
        put_premium: Premium,
        position: Position,
    ) -> Result<Self, StrategyError> {
        if call_strike.0 <= 0. {
            return Err(StrategyError::InvalidStrike(call_strike.0));
        }

        if put_strike.0 <= 0. {
            return Err(StrategyError::InvalidStrike(put_strike.0));
        }

        if call_premium.0 <= 0. {
            return Err(StrategyError::InvalidPremium(call_premium.0));
        }

        if put_premium.0 <= 0. {
            return Err(StrategyError::InvalidPremium(put_premium.0));
        }

        if call_strike.0 <= put_strike.0 {
            return Err(StrategyError::InvalidStrangleStrikes {
                call_strike: call_strike.0,
                put_strike: put_strike.0,
            });
        }

        match position {
            Position::Long => {
                let call_option =
                    OptionLeg::build(OptionKind::Call, position, call_strike, call_premium)?;

                let put_option =
                    OptionLeg::build(OptionKind::Put, position, put_strike, put_premium)?;

                Ok(Self {
                    legs: [call_option, put_option],
                })
            }

            Position::Short => {
                let call_option =
                    OptionLeg::build(OptionKind::Call, position, call_strike, call_premium)?;

                let put_option =
                    OptionLeg::build(OptionKind::Put, position, put_strike, put_premium)?;

                Ok(Self {
                    legs: [call_option, put_option],
                })
            }
        }
    }

    pub fn credit(&self) -> f64 {
        self.legs[0].credit() + self.legs[1].credit()
    }

    pub fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        Ok(self.legs.iter().map(|leg| leg.payoff(spot).unwrap()).sum())
    }

    pub fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        Ok(self.legs.iter().map(|leg| leg.pnl(spot).unwrap()).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[test]
    fn test_build_straddle() {
        let strike = Strike(100.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        let straddle = Straddle::build(strike, call_premium, put_premium, position);

        assert!(straddle.is_ok());

        let straddle = straddle.expect("straddle not built");

        assert_eq!(straddle.legs[0].strike, straddle.legs[1].strike);
        assert_eq!(straddle.legs[0].premium, call_premium);
        assert_eq!(straddle.legs[1].premium, put_premium);
        assert_eq!(straddle.legs[0].position, straddle.legs[1].position);
    }

    #[test]
    fn test_build_straddle_invalid_strike() {
        let strike = Strike(-100.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        let straddle = Straddle::build(strike, call_premium, put_premium, position);

        assert!(straddle.is_err());
    }

    #[test]
    fn test_build_straddle_invalid_premium() {
        let strike = Strike(100.);
        let call_premium = Premium(-10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        let straddle = Straddle::build(strike, call_premium, put_premium, position);

        assert!(straddle.is_err());
    }

    #[fixture]
    fn straddle_test() -> Straddle {
        let strike = Strike(100.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        Straddle::build(strike, call_premium, put_premium, position).unwrap()
    }

    #[rstest]
    #[case(Spot(80.), 20.)]
    #[case(Spot(100.), 0.)]
    #[case(Spot(120.), 20.)]
    #[case(Spot(150.), 50.)]
    fn test_straddle_payoff(
        straddle_test: Straddle,
        #[case] spot: Spot,
        #[case] expected_value: f64,
    ) {
        assert_eq!(straddle_test.payoff(spot).unwrap(), expected_value);
    }

    #[rstest]
    #[case(Spot(80.), 2.)]
    #[case(Spot(100.), -18.)]
    #[case(Spot(120.), 2.)]
    #[case(Spot(150.), 32.)]
    fn test_straddle_pnl(straddle_test: Straddle, #[case] spot: Spot, #[case] expected_value: f64) {
        assert_eq!(straddle_test.pnl(spot).unwrap(), expected_value);
    }

    #[test]
    fn test_build_strangle() {
        let call_strike = Strike(100.);
        let put_strike = Strike(90.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        let strangle =
            Strangle::build(call_strike, call_premium, put_strike, put_premium, position);

        assert!(strangle.is_ok());

        let strangle = strangle.expect("strangle not built");

        assert_eq!(strangle.legs[0].strike, call_strike);
        assert_eq!(strangle.legs[1].strike, put_strike);
        assert_eq!(strangle.legs[0].premium, call_premium);
        assert_eq!(strangle.legs[1].premium, put_premium);
        assert_eq!(strangle.legs[0].position, position);
    }

    #[test]
    #[should_panic(expected = "InvalidStrike")]
    fn test_build_strangle_invalid_strike() {
        let call_strike = Strike(-100.);
        let put_strike = Strike(90.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        let _strangle =
            Strangle::build(call_strike, call_premium, put_strike, put_premium, position).unwrap();
    }

    #[test]
    #[should_panic(expected = "InvalidPremium")]
    fn test_build_strangle_invalid_premium() {
        let call_strike = Strike(100.);
        let put_strike = Strike(90.);
        let call_premium = Premium(10.);
        let put_premium = Premium(-8.);
        let position = Position::Long;

        let _strangle =
            Strangle::build(call_strike, call_premium, put_strike, put_premium, position).unwrap();
    }

    #[fixture]
    fn strangle_test() -> Strangle {
        let call_strike = Strike(100.);
        let put_strike = Strike(90.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        Strangle::build(call_strike, call_premium, put_strike, put_premium, position).unwrap()
    }

    #[rstest]
    #[case(Spot(80.), 10.)]
    #[case(Spot(100.), 0.)]
    #[case(Spot(120.), 20.)]
    #[case(Spot(150.), 50.)]
    fn test_strangle_payoff(
        strangle_test: Strangle,
        #[case] spot: Spot,
        #[case] expected_value: f64,
    ) {
        assert_eq!(strangle_test.payoff(spot).unwrap(), expected_value);
    }

    #[rstest]
    #[case(Spot(80.), -8.)]
    #[case(Spot(100.), -18.)]
    #[case(Spot(120.), 2.)]
    #[case(Spot(150.), 32.)]
    fn test_strangle_pnl(strangle_test: Strangle, #[case] spot: Spot, #[case] expected_value: f64) {
        assert_eq!(strangle_test.pnl(spot).unwrap(), expected_value);
    }

    #[test]
    fn test_strangle_invalid_strangle_prices() {
        let call_strike = Strike(10.);
        let put_strike = Strike(90.);
        let call_premium = Premium(10.);
        let put_premium = Premium(8.);
        let position = Position::Long;

        let _strangle =
            Strangle::build(call_strike, call_premium, put_strike, put_premium, position);
        assert!(_strangle.is_err());
    }
}
