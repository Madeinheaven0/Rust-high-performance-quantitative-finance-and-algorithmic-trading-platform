//! # The advanced options strategies for Bullish and Bearish market
//! All the structuration and computation for the payoff of advanced strategies used for Bearish or Bullish market (Bull and Bear Spread)

use super::categorical_options::CallPutCategory;
use super::errors::PriceError;

/// The structure representing a Bull Spread strategy (Bull Call or Bull Put)
pub struct BullSpread {
    pub strike_down: f64,
    pub strike_up: f64,
    pub prime_down: f64,
    pub prime_up: f64,
    pub category: CallPutCategory,
}

/// The structure representing a Bear Spread strategy (Bear Call or Bear Put)
pub struct BearSpread {
    pub strike_down: f64,
    pub strike_up: f64,
    pub prime_down: f64,
    pub prime_up: f64,
    pub category: CallPutCategory,
}

impl BullSpread {
    /// Create a new `BullSpread`
    ///
    /// # Errors
    /// Returns an error if strikes or primes are negative or if `strike_down >= strike_up`.
    pub fn build(
        strike_down: impl Into<f64>,
        strike_up: impl Into<f64>,
        prime_down: impl Into<f64>,
        prime_up: impl Into<f64>,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();
        let prime_down = prime_down.into();
        let prime_up = prime_up.into();

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
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category,
        })
    }

    /// Computes the option strategy payoff at expiration given a spot price.
    pub fn payoff(&self, spot_price: f64) -> Result<f64, PriceError> {
        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        let payoff = match self.category {
            CallPutCategory::Call => {
                let first_payoff = (spot_price - self.strike_down).max(0.0);
                let second_payoff = (spot_price - self.strike_up).max(0.0);
                first_payoff - second_payoff
            }
            CallPutCategory::Put => {
                let first_payoff = (self.strike_down - spot_price).max(0.0);
                let second_payoff = (self.strike_up - spot_price).max(0.0);
                first_payoff - second_payoff
            }
        };

        Ok(payoff)
    }

    /// Computes the total profit and loss (PnL) including initial net premium.
    pub fn pnl(&self, spot_price: f64) -> Result<f64, PriceError> {
        let payoff = self.payoff(spot_price)?;
        Ok(payoff + self.prime_up - self.prime_down)
    }
}

impl BearSpread {
    /// Create a new `BearSpread`
    ///
    /// # Errors
    /// Returns an error if strikes or primes are negative or if `strike_down >= strike_up`.
    pub fn build(
        strike_down: impl Into<f64>,
        strike_up: impl Into<f64>,
        prime_down: impl Into<f64>,
        prime_up: impl Into<f64>,
        category: CallPutCategory,
    ) -> Result<Self, PriceError> {
        let strike_down = strike_down.into();
        let strike_up = strike_up.into();
        let prime_down = prime_down.into();
        let prime_up = prime_up.into();

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
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category,
        })
    }

    /// Computes the option strategy payoff at expiration given a spot price.
    pub fn payoff(&self, spot_price: f64) -> Result<f64, PriceError> {
        if spot_price <= 0.0 {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        let payoff = match self.category {
            CallPutCategory::Call => {
                let first_payoff = (spot_price - self.strike_down).max(0.0);
                let second_payoff = (spot_price - self.strike_up).max(0.0);
                second_payoff - first_payoff
            }
            CallPutCategory::Put => {
                let first_payoff = (self.strike_down - spot_price).max(0.0);
                let second_payoff = (self.strike_up - spot_price).max(0.0);
                second_payoff - first_payoff
            }
        };

        Ok(payoff)
    }

    /// Computes the total profit and loss (PnL) including initial net premium.
    pub fn pnl(&self, spot_price: f64) -> Result<f64, PriceError> {
        let payoff = self.payoff(spot_price)?;
        Ok(payoff - self.prime_up + self.prime_down)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bull_spread() {
        let strike_down = 10.0;
        let strike_up = 20.0;
        let prime_down = 10.0;
        let prime_up = 10.0;
        let category = CallPutCategory::Call;

        let option = BullSpread::build(
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category.clone(),
        )
            .unwrap();

        assert_eq!(option.strike_down, strike_down);
        assert_eq!(option.category, category);
    }

    #[test]
    fn test_payoff_bull_call() {
        let spot_price = 90.0;
        let strike_down = 20.0;
        let strike_up = 90.0;
        let prime_down = 10.0;
        let prime_up = 10.0;
        let category = CallPutCategory::Call;

        let option = BullSpread::build(
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category,
        )
            .unwrap();

        assert_eq!(option.payoff(spot_price).unwrap(), 70.0);
    }

    #[test]
    fn test_payoff_bear_call_spread() {
        let spot_price = 70.0;
        let strike_down = 20.0;
        let strike_up = 90.0;
        let prime_down = 10.0;
        let prime_up = 10.0;
        let category = CallPutCategory::Call;

        let option = BearSpread::build(
            strike_down,
            strike_up,
            prime_down,
            prime_up,
            category,
        )
            .unwrap();

        assert_eq!(option.payoff(spot_price).unwrap(), -50.0);
    }
}