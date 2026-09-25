//! # The Iron series strategies
//! All the structuration and computation for the payoff of advanced strategies of option used when we are in range period

use super::categorical_options::{CallPutCategory, LongShortCategory};
use super::errors::PriceError;
use super::spread::{BearSpread, BullSpread};
use super::volatility::{Straddle, Strangle};

/// The structure of an iron condor
///
/// Implements a method to build and compute pub yoff of that strategy
///
/// ## Examples
///
/// ```rust
/// use quant_core::payoffs::range_bound::IronCondor;
/// use quant_core::payoffs::errors::PriceError;
///
/// fn main() -> Result<(), PriceError> {
///     let iron_condor = IronCondor::build(
///         100.,
///         50.,
///         60.,
///         70.,
///         80.,
///     )?;
///
///     assert_eq!(iron_condor.payoff(), -10.);
///
///     Ok(())
/// }
/// ```
pub struct IronCondor {
    pub spot_price: f64,
    pub strike_down1: f64,
    pub strike_up1: f64,
    pub strike_down2: f64,
    pub strike_up2: f64,
}

/// The structure of an iron butterfly
///
/// Implements a method to build and compute pub yoff of that strategy
///
/// ## Examples
///
/// ```rust
/// use quant_core::payoffs::range_bound::IronButterfly;
/// use quant_core::payoffs::errors::PriceError;
///
/// fn main() -> Result<(), PriceError> {
///     let iron_butterfly = IronButterfly::build(
///         100,
///         80,
///         70,
///         90,
///     )?;
///
///     assert_eq!(iron_butterfly.payoff(), -10);
///
///     Ok(())
/// }
/// ```
pub struct IronButterfly {
    pub spot_price: f64,
    pub main_strike: f64,
    pub strike_down: f64,
    pub strike_up: f64,
}

impl IronCondor {
    /// Create the `IronCondor` strategy
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price` or at least one of the strike is negative
    /// of if `strike_down1`is greater or equal to `strike_up1` or
    /// if `strike_down2` is greater or equal to `strike_up2`
    pub fn build(
        spot_price: impl Into<f64>,
        strike_down1: impl Into<f64>,
        strike_up1: impl Into<f64>,
        strike_down2: impl Into<f64>,
        strike_up2: impl Into<f64>,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_down1 = strike_down1.into();
        let strike_up1 = strike_up1.into();
        let strike_down2 = strike_down2.into();
        let strike_up2 = strike_up2.into();

        if spot_price < 0. {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_down1 < 0. || strike_up1 < 0. || strike_down2 < 0. || strike_up2 < 0. {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_down1 >= strike_up1 {
            return Err(PriceError::StrikeConfigurationError(
                strike_down1,
                strike_up1,
            ));
        }

        if strike_down2 >= strike_up2 {
            return Err(PriceError::StrikeConfigurationError(
                strike_down2,
                strike_up2,
            ));
        }

        if strike_down1 >= strike_down2 {
            return Err(PriceError::StrikeConfigurationError(
                strike_down1,
                strike_down2,
            ));
        }

        if strike_up1 >= strike_up2 {
            return Err(PriceError::StrikeConfigurationError(strike_up1, strike_up2));
        }

        if strike_down1 >= strike_up2 {
            return Err(PriceError::StrikeConfigurationError(
                strike_down1,
                strike_up2,
            ));
        }

        if strike_up1 >= strike_down2 {
            return Err(PriceError::StrikeConfigurationError(
                strike_up1,
                strike_down2,
            ));
        }

        Ok(Self {
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        })
    }

    /// Calculate the payoff of the `IronCondor`
    pub fn payoff(&self) -> f64 {
        let first_payoff = BullSpread::build(
            self.spot_price,
            self.strike_down1,
            self.strike_up1,
            CallPutCategory::Put,
        )
        .unwrap()
        .payoff();

        let second_payoff = BearSpread::build(
            self.spot_price,
            self.strike_down2,
            self.strike_up2,
            CallPutCategory::Call,
        )
        .unwrap()
        .payoff();

        first_payoff + second_payoff
    }
}

impl IronButterfly {
    /// Create the IronButterfly
    ///
    /// # Errors
    ///
    /// Returns and error if `spot_price` or at least one of the strike price is negative
    /// or
    /// if `main_strike` is not between `strike_down` and `strike_up`
    pub fn build(
        spot_price: impl Into<f64>,
        main_strike: impl Into<f64>,
        strike_down: impl Into<f64>,
        strike_up: impl Into<f64>,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let main_strike = main_strike.into();
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();

        if spot_price < 0. {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if main_strike <= 0. || strike_down <= 0. || strike_up <= 0. {
            return Err(PriceError::StrikePriceNegative);
        }

        if main_strike <= strike_down {
            return Err(PriceError::StrikeConfigurationError(
                main_strike,
                strike_down,
            ));
        }

        if strike_down >= strike_up {
            return Err(PriceError::StrikeConfigurationError(strike_down, strike_up));
        }

        if main_strike >= strike_up {
            return Err(PriceError::StrikeConfigurationError(main_strike, strike_up));
        }

        Ok(Self {
            spot_price,
            main_strike,
            strike_down,
            strike_up,
        })
    }

    pub fn payoff(&self) -> f64 {
        let straddle =
            Straddle::build(self.spot_price, self.main_strike, LongShortCategory::Short).unwrap();

        let strangle = Strangle::build(
            self.spot_price,
            self.strike_down,
            self.strike_up,
            LongShortCategory::Long,
        )
        .unwrap();

        let first_payoff = straddle.payoff();
        let second_payoff = strangle.payoff();

        first_payoff + second_payoff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_iron_condor() {
        let spot_price = 100.;
        let strike_down1 = 50.;
        let strike_up1 = 60.;
        let strike_down2 = 70.;
        let strike_up2 = 80.;

        let condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        )
        .unwrap();
        assert_eq!(condor.spot_price, spot_price);
        assert_eq!(condor.strike_down1, strike_down1);
        assert_eq!(condor.strike_up1, strike_up1);
        assert_eq!(condor.strike_down2, strike_down2);
        assert_eq!(condor.strike_up2, strike_up2);
    }

    #[test]
    #[should_panic]
    fn test_create_iron_condor_with_wrong_spot_price() {
        let spot_price = -100.;
        let strike_down1 = 60.;
        let strike_up1 = 80.;
        let strike_down2 = 50.;
        let strike_up2 = 70.;

        let _condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_create_iron_condor_with_wrong_strike_down1() {
        let spot_price = 100.;
        let strike_down1 = -60.;
        let strike_up1 = 80.;
        let strike_down2 = 50.;
        let strike_up2 = 70.;

        let _condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "StrikeConfigurationError")]
    fn test_create_iron_condor_with_bad_strike_configuration() {
        let spot_price = 100.;
        let strike_down1 = 90.;
        let strike_up1 = 80.;
        let strike_down2 = 50.;
        let strike_up2 = 70.;

        let _condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        )
        .unwrap();
    }

    #[test]
    fn test_iron_condor_payoff() {
        let spot_price = 100.;
        let strike_down1 = 50.;
        let strike_up1 = 60.;
        let strike_down2 = 70.;
        let strike_up2 = 80.;

        let condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        )
        .unwrap();

        assert_eq!(condor.payoff(), -10.);
    }

    #[test]
    fn test_build_iron_butterfly() {
        let spot_price = 100.;
        let main_strike = 80.;
        let strike_down1 = 70.;
        let strike_up1 = 90.;

        let iron_butterfly =
            IronButterfly::build(spot_price, main_strike, strike_down1, strike_up1).unwrap();

        assert_eq!(iron_butterfly.main_strike, main_strike);
        assert_eq!(iron_butterfly.strike_down, strike_down1);
        assert_eq!(iron_butterfly.strike_up, strike_up1);
        assert_eq!(iron_butterfly.spot_price, spot_price);
    }

    #[test]
    #[should_panic(expected = "SpotPriceNegative")]
    fn test_build_iron_butterfly_with_wrong_spot_price() {
        let spot_price = -100.;
        let main_strike = 80.;
        let strike_down1 = 70.;
        let strike_up1 = 90.;

        let _iron_butterfly =
            IronButterfly::build(spot_price, main_strike, strike_down1, strike_up1).unwrap();
    }

    #[test]
    #[should_panic(expected = "StrikeConfigurationError")]
    fn test_build_iron_butterfly_with_wrong_configuration() {
        let spot_price = 100.;
        let main_strike = 80.;
        let strike_down1 = 70.;
        let strike_up1 = 80.;

        let _iron_butterfly =
            IronButterfly::build(spot_price, main_strike, strike_down1, strike_up1).unwrap();
    }

    #[test]
    fn test_iron_butterfly_payoff() {
        let spot_price = 100.;
        let main_strike = 80.;
        let strike_down1 = 70.;
        let strike_up1 = 90.;

        let iron_butterfly =
            IronButterfly::build(spot_price, main_strike, strike_down1, strike_up1).unwrap();

        assert_eq!(iron_butterfly.payoff(), -10.);
    }
}
