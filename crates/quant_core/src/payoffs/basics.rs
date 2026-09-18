//! # The Basics Options
//! All the structuration and computation for the payoff of simple strategies (call or put)

use super::categorical_options::CallPutCategory;
use super::errors::PriceError;

/// Represents the basic option's type used for strategies as known Call and Put European
/// options.
///
/// Implements method who compute the payoff of the option based on whether is a Call or a PUt
///
/// ## Example
///
/// ```
/// use quant_core::payoffs::basics::{BasicOption, EuropeanOption};
///
/// # fn main() -> Result<(), &'static str> {
/// let call = BasicOption::new(300.0, 250.0, EuropeanOption::Call)?;
///
/// assert_eq!(call.payoff(), 50.0);
///
/// # Ok(())
/// # }
/// ```
pub struct BasicOption {
    pub spot_price: f64,
    pub strike_price: f64,
    pub category: CallPutCategory,
}

impl BasicOption {
    /// Creates a new `BasicOption`.
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price` or `strike_price` is negative or zero.
    pub fn build(
        spot_price: f64,
        strike_price: f64,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if spot_price <= 0.0 || strike_price <= 0.0 {
            return Err(PriceError::StrikePriceNegative);
        }

        Ok(Self {
            spot_price,
            strike_price,
            category,
        })
    }

    /// Computes the option's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            CallPutCategory::Call => (self.spot_price - self.strike_price).max(0.),
            CallPutCategory::Put => (self.strike_price - self.spot_price).max(0.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_option() {
        let spot_price = 100.;
        let strike_price = 10.;
        let category = CallPutCategory::Call;

        let basic_option = BasicOption::build(spot_price, strike_price, category).unwrap();
        assert_eq!(basic_option.spot_price, spot_price);
        assert_eq!(basic_option.strike_price, strike_price);
    }

    #[test]
    #[should_panic]
    fn test_create_option_invalid_spot_price() {
        let spot_price = -100.;
        let strike_price = 10.;
        let category = CallPutCategory::Call;

        let _basic_option = BasicOption::build(spot_price, strike_price, category).unwrap();
    }

    #[test]
    fn test_payoff() {
        let spot_price = 100.;
        let strike_price = 10.;
        let category = CallPutCategory::Call;

        let basic_option = BasicOption::build(spot_price, strike_price, category).unwrap();

        let payoff = basic_option.payoff();

        assert_eq!(payoff, 90.);
    }
}
