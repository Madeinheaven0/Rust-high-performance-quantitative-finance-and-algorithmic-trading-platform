//! # The Iron series strategies
//! All the structuration and computation for the payoff of advanced strategies of option used when we are in range period

use super::categorical_options::CallPutCategory;
use super::errors::PriceError;
use super::spread::{BearSpread, BullSpread};
use crate::payoffs::basics::BasicOption;

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
    pub call_prime1: f64,
    pub call_prime2: f64,
    pub put_prime1: f64,
    pub put_prime2: f64,
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
    pub main_prime_call: f64,
    pub main_prime_put: f64,
    pub down_prime: f64,
    pub up_prime: f64,
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
        call_prime1: impl Into<f64>,
        call_prime2: impl Into<f64>,
        put_prime1: impl Into<f64>,
        put_prime2: impl Into<f64>,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_down1 = strike_down1.into();
        let strike_up1 = strike_up1.into();
        let strike_down2 = strike_down2.into();
        let strike_up2 = strike_up2.into();
        let call_prime1 = call_prime1.into();
        let call_prime2 = call_prime2.into();
        let put_prime1 = put_prime1.into();
        let put_prime2 = put_prime2.into();

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

        if call_prime1 <= 0. || call_prime2 <= 0. || put_prime1 <= 0. || put_prime2 <= 0.0 {
            return Err(PriceError::PrimePriceError);
        }

        Ok(Self {
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
        })
    }

    /// Calculate the payoff of the `IronCondor`
    pub fn payoff(&self) -> f64 {
        let first_payoff = BullSpread::build(
            self.spot_price,
            self.strike_down1,
            self.strike_up1,
            self.call_prime1,
            self.call_prime2,
            CallPutCategory::Put,
        )
        .unwrap()
        .payoff();

        let second_payoff = BearSpread::build(
            self.spot_price,
            self.strike_down2,
            self.strike_up2,
            self.put_prime1,
            self.put_prime2,
            CallPutCategory::Call,
        )
        .unwrap()
        .payoff();

        first_payoff + second_payoff
    }

    pub fn pnl(&self) -> f64 {
        self.payoff() - self.call_prime1 + self.call_prime2 - self.put_prime1 + self.put_prime2
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
        main_prime_call: impl Into<f64>,
        main_prime_put: impl Into<f64>,
        down_prime: impl Into<f64>,
        up_prime: impl Into<f64>,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let main_strike = main_strike.into();
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();
        let main_prime_call = main_prime_call.into();
        let main_prime_put = main_prime_put.into();
        let down_prime = down_prime.into();
        let up_prime = up_prime.into();

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

        if main_prime_call <= 0. || up_prime <= 0. || down_prime <= 0. || main_prime_put <= 0. {
            return Err(PriceError::PrimePriceError);
        }

        Ok(Self {
            spot_price,
            main_strike,
            strike_down,
            strike_up,
            main_prime_call,
            main_prime_put,
            up_prime,
            down_prime,
        })
    }

    /// Compute the payoff of the strategy
    pub fn payoff(&self) -> f64 {
        let main_put = BasicOption::build(
            self.spot_price,
            self.main_strike,
            self.main_prime_put,
            CallPutCategory::Put,
        )
        .unwrap();
        let put_down = BasicOption::build(
            self.spot_price,
            self.strike_down,
            self.down_prime,
            CallPutCategory::Put,
        )
        .unwrap();
        let main_call = BasicOption::build(
            self.spot_price,
            self.main_strike,
            self.main_prime_call,
            CallPutCategory::Call,
        )
        .unwrap();
        let call_up = BasicOption::build(
            self.spot_price,
            self.strike_up,
            self.up_prime,
            CallPutCategory::Call,
        )
        .unwrap();

        let payoff1 = main_put.payoff();
        let payoff2 = put_down.payoff();
        let payoff3 = main_call.payoff();
        let payoff4 = call_up.payoff();

        payoff1 + payoff2 + payoff3 + payoff4
    }

    pub fn pnl(&self) -> f64 {
        let payoff = self.payoff();

        payoff + self.main_prime_put + self.main_prime_call - self.down_prime - self.up_prime
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
        let call_prime1 = 2.;
        let call_prime2 = 2.;
        let put_prime1 = 1.;
        let put_prime2 = 1.;

        let condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
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
        let call_prime1 = 2.;
        let call_prime2 = 2.;
        let put_prime1 = 1.;
        let put_prime2 = 1.;

        let _condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
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
        let call_prime1 = 2.;
        let call_prime2 = 2.;
        let put_prime1 = 1.;
        let put_prime2 = 1.;

        let _condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
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
        let call_prime1 = 2.;
        let call_prime2 = 2.;
        let put_prime1 = 1.;
        let put_prime2 = 1.;

        let _condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
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
        let call_prime1 = 2.;
        let call_prime2 = 2.;
        let put_prime1 = 1.;
        let put_prime2 = 1.;

        let condor = IronCondor::build(
            spot_price,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
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
        let main_prime_call = 2.;
        let main_prime_put = 2.;
        let down_prime = 2.;
        let up_prime = 2.;

        let iron_butterfly = IronButterfly::build(
            spot_price,
            main_strike,
            strike_down1,
            strike_up1,
            main_prime_call,
            main_prime_put,
            down_prime,
            up_prime,
        )
        .unwrap();

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
        let main_prime_call = 2.;
        let main_prime_put = 2.;
        let down_prime = 2.;
        let up_prime = 2.;

        let _iron_butterfly = IronButterfly::build(
            spot_price,
            main_strike,
            strike_down1,
            strike_up1,
            main_prime_call,
            main_prime_put,
            down_prime,
            up_prime,
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "StrikeConfigurationError")]
    fn test_build_iron_butterfly_with_wrong_configuration() {
        let spot_price = 100.;
        let main_strike = 80.;
        let strike_down1 = 70.;
        let strike_up1 = 80.;
        let main_prime_call = 2.;
        let main_prime_put = 2.;
        let down_prime = 2.;
        let up_prime = 2.;

        let _iron_butterfly = IronButterfly::build(
            spot_price,
            main_strike,
            strike_down1,
            strike_up1,
            main_prime_call,
            main_prime_put,
            down_prime,
            up_prime,
        )
        .unwrap();
    }

    #[test]
    fn test_iron_butterfly_payoff() {
        let spot_price = 100.;
        let main_strike = 80.;
        let strike_down1 = 70.;
        let strike_up1 = 90.;
        let main_prime_call = 2.;
        let main_prime_put = 2.;
        let down_prime = 2.;
        let up_prime = 2.;

        let iron_butterfly = IronButterfly::build(
            spot_price,
            main_strike,
            strike_down1,
            strike_up1,
            main_prime_call,
            main_prime_put,
            down_prime,
            up_prime,
        )
        .unwrap();

        assert_eq!(iron_butterfly.payoff(), -10.);
    }
}
