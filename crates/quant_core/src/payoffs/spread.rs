//! # The advanced options strategies for Bullish and Bearish market
//! All the structuration and computation for the payoff of advanced strategies used for Bearish or Bullish market(Bull and BearSpread)

use super::categorical_options::CallPutCategory;
use super::errors::PriceError;

/// The two categories of bull spread
/// The structure of a bull spread
///
/// Implements the method who compute the payoff of the strategies whether is a bull call spread or a bull put spread
///
/// ## Example
///
///```rust
/// use quant_core::payoffs::exotics::BullSpread;
/// use quant_core::payoffs::categorical_options::CallPutCategory;
/// use quant_core::payoffs::errors::PriceError;
///
/// fn main() -> Result<(), PriceError> {
/// let bullcallspread = BullSpread::build(60., 27., 68., CallPutCategory::Call)?;
///
/// assert_eq!(bullcallspread.payoff(), 33.0);
///
/// 0k(())
/// }
/// ```
pub struct BullSpread {
    pub spot_price: f64,
    pub strike_down: f64,
    pub strike_up: f64,
    pub prime_up: f64,
    pub prime_down: f64,
    pub category: CallPutCategory,
}

/// The structure of a bear spread
///
///  Implements method who compute the payoff of the strategies whether is a bull call spread or a bull put spread
/// ## Example
///
/// ```rust
/// use quant_core::payoffs::exotics::BearSpread;
/// use quant_core::payoffs::errors::PriceError;
/// use quant_core::payoffs::categorical_options::CallPutCategory;
///
/// fn main() -> Result<(), PriceError> {
/// let bearcallspread = BearSpread::build(50., 30., 100., CallPutCategory::Call)?;
///
/// assert_eq!(bearcallspread.payoff(), -20.);
///
/// Ok(())
/// }
/// ```
pub struct BearSpread {
    pub spot_price: f64,
    pub strike_down: f64,
    pub strike_up: f64,
    pub prime_up: f64,
    pub prime_down: f64,
    pub category: CallPutCategory,
}

impl BullSpread {
    /// Create a new `BullSpread`
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price` or `strike_price1` or `strike_price2` is negative or zero
    /// or
    /// if `strike_price1` is greater or equal to `strike_price2`
    pub fn build(
        spot_price: impl Into<f64>,
        strike_down: impl Into<f64>,
        strike_up: impl Into<f64>,
        prime_down: impl Into<f64>,
        prime_up: impl Into<f64>,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();
        let prime_down = prime_down.into();
        let prime_up = prime_up.into();

        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_down <= 0.0 || strike_up <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_down >= strike_up {
            return Err(PriceError::StrikeConfigurationError(strike_down, strike_up));
        }

        if prime_down <= 0.0 || prime_up <= 0.0 {
            return Err(PriceError::PrimePriceError);
        }

        Ok(Self {
            spot_price,
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category,
        })
    }

    /// Computes the option's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            CallPutCategory::Call => {
                let first_payoff = (self.spot_price - self.strike_down).max(0.);
                let second_payoff = (self.spot_price - self.strike_up).max(0.);
                first_payoff - second_payoff
            }
            CallPutCategory::Put => {
                let first_payoff = (self.strike_down - self.spot_price).max(0.);
                let second_payoff = (self.strike_up - self.spot_price).max(0.);
                first_payoff - second_payoff
            }
        }
    }

    pub fn pnl(&self) -> f64 {
        self.payoff() + self.prime_up - self.prime_down
    }
}

impl BearSpread {
    /// Create a `BearSpread`
    ///
    /// # Errors
    ///
    /// return error if `spot_price` or `strike_price1` or `strike_price2` is negative or zero
    /// or
    /// if `strike_price1` is greater or equal to `strike_price2`
    pub fn build(
        spot_price: impl Into<f64>,
        strike_down: impl Into<f64>,
        strike_up: impl Into<f64>,
        prime_down: impl Into<f64>,
        prime_up: impl Into<f64>,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();
        let prime_down = prime_down.into();
        let prime_up = prime_up.into();

        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_down <= 0.0 || strike_up <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_down >= strike_up {
            return Err(PriceError::StrikeConfigurationError(strike_down, strike_up));
        }

        if prime_down <= 0.0 || prime_up <= 0.0 {
            return Err(PriceError::PrimePriceError);
        }

        Ok(Self {
            spot_price,
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category,
        })
    }

    pub fn payoff(&self) -> f64 {
        match self.category {
            CallPutCategory::Call => {
                let first_payoff = (self.spot_price - self.strike_down).max(0.);
                let second_payoff = (self.spot_price - self.strike_up).max(0.);
                second_payoff - first_payoff
            }
            CallPutCategory::Put => {
                let first_payoff = (self.strike_down - self.spot_price).max(0.);
                let second_payoff = (self.strike_up - self.spot_price).max(0.);
                second_payoff - first_payoff
            }
        }
    }

    pub fn pnl(&self) -> f64 {
        self.payoff() - self.prime_up + self.prime_down
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_bull_spread() {
        let spot_price: f64 = 78.0;
        let strike_down: f64 = 10.0;
        let strike_up: f64 = 20.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let option = BullSpread::build(
            spot_price,
            strike_down,
            strike_up,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();

        assert_eq!(option.spot_price, spot_price);
        assert_eq!(option.strike_down, strike_down);
        assert_eq!(option.category, category);
    }

    #[test]
    #[should_panic]
    fn test_create_bull_spread_invalid_spot_price() {
        let spot_price: f64 = -78.0;
        let strike_price1: f64 = 10.0;
        let strike_price2: f64 = 20.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let _option = BullSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_create_bull_spread_invalid_strike_price() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 50.0;
        let strike_price2: f64 = 40.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let _option = BullSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category,
        )
        .unwrap();
    }

    #[test]
    fn test_payoff_bull_call() {
        let spot_price: f64 = 90.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let option = BullSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();

        let payoff = option.payoff();
        assert_eq!(payoff, 70.0);
    }

    #[test]
    fn test_payoff_bull_put_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Put;

        let option = BullSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();

        let payoff = option.payoff();
        assert_eq!(payoff, -20.0);
    }

    #[test]
    fn test_created_bear_spread() {
        let spot_price: f64 = 70.0;
        let strike_down: f64 = 20.0;
        let strike_up: f64 = 90.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let option = BearSpread::build(
            spot_price,
            strike_down,
            strike_up,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();

        assert_eq!(option.spot_price, spot_price);
        assert_eq!(option.strike_down, strike_down);
        assert_eq!(option.strike_up, strike_up);
        assert_eq!(option.category, category);
    }

    #[test]
    #[should_panic]
    fn test_created_bear_spread_invalid_spot_price() {
        let spot_price: f64 = -70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let _option = BearSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_created_bear_spread_invalid_strike_price() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 90.0;
        let strike_price2: f64 = 20.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let _option = BearSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category.clone(),
        )
        .unwrap();
    }

    #[test]
    fn test_payoff_bear_call_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Call;

        let option = BearSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_down,
            category,
        );
        let payoff = option.unwrap().payoff();

        assert_eq!(payoff, -50.0);
    }

    #[test]
    fn test_payoff_bear_put_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let prime_down: f64 = 10.0;
        let prime_up: f64 = 10.0;
        let category = CallPutCategory::Put;

        let option = BearSpread::build(
            spot_price,
            strike_price1,
            strike_price2,
            prime_up,
            prime_up,
            category,
        );
        let payoff = option.unwrap().payoff();

        assert_eq!(payoff, 20.0);
    }
}
