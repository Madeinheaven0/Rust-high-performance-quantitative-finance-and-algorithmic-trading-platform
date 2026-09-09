//! # The Basics Options
//! All the structuration and computation for the payoff of simple strategies (call or put)

/// The type of the European option ( Call or Put)
pub enum EuropeanOption {
    Call,
    Put
}

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
    pub category: EuropeanOption,
}

impl BasicOption {
    /// Creates a new `BasicOption`.
    ///
    /// # Errors
    ///
    /// Returns an error if `spot_price` or `strike_price` is negative or zero.
    pub fn new<'a>(spot_price: f64, strike_price: f64, category: EuropeanOption) -> Result<Self, &'a str> {
        if spot_price <= 0.0 || strike_price <= 0.0 {
            return Err("The spot_price or strike_price must be positive.");
        }

        Ok(
            Self {
                spot_price,
                strike_price,
                category
            }
        )
    }

    /// Computes the option's payoff at expiration.
    pub fn payoff(&self) -> f64 {
        match self.category {
            EuropeanOption::Call => (self.spot_price - self.strike_price).max(0.),
            EuropeanOption::Put => (self.strike_price - self.spot_price).max(0.),
        }
    }
}
