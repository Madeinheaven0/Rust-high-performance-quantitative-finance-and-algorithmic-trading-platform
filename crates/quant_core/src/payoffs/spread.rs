//! # The advanced options strategies for Bullish and Bearish market
//! All the structuration and computation for the payoff of advanced strategies used for Bearish or Bullish market(Bull and BearSpread)

use super::categorical_options::CallPutCategory;
use super::errors::PriceError;
use crate::payoffs::range_bound::LongButterflySpreadCategory::Call;

/// The two categories of bull spread

/// The structure of a bull spread
///
/// Implements the method who compute the payoff of the strategies whether is a bull call spread or a bull put spread
///
/// ## Example
///
///```rust
/// use quant_core::payoffs::exotics::{SpreadCategory  , BullSpread};
///
/// fn main() -> Resul<(), &'static str> {
/// let bullcallspread = BullSpread::build(60., 27., 68., SpreadCategory::Call)?;
///
/// assert_eq!(bullcallspread.payoff(), 33.0);
///
/// 0k(())
/// }
/// ```
pub struct BullSpread {
    pub spot_price: f64,
    pub strike_price1: f64,
    pub strike_price2: f64,
    pub category: CallPutCategory,
}

/// The structure of a bear spread
///
///  Implements method who compute the payoff of the strategies whether is a bull call spread or a bull put spread
/// ## Example
///
/// ```rust
/// use quant_core::payoffs::exotics::{SpreadCategory  , BearSpread};
///
/// fn main() -> Result<(), &'static str> {
/// let bearcallspread = BearSpread::build(50., 30., 100., Category::Call)?;
///
/// assert_eq!(bearcallspread.payoff(), -20);
///
/// Ok(())
/// }
/// ```
pub struct BearSpread {
    pub spot_price: f64,
    pub strike_price1: f64,
    pub strike_price2: f64,
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
        spot_price: f64,
        strike_price1: f64,
        strike_price2: f64,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_price1 <= 0.0 || strike_price2 <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_price1 >= strike_price2 {
            return Err(PriceError::StrikeConfigurationError(
                strike_price1,
                strike_price2,
            ));
        }

        Ok(Self {
            spot_price,
            strike_price1,
            strike_price2,
            category,
        })
    }

    /// Computes the option's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            CallPutCategory::Call => {
                let first_payoff = (self.spot_price - self.strike_price1).max(0.);
                let second_payoff = (self.spot_price - self.strike_price2).max(0.);
                first_payoff - second_payoff
            }
            CallPutCategory::Put => {
                let first_payoff = (self.strike_price1 - self.spot_price).max(0.);
                let second_payoff = (self.strike_price2 - self.spot_price).max(0.);
                first_payoff - second_payoff
            }
        }
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
        spot_price: f64,
        strike_price1: f64,
        strike_price2: f64,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_price1 <= 0.0 || strike_price2 <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_price1 >= strike_price2 {
            return Err(PriceError::StrikeConfigurationError(
                strike_price1,
                strike_price2,
            ));
        }

        Ok(Self {
            spot_price,
            strike_price1,
            strike_price2,
            category,
        })
    }

    pub fn payoff(&self) -> f64 {
        match self.category {
            CallPutCategory::Call => {
                let first_payoff = (self.spot_price - self.strike_price1).max(0.);
                let second_payoff = (self.spot_price - self.strike_price2).max(0.);
                second_payoff - first_payoff
            }
            CallPutCategory::Put => {
                let first_payoff = (self.strike_price1 - self.spot_price).max(0.);
                let second_payoff = (self.strike_price2 - self.spot_price).max(0.);
                second_payoff - first_payoff
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_bull_spread() {
        let spot_price: f64 = 78.0;
        let strike_price1: f64 = 10.0;
        let strike_price2: f64 = 20.0;
        let category = CallPutCategory::Call;

        let option =
            BullSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();

        assert_eq!(option.spot_price, spot_price);
        assert_eq!(option.strike_price1, strike_price1);
        assert_eq!(option.category, category);
    }

    #[test]
    #[should_panic]
    fn test_create_bull_spread_invalid_spot_price() {
        let spot_price: f64 = -78.0;
        let strike_price1: f64 = 10.0;
        let strike_price2: f64 = 20.0;
        let category = CallPutCategory::Call;

        let _option =
            BullSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_create_bull_spread_invalid_strike_price() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 50.0;
        let strike_price2: f64 = 40.0;
        let category = CallPutCategory::Call;

        let _option =
            BullSpread::build(spot_price, strike_price1, strike_price2, category).unwrap();
    }

    #[test]
    fn test_payoff_bull_call() {
        let spot_price: f64 = 90.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let category = CallPutCategory::Call;

        let option =
            BullSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();

        let payoff = option.payoff();
        assert_eq!(payoff, 70.0);
    }

    #[test]
    fn test_payoff_bull_put_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let category = CallPutCategory::Put;

        let option =
            BullSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();

        let payoff = option.payoff();
        assert_eq!(payoff, -20.0);
    }

    #[test]
    fn test_created_bear_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let category = CallPutCategory::Call;

        let option =
            BearSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();

        assert_eq!(option.spot_price, spot_price);
        assert_eq!(option.strike_price1, strike_price1);
        assert_eq!(option.strike_price2, strike_price2);
        assert_eq!(option.category, category);
    }

    #[test]
    #[should_panic]
    fn test_created_bear_spread_invalid_spot_price() {
        let spot_price: f64 = -70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let category = CallPutCategory::Call;

        let _option =
            BearSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_created_bear_spread_invalid_strike_price() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 90.0;
        let strike_price2: f64 = 20.0;
        let category = CallPutCategory::Call;

        let _option =
            BearSpread::build(spot_price, strike_price1, strike_price2, category.clone()).unwrap();
    }

    #[test]
    fn test_payoff_bear_call_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let category = CallPutCategory::Call;

        let option = BearSpread::build(spot_price, strike_price1, strike_price2, category);
        let payoff = option.unwrap().payoff();

        assert_eq!(payoff, -50.0);
    }

    #[test]
    fn test_payoff_bear_put_spread() {
        let spot_price: f64 = 70.0;
        let strike_price1: f64 = 20.0;
        let strike_price2: f64 = 90.0;
        let category = CallPutCategory::Put;

        let option = BearSpread::build(spot_price, strike_price1, strike_price2, category);
        let payoff = option.unwrap().payoff();

        assert_eq!(payoff, 20.0);
    }
}
