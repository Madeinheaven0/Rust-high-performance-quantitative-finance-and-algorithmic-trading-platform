//! # The Module for the strategies use when the market is in range

use crate::payoffs::domain_types::{OptionKind, Position, Premium, Spot, Strike};
use crate::errors::StrategyError;
use crate::payoffs::leg::OptionLeg;

pub trait RangeStrategy {
    fn payoff(&self, spot: Spot) -> Result<f64, StrategyError>;
    fn pnl(&self, spot: Spot) -> Result<f64, StrategyError>;
}

/// # The Basic Structure of a Iron Strategy
pub struct Iron {
    pub legs: [OptionLeg; 4],
}

impl RangeStrategy for Iron {
    fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        if spot.0 <= 0.0 {
            return Err(StrategyError::InvalidSpot(spot.0));
        }

        self.legs
            .iter()
            .try_fold(
            0.0, |acc, leg| {
                    Ok(acc + leg.payoff(spot)?)
                }
        )
    }

    fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.legs
        .iter()
        .try_fold(
            0.0, |acc, leg| {
                Ok(acc + leg.pnl(spot)?)
            }
        )
    }
}

/// ## The Iron Condor Structure
pub struct IronCondor(Iron);

impl IronCondor {
    pub fn build(
        long_put_strike: Strike,
        long_put_premium: Premium,
        short_put_strike: Strike,
        short_put_premium: Premium,
        short_call_strike: Strike,
        short_call_premium: Premium,
        long_call_strike: Strike,
        long_call_premium: Premium,
    ) -> Result<Self, StrategyError> {
        if long_put_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(long_put_strike.0));
        }
        if short_put_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(short_put_strike.0));
        }
        if long_call_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(long_call_strike.0));
        }
        if short_call_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(short_call_strike.0));
        }

        if long_put_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(long_put_premium.0));
        }
        if short_put_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(short_put_premium.0));
        }
        if long_call_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(long_call_premium.0));
        }
        if short_call_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(short_call_premium.0));
        }

        let iron_condor_assertion = (long_put_strike.0 < short_put_strike.0)
            && (short_put_strike.0 < short_call_strike.0)
            && (short_call_strike.0 < long_call_strike.0);

        if !iron_condor_assertion {
            return Err(StrategyError::InvalidIronCondorStrikes {
                k1: long_put_strike.0,
                k2: short_put_strike.0,
                k3: short_call_strike.0,
                k4: long_call_strike.0,
            });
        }

        let long_put_option = OptionLeg::build(
            OptionKind::Put,
            Position::Long,
            long_put_strike,
            long_put_premium,
        )?;

        let short_put_option = OptionLeg::build(
            OptionKind::Put,
            Position::Short,
            short_put_strike,
            short_put_premium,
        )?;

        let short_call_option = OptionLeg::build(
            OptionKind::Call,
            Position::Short,
            short_call_strike,
            short_call_premium,
        )?;

        let long_call_option = OptionLeg::build(
            OptionKind::Call,
            Position::Long,
            long_call_strike,
            long_call_premium,
        )?;

        let legs = [
            long_put_option,
            short_put_option,
            short_call_option,
            long_call_option,
        ];

        Ok(IronCondor(Iron { legs }))
    }

    pub fn credit(&self) -> f64 {
        self.0.legs.iter().map(|leg| leg.credit()).sum()
    }
}

impl RangeStrategy for IronCondor {
    fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.payoff(spot)
    }
    
    fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.pnl(spot)
    }
}

/// The Iron Butterfly Structure
pub struct IronButterfly(Iron);

impl IronButterfly {
    pub fn build(
        long_put_strike: Strike,
        long_put_premium: Premium,
        short_strike: Strike,
        short_put_premium: Premium,
        short_call_premium: Premium,
        long_call_strike: Strike,
        long_call_premium: Premium,
    ) -> Result<Self, StrategyError> {
        if long_put_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(long_put_strike.0));
        }
        if long_put_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(long_put_premium.0));
        }
        if short_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(short_strike.0));
        }
        if short_call_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(short_call_premium.0));
        }
        if long_call_strike.0 < 0.0 {
            return Err(StrategyError::InvalidStrike(long_call_strike.0));
        }
        if short_put_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(short_put_premium.0));
        }
        if long_call_premium.0 < 0.0 {
            return Err(StrategyError::InvalidPremium(long_call_premium.0));
        }

        let iron_butterfly_assertion =
            long_put_strike.0 < short_strike.0 && short_strike.0 < long_call_strike.0;

        if !iron_butterfly_assertion {
            return Err(StrategyError::InvalidIronButterflyStrikes {
                k1: long_put_strike.0,
                k_atm: short_strike.0,
                k3: long_call_strike.0,
            });
        }

        let long_put_option = OptionLeg::build(
            OptionKind::Put,
            Position::Long,
            long_put_strike,
            long_put_premium,
        )?;

        let short_put_option = OptionLeg::build(
            OptionKind::Put,
            Position::Short,
            short_strike,
            short_put_premium,
        )?;

        let short_call_option = OptionLeg::build(
            OptionKind::Call,
            Position::Short,
            short_strike,
            short_call_premium,
        )?;

        let long_call_option = OptionLeg::build(
            OptionKind::Call,
            Position::Long,
            long_call_strike,
            long_call_premium,
        )?;

        let legs = [
            long_put_option,
            short_put_option,
            short_call_option,
            long_call_option,
        ];

        Ok(Self(Iron { legs }))
    }

    pub fn credit(&self) -> f64 {
        self.0.legs.iter().map(|leg| leg.credit()).sum()
    }
}

impl RangeStrategy for IronButterfly {
    fn payoff(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.payoff(spot)
    }
    
    fn pnl(&self, spot: Spot) -> Result<f64, StrategyError> {
        self.0.pnl(spot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[test]
    fn test_iron_condor_build() {
        let long_put_strike = Strike(70.0);
        let long_put_premium = Premium(2.0);
        let short_put_strike = Strike(80.0);
        let short_put_premium = Premium(3.0);
        let short_call_strike = Strike(90.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        let iron_condor = IronCondor::build(
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_condor.is_ok());

        let iron_condor = iron_condor.unwrap();

        assert_eq!(iron_condor.0.legs[0].strike.0, long_put_strike.0);
        assert_eq!(iron_condor.0.legs[1].strike.0, short_put_strike.0);
        assert_eq!(iron_condor.0.legs[2].strike.0, short_call_strike.0);
        assert_eq!(iron_condor.0.legs[3].strike.0, long_call_strike.0);
    }

    #[test]
    fn test_iron_condor_build_invalid_strike() {
        let long_put_strike = Strike(70.0);
        let long_put_premium = Premium(2.0);
        let short_put_strike = Strike(-80.0);
        let short_put_premium = Premium(3.0);
        let short_call_strike = Strike(90.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        let iron_condor = IronCondor::build(
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_condor.is_err());
    }

    #[test]
    fn test_iron_condor_build_invalid_premium() {
        let long_put_strike = Strike(70.0);
        let long_put_premium = Premium(2.0);
        let short_put_strike = Strike(80.0);
        let short_put_premium = Premium(-3.0);
        let short_call_strike = Strike(90.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        let iron_condor = IronCondor::build(
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_condor.is_err());
    }

    #[test]
    fn test_iron_condor_build_invalid_strike_configuration() {
        let long_put_strike = Strike(100.0);
        let long_put_premium = Premium(2.0);
        let short_put_strike = Strike(90.0);
        let short_put_premium = Premium(3.0);
        let short_call_strike = Strike(80.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(70.0);
        let long_call_premium = Premium(5.0);

        let iron_condor = IronCondor::build(
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_condor.is_err());
    }

    #[fixture]
    fn iron_condor() -> IronCondor {
        let long_put_strike = Strike(70.0);
        let long_put_premium = Premium(2.0);
        let short_put_strike = Strike(80.0);
        let short_put_premium = Premium(3.0);
        let short_call_strike = Strike(90.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        IronCondor::build(
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        )
        .unwrap()
    }

    #[rstest]
    #[case(Spot(80.), 0.)]
    #[case(Spot(100.), -10.)]
    #[case(Spot(120.), -10.)]
    #[case(Spot(150.), -10.)]
    fn test_iron_condor_payoff(iron_condor: IronCondor, #[case] spot: Spot, #[case] expected: f64) {
        assert_eq!(iron_condor.payoff(spot).unwrap(), expected);
    }

    #[rstest]
    #[case(Spot(80.), 0.)]
    #[case(Spot(100.), -10.)]
    #[case(Spot(120.), -10.)]
    #[case(Spot(150.), -10.)]
    fn test_iron_condor_pnl(iron_condor: IronCondor, #[case] spot: Spot, #[case] expected: f64) {
        assert_eq!(iron_condor.pnl(spot).unwrap(), expected);
    }

    #[test]
    fn test_iron_butterfly_build() {
        let long_put_strike = Strike(80.0);
        let long_put_premium = Premium(2.0);
        let short_strike = Strike(90.0);
        let short_put_premium = Premium(3.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        let iron_butterfly = IronButterfly::build(
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_butterfly.is_ok());

        let iron_butterfly = iron_butterfly.unwrap();

        assert_eq!(iron_butterfly.0.legs[0].strike, long_put_strike);
        assert_eq!(iron_butterfly.0.legs[1].strike, short_strike);
        assert_eq!(iron_butterfly.0.legs[2].strike, short_strike);
        assert_eq!(iron_butterfly.0.legs[3].strike, long_call_strike);
    }

    #[test]
    fn test_iron_butterfly_build_invalid_strike() {
        let long_put_strike = Strike(-80.0);
        let long_put_premium = Premium(2.0);
        let short_strike = Strike(90.0);
        let short_put_premium = Premium(3.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        let iron_butterfly = IronButterfly::build(
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_butterfly.is_err());
    }

    #[test]
    fn test_iron_butterfly_build_invalid_premium() {
        let long_put_strike = Strike(80.0);
        let long_put_premium = Premium(2.0);
        let short_strike = Strike(90.0);
        let short_put_premium = Premium(3.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(-5.0);

        let iron_butterfly = IronButterfly::build(
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_butterfly.is_err());
    }

    #[test]
    fn test_iron_butterfly_build_invalid_strike_configuration() {
        let long_put_strike = Strike(100.0);
        let long_put_premium = Premium(2.0);
        let short_strike = Strike(90.0);
        let short_put_premium = Premium(3.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        let iron_butterfly = IronButterfly::build(
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        );

        assert!(iron_butterfly.is_err());
    }

    #[fixture]
    fn iron_butterfly() -> IronButterfly {
        let long_put_strike = Strike(80.0);
        let long_put_premium = Premium(2.0);
        let short_strike = Strike(90.0);
        let short_put_premium = Premium(3.0);
        let short_call_premium = Premium(4.0);
        let long_call_strike = Strike(100.0);
        let long_call_premium = Premium(5.0);

        IronButterfly::build(
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium,
        )
        .unwrap()
    }

    #[rstest]
    #[case(Spot(80.), -10.)]
    #[case(Spot(100.), -10.)]
    #[case(Spot(120.), -10.)]
    #[case(Spot(150.), -10.)]
    fn test_iron_butterfly_payoff(
        iron_butterfly: IronButterfly,
        #[case] spot: Spot,
        #[case] expected: f64,
    ) {
        assert_eq!(iron_butterfly.payoff(spot).unwrap(), expected);
    }

    #[rstest]
    #[case(Spot(80.), -10.)]
    #[case(Spot(100.), -10.)]
    #[case(Spot(120.), -10.)]
    #[case(Spot(150.), -10.)]
    fn test_iron_butterfly_pnl(
        iron_butterfly: IronButterfly,
        #[case] spot: Spot,
        #[case] expected: f64,
    ) {
        assert_eq!(iron_butterfly.pnl(spot).unwrap(), expected);
    }
}
