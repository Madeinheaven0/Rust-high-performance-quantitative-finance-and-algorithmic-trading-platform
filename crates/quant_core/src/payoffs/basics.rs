//! # The Basics Options
//! All the structuration and computation for the payoff of simple strategies (call or put)

use super::categorical_options::CallPutCategory;
use super::errors::PriceError;

/// Represents the basic option's type used for strategies as known Call and Put European options.
///
/// Implements methods to compute the payoff and PnL of the option based on whether it is a Call or a Put.
///
/// ## Example
///
/// ```
/// use quant_core::payoffs::basics::BasicOption;
/// use quant_core::payoffs::categorical_options::CallPutCategory;
/// use quant_core::payoffs::errors::PriceError;
///
/// # fn main() -> Result<(), PriceError> {
/// let call = BasicOption::build(300.0, 250.0, 10.0, CallPutCategory::Call)?;
///
/// assert_eq!(call.payoff(), 50.0);
/// assert_eq!(call.pnl(), 40.0);
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BasicOption {
    pub spot_price: f64,
    pub strike_price: f64,
    pub prime: f64,
    pub category: CallPutCategory,
}

impl BasicOption {
    /// Creates a new `BasicOption`.
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price` or `strike_price` is negative or zero.
    pub fn build(
        spot_price: impl Into<f64>,
        strike_price: impl Into<f64>,
        prime: impl Into<f64>,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        let spot_price = spot_price.into();
        let strike_price = strike_price.into();
        let prime = prime.into();

        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_price <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        Ok(Self {
            spot_price,
            strike_price,
            prime,
            category,
        })
    }

    /// Computes the option's payoff at expiration.
    #[inline]
    pub fn payoff(&self) -> f64 {
        match self.category {
            CallPutCategory::Call => (self.spot_price - self.strike_price).max(0.0),
            CallPutCategory::Put => (self.strike_price - self.spot_price).max(0.0),
        }
    }

    /// Computes the option's Profit and Loss (PnL) at expiration.
    #[inline]
    pub fn pnl(&self) -> f64 {
        self.payoff() - self.prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_option() {
        let spot_price = 100.0;
        let strike_price = 10.0;
        let prime = 3.0;
        let category = CallPutCategory::Call;

        let basic_option = BasicOption::build(spot_price, strike_price, prime, category).unwrap();
        assert_eq!(basic_option.spot_price, spot_price);
        assert_eq!(basic_option.strike_price, strike_price);
    }

    #[test]
    #[should_panic]
    fn test_create_option_invalid_spot_price() {
        let spot_price = -100.0;
        let strike_price = 10.0;
        let prime = 3.0;
        let category = CallPutCategory::Call;

        let _basic_option = BasicOption::build(spot_price, strike_price, prime, category).unwrap();
    }

    #[test]
    fn test_payoff_and_pnl_call() {
        let call = BasicOption::build(100.0, 90.0, 3.0, CallPutCategory::Call).unwrap();
        assert_eq!(call.payoff(), 10.0);
        assert_eq!(call.pnl(), 7.0);
    }

    #[test]
    fn test_payoff_and_pnl_put() {
        let put = BasicOption::build(80.0, 100.0, 4.0, CallPutCategory::Put).unwrap();
        assert_eq!(put.payoff(), 20.0);
        assert_eq!(put.pnl(), 16.0);
    }
}