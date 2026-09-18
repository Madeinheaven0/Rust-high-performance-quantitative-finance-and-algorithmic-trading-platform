//! # The Iron series strategies
//! All the structuration and computation for the payoff of advanced strategies of option used when we are in range period

use super::errors::PriceError;
use super::spread::{BearSpread, BullSpread};
use super::categorical_options::CallPutCategory;

#[derive(Clone, PartialEq, Debug)]
pub enum IronCategory {
    Condor,
    Butterfly,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LongButterflySpreadCategory {
    Call,
    Put,
}

pub struct Iron {
    pub spot_price: f64,
    pub strike_price1: f64,
    pub strike_price2: f64,
    pub strike_price3: f64,
    pub strike_price4: f64,
    pub category: IronCategory,
}

impl Iron {
    fn build(
        spot_price: f64,
        strike_price1: f64,
        strike_price2: f64,
        strike_price3: f64,
        strike_price4: f64,
        category: IronCategory,
    ) -> Result<Self, PriceError> {
        if spot_price < 0. {
            return Err(PriceError::SpotPriceNegative(spot_price));
        }

        if strike_price1 < 0. || strike_price2 < 0. || strike_price3 < 0. || strike_price4 < 0. {
            return Err(PriceError::StrikePriceNegative);
        }

        if strike_price1 >= strike_price2 {
            return return Err(PriceError::StrikeConfigurationError(
                strike_price1,
                strike_price2,
            ));
        }

        if strike_price3 >= strike_price4 {
            return Err(PriceError::StrikeConfigurationError(
                strike_price3,
                strike_price4,
            ));
        }

        Ok(Self {
            spot_price,
            strike_price1,
            strike_price2,
            strike_price3,
            strike_price4,
            category,
        })
    }

    fn payoff(&self) -> f64 {
        match self.category {
            IronCategory::Butterfly => self.strike_price1,
            IronCategory::Condor => {
                let first_payoff = BullSpread::build(
                    self.spot_price,
                    self.strike_price1,
                    self.strike_price2,
                    CallPutCategory::Put
                ).unwrap()
                    .payoff();

                let second_payoff = BearSpread::build(
                    self.spot_price,
                    self.strike_price3,
                    self.strike_price4,
                    CallPutCategory::Call
                ).unwrap()
                    .payoff();

                first_payoff + second_payoff
            }
        }
    }
}
