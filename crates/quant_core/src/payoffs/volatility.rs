//! # Advanced strategies for volatility
//!
//! Structuring and payoff calculations for options volatility strategies (long and short straddles/strangles).

use super::categorical_options::LongShortCategory;
use super::errors::PriceError;

/// Structure representing a Straddle strategy.
///
/// Implements payoff and PnL computation for Long or Short straddles.
///
/// ## Examples
///
/// ```rust
/// use quant_core::payoffs::categorical_options::LongShortCategory;
/// use quant_core::payoffs::errors::PriceError;
/// use quant_core::payoffs::volatility::Straddle;
///
/// fn main() -> Result<(), PriceError> {
///     let long_straddle = Straddle::build(100.0, 100.0, 2.5, 2.5, LongShortCategory::Long)?;
///
///     assert_eq!(long_straddle.payoff(), 0.0);
///     assert_eq!(long_straddle.pnl(), -5.0);
///
///     Ok(())
/// }
/// ```
pub struct Straddle {
    pub category: LongShortCategory,
    pub spot_price: f64,
    pub strike_price: f64,
    pub call_prime: f64,
    pub put_prime: f64,
}

/// Structure representing a Strangle strategy.
///
/// Implements payoff and PnL computation for Long or Short strangles.
///
/// ## Examples
///
/// ```rust
/// use quant_core::payoffs::categorical_options::LongShortCategory;
/// use quant_core::payoffs::errors::PriceError;
/// use quant_core::payoffs::volatility::Strangle;
///
/// fn main() -> Result<(), PriceError> {
///     let long_strangle = Strangle::build(100.0, 80.0, 110.0, 1.5, 1.5, LongShortCategory::Long)?;
///
///     assert_eq!(long_strangle.payoff(), 0.0);
///     assert_eq!(long_strangle.pnl(), -3.0);
///
///     Ok(())
/// }
/// ```
pub struct Strangle {
    pub category: LongShortCategory,
    pub spot_price: f64,
    pub strike_down: f64,
    pub strike_up: f64,
    pub call_prime: f64,
    pub put_prime: f64,
}

impl Straddle {
    /// Create a new `Straddle`.
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price`, `strike_price`, `call_prime`, or `put_prime` is non-positive.
    pub fn build(
        spot_price: impl Into<f64>,
        strike_price: impl Into<f64>,
        call_prime: impl Into<f64>,
        put_prime: impl Into<f64>,
        category: LongShortCategory,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_price = strike_price.into();
        let call_prime = call_prime.into();
        let put_prime = put_prime.into();

        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_price <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        if call_prime <= 0.0 || put_prime <= 0.0 {
            return Err(PriceError::PrimePriceError);
        }

        Ok(Self {
            spot_price,
            strike_price,
            call_prime,
            put_prime,
            category,
        })
    }

    /// Computes the option strategy's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            LongShortCategory::Long => {
                let first_payoff = (self.spot_price - self.strike_price).max(0.0);
                let second_payoff = (self.strike_price - self.spot_price).max(0.0);

                first_payoff + second_payoff
            }
            LongShortCategory::Short => {
                let first_payoff = -(self.spot_price - self.strike_price).max(0.0);
                let second_payoff = -(self.strike_price - self.spot_price).max(0.0);

                first_payoff + second_payoff
            }
        }
    }

    /// Computes net profit and loss (PnL) accounting for initial premiums paid/received.
    pub fn pnl(&self) -> f64 {
        let payoff = self.payoff();
        match self.category {
            LongShortCategory::Long => payoff - self.call_prime - self.put_prime,
            LongShortCategory::Short => payoff + self.call_prime + self.put_prime,
        }
    }
}

impl Strangle {
    /// Create a new `Strangle`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `spot_price`, `strike_down`, or `strike_up` is non-positive
    /// - `strike_down >= strike_up`
    /// - `call_prime` or `put_prime` is non-positive
    pub fn build(
        spot_price: impl Into<f64>,
        strike_down: impl Into<f64>,
        strike_up: impl Into<f64>,
        call_prime: impl Into<f64>,
        put_prime: impl Into<f64>,
        category: LongShortCategory,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();
        let call_prime = call_prime.into();
        let put_prime = put_prime.into();

        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_down <= 0.0 || strike_up <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_down >= strike_up {
            return Err(PriceError::StrikeConfigurationError(
                strike_down,
                strike_up,
            ));
        }

        if call_prime <= 0.0 || put_prime <= 0.0 {
            return Err(PriceError::PrimePriceError);
        }

        Ok(Self {
            spot_price,
            strike_down,
            strike_up,
            call_prime,
            put_prime,
            category,
        })
    }

    /// Computes the option strategy's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            LongShortCategory::Long => {
                let first_payoff = (self.spot_price - self.strike_up).max(0.0);
                let second_payoff = (self.strike_down - self.spot_price).max(0.0);

                first_payoff + second_payoff
            }
            LongShortCategory::Short => {
                let first_payoff = -(self.spot_price - self.strike_up).max(0.0);
                let second_payoff = -(self.strike_down - self.spot_price).max(0.0);

                first_payoff + second_payoff
            }
        }
    }

    /// Computes net profit and loss (PnL) accounting for initial premiums paid/received.
    pub fn pnl(&self) -> f64 {
        let payoff = self.payoff();
        match self.category {
            LongShortCategory::Long => payoff - self.call_prime - self.put_prime,
            LongShortCategory::Short => payoff + self.call_prime + self.put_prime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    // --- Fixtures ---

    #[fixture]
    fn default_straddle_params() -> (f64, f64, f64, f64, LongShortCategory) {
        (100.0, 100.0, 1.0, 2.0, LongShortCategory::Long)
    }

    #[fixture]
    fn default_strangle_params() -> (f64, f64, f64, f64, f64, LongShortCategory) {
        (100.0, 80.0, 100.0, 1.0, 1.0, LongShortCategory::Long)
    }

    // --- Straddle Tests ---

    #[rstest]
    #[case(LongShortCategory::Long)]
    #[case(LongShortCategory::Short)]
    fn test_build_straddle_category(#[case] category: LongShortCategory) {
        let straddle = Straddle::build(100.0, 100.0, 1.0, 2.0, category.clone()).unwrap();
        assert_eq!(straddle.category, category);
        assert_eq!(straddle.spot_price, 100.0);
        assert_eq!(straddle.strike_price, 100.0);
    }

    #[rstest]
    #[case(-100.0, 100.0, 1.0, 2.0, "Negative spot")]
    #[case(100.0, -100.0, 1.0, 2.0, "Negative strike")]
    #[case(100.0, 100.0, -1.0, 2.0, "Negative call prime")]
    #[case(100.0, 100.0, 1.0, -2.0, "Negative put prime")]
    #[should_panic]
    fn test_straddle_invalid_inputs(
        #[case] spot: f64,
        #[case] strike: f64,
        #[case] call_p: f64,
        #[case] put_p: f64,
        #[case] _reason: &str,
    ) {
        Straddle::build(spot, strike, call_p, put_p, LongShortCategory::Long).unwrap();
    }

    #[rstest]
    #[case(100.0, 90.0, LongShortCategory::Long, 10.0)]
    #[case(100.0, 90.0, LongShortCategory::Short, -10.0)]
    #[case(80.0, 100.0, LongShortCategory::Long, 20.0)]
    #[case(80.0, 100.0, LongShortCategory::Short, -20.0)]
    fn test_straddle_payoff(
        #[case] spot: f64,
        #[case] strike: f64,
        #[case] category: LongShortCategory,
        #[case] expected_payoff: f64,
    ) {
        let straddle = Straddle::build(spot, strike, 1.0, 2.0, category).unwrap();
        assert_eq!(straddle.payoff(), expected_payoff);
    }

    // --- Strangle Tests ---

    #[rstest]
    #[case(LongShortCategory::Long)]
    #[case(LongShortCategory::Short)]
    fn test_build_strangle_category(#[case] category: LongShortCategory) {
        let strangle = Strangle::build(100.0, 80.0, 100.0, 1.0, 1.0, category.clone()).unwrap();
        assert_eq!(strangle.category, category);
        assert_eq!(strangle.spot_price, 100.0);
        assert_eq!(strangle.strike_down, 80.0);
        assert_eq!(strangle.strike_up, 100.0);
    }

    #[rstest]
    #[case(-100.0, 80.0, 100.0, 1.0, 1.0, "Negative spot")]
    #[case(100.0, 180.0, 90.0, 1.0, 1.0, "Inverted strikes (down > up)")]
    #[case(100.0, 80.0, 100.0, -1.0, 1.0, "Negative call prime")]
    #[case(100.0, 80.0, 100.0, 1.0, -1.0, "Negative put prime")]
    #[should_panic]
    fn test_build_strangle_invalid_inputs(
        #[case] spot: f64,
        #[case] strike_down: f64,
        #[case] strike_up: f64,
        #[case] call_p: f64,
        #[case] put_p: f64,
        #[case] _reason: &str,
    ) {
        Strangle::build(
            spot,
            strike_down,
            strike_up,
            call_p,
            put_p,
            LongShortCategory::Long,
        )
            .unwrap();
    }

    #[rstest]
    #[case(100.0, 75.0, 90.0, LongShortCategory::Long, 10.0)]
    #[case(100.0, 80.0, 90.0, LongShortCategory::Short, -10.0)]
    #[case(70.0, 75.0, 90.0, LongShortCategory::Long, 5.0)]
    #[case(82.0, 75.0, 90.0, LongShortCategory::Long, 0.0)] // Inside strangle range
    fn test_strangle_payoff(
        #[case] spot: f64,
        #[case] strike_down: f64,
        #[case] strike_up: f64,
        #[case] category: LongShortCategory,
        #[case] expected_payoff: f64,
    ) {
        let strangle =
            Strangle::build(spot, strike_down, strike_up, 1.0, 1.0, category).unwrap();
        assert_eq!(strangle.payoff(), expected_payoff);
    }
}