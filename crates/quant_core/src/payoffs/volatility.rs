//! # Advanced strategies for volatility
//!All the structuration and computation for the payoff of advanced strategies used to take profit of the volatility (long and short straddle)

use super::categorical_options::LongShortCategory;
use super::errors::PriceError;

///The structure of a straddle
///
///Implements the method who compute the payoff of the strategies whether is a Long or Short straddle
///
/// ## Examples
///
/// ```rust
/// use quant_core::payoffs::volatiliy::Straddle;
/// use quant_core::payoffs::categorigal_options::LongShortCategory;
/// use quant_core::payoffs::errors::PriceError;
///
/// fn main() -> Result<(), PriceError> {
/// let long_straddle = Straddle::build(50.0, 30.0, LongShortCategory::Long)?;
///
/// assert_eq!(long_straddle.payoff(), 20);
///
/// Ok(())
/// }
/// ```
pub struct Straddle {
    pub category: LongShortCategory,
    pub spot_price: f64,
    pub strike_price: f64,
    pub call_prime: f64,
    pub put_prime: f64,
}

///The structure of Strangle
///
///Implements the method who compute the payoff of the strategies whether is a Long or Short strangle
///
/// ## Examples
///
/// ```rust
/// use quant_core::payoffs::volatiliy::Strangle;
/// use quant_core::payoffs::categorigal_options::LongShortCategory;
/// use quant_core::payoffs::errors::PriceError;
///
/// fn main() -> Result<(), PriceError> {
/// let long_strangle = Strangle::build(100.0, 50.0, 90.0, LongShortCategory::Long)?;
///
/// assert_eq!(long_strangle.payoff(), 10);
///
/// Ok(())
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
    /// Create a new `Straddle`
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price` or `strike_price`  is negative or zero
    pub fn build(
        spot_price: impl Into<f64>,
        strike_price: impl Into<f64>,
        call_prime: f64,
        put_prime: f64,
        category: LongShortCategory,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_price = strike_price.into();
        let call_prime = call_prime.into();
        let put_prime = put_prime.into();

        if spot_price <= 0.0 || strike_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
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

    /// Computes the option's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            LongShortCategory::Long => {
                let first_payoff = (self.spot_price - self.strike_price).max(0.);
                let second_payoff = (self.strike_price - self.spot_price).max(0.);

                first_payoff + second_payoff
            }
            LongShortCategory::Short => {
                let first_payoff = -(self.spot_price - self.strike_price).max(0.);
                let second_payoff = -(self.strike_price - self.spot_price).max(0.);

                first_payoff + second_payoff
            }
        }
    }

    pub fn pnl(&self) -> f64 {
        let payoff = self.payoff();
        match self.category {
            LongShortCategory::Long => payoff - self.call_prime - self.put_prime,
            LongShortCategory::Short => payoff + self.call_prime + self.put_prime,
        }
    }
}

impl Strangle {
    /// Create a new `Strangle`
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price`, `strike_price1` or `strike_price2`  is negative or zero
    /// or `strike_price1` greater or equal to `strike_price2`
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
            return Err(PriceError::StrikeConfigurationError(strike_down, strike_up));
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

    /// Computes the option's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            LongShortCategory::Long => {
                let first_payoff = (self.spot_price - self.strike_up).max(0.);
                let second_payoff = (self.strike_down - self.spot_price).max(0.);

                first_payoff + second_payoff
            }
            LongShortCategory::Short => {
                let first_payoff = -(self.spot_price - self.strike_up).max(0.);
                let second_payoff = -(self.strike_down - self.spot_price).max(0.);

                first_payoff + second_payoff
            }
        }
    }

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

    #[test]
    fn test_build_long_straddle() {
        let spot_price = 100.0;
        let strike_price = 100.0;
        let prime1 = 1.0;
        let prime2 = 2.0;
        let category = LongShortCategory::Long;

        let long_straddle =
            Straddle::build(spot_price, strike_price, prime1, prime2, category).unwrap();

        assert_eq!(long_straddle.category, LongShortCategory::Long);
        assert_eq!(long_straddle.spot_price, spot_price);
        assert_eq!(long_straddle.strike_price, strike_price);
    }

    #[test]
    fn test_build_short_straddle() {
        let spot_price = 100.0;
        let strike_price = 100.0;
        let prime1 = 1.0;
        let prime2 = 2.0;
        let category = LongShortCategory::Short;

        let short_straddle =
            Straddle::build(spot_price, strike_price, prime1, prime2, category).unwrap();

        assert_eq!(short_straddle.category, LongShortCategory::Short);
        assert_eq!(short_straddle.spot_price, 100.0);
        assert_eq!(short_straddle.strike_price, 100.0);
    }

    #[test]
    #[should_panic]
    fn test_straddle_invalid_price() {
        let spot_price = 100.0;
        let strike_price = -100.0;
        let prime1 = 1.0;
        let prime2 = 2.0;
        let category = LongShortCategory::Long;

        let _long_straddle =
            Straddle::build(spot_price, strike_price, prime1, prime2, category).unwrap();
    }

    #[test]
    fn test_long_straddle_payoff() {
        let spot_price = 100.0;
        let strike_price = 90.0;
        let prime1 = 1.0;
        let prime2 = 2.0;
        let category = LongShortCategory::Long;

        let straddle = Straddle::build(spot_price, strike_price, prime1, prime2, category).unwrap();

        let payoff = straddle.payoff();

        assert_eq!(payoff, 10.0);
    }

    #[test]
    fn test_short_straddle_payoff() {
        let spot_price = 100.0;
        let strike_price = 90.0;
        let prime1 = 1.0;
        let prime2 = 2.0;
        let category = LongShortCategory::Short;

        let short_straddle =
            Straddle::build(spot_price, strike_price, prime1, prime2, category).unwrap();

        let payoff = short_straddle.payoff();
        assert_eq!(payoff, -10.0);
    }

    #[test]
    fn test_build_long_strangle() {
        let spot_price = 100.0;
        let strike_down = 80.0;
        let call_prime = 1.0;
        let put_prime = 1.0;
        let strike_up = 100.0;
        let category = LongShortCategory::Long;

        let long_strangle = Strangle::build(
            spot_price,
            strike_down,
            strike_up,
            call_prime,
            put_prime,
            category,
        )
        .unwrap();

        assert_eq!(long_strangle.category, LongShortCategory::Long);
        assert_eq!(long_strangle.spot_price, 100.0);
        assert_eq!(long_strangle.strike_down, 80.0);
        assert_eq!(long_strangle.strike_up, 100.0);
    }

    #[test]
    fn test_build_short_strangle() {
        let spot_price = 100.0;
        let strike_down = 80.0;
        let strike_up = 100.0;
        let call_prime = 1.0;
        let put_prime = 1.0;
        let category = LongShortCategory::Short;

        let long_strangle = Strangle::build(
            spot_price,
            strike_down,
            strike_up,
            call_prime,
            put_prime,
            category,
        )
        .unwrap();

        assert_eq!(long_strangle.category, LongShortCategory::Short);
        assert_eq!(long_strangle.spot_price, 100.0);
        assert_eq!(long_strangle.strike_down, 80.0);
        assert_eq!(long_strangle.strike_up, 100.0);
    }

    #[test]
    #[should_panic]
    fn test_build_straddle_invalid_price() {
        let spot_price = -100.0;
        let strike_price1 = 80.0;
        let strike_price2 = 100.0;
        let call_prime = 1.0;
        let put_prime = 1.0;
        let category = LongShortCategory::Long;

        let _long_strangle = Strangle::build(
            spot_price,
            strike_price1,
            strike_price2,
            call_prime,
            put_prime,
            category,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_straddle_invalid_strike() {
        let spot_price = 100.0;
        let strike_down = 180.0;
        let strike_up = 90.0;
        let call_prime = 1.0;
        let put_prime = 1.0;
        let category = LongShortCategory::Long;

        let _long_strangle = Strangle::build(
            spot_price,
            strike_down,
            strike_up,
            call_prime,
            put_prime,
            category,
        )
        .unwrap();
    }

    #[test]
    fn test_long_strangle_payoff() {
        let spot_price = 100.0;
        let strike_down = 75.0;
        let strike_up = 90.0;
        let call_prime = 1.0;
        let put_prime = 1.0;
        let category = LongShortCategory::Long;

        let long_strangle = Strangle::build(
            spot_price,
            strike_down,
            strike_up,
            call_prime,
            put_prime,
            category,
        )
        .unwrap();

        assert_eq!(long_strangle.payoff(), 10.0);
    }

    #[test]
    fn test_short_strangle_payoff() {
        let spot_price = 100.0;
        let strike_down = 80.0;
        let strike_up = 90.0;
        let call_prime = 1.0;
        let put_prime = 1.0;
        let category = LongShortCategory::Short;

        let short_strangle = Strangle::build(
            spot_price,
            strike_down,
            strike_up,
            call_prime,
            put_prime,
            category,
        )
        .unwrap();

        assert_eq!(short_strangle.payoff(), -10.0);
    }
}
