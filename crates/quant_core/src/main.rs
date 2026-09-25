pub mod cli;
pub mod payoffs;

use crate::cli::{Cli, Strategy};
use crate::payoffs::basics::BasicOption;
use crate::payoffs::errors::PriceError;
use crate::payoffs::range_bound::{IronButterfly, IronCondor};
use crate::payoffs::spread::{BearSpread, BullSpread};
use crate::payoffs::volatility::{Straddle, Strangle};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    run(cli)?;

    Ok(())
}

fn run(cli: Cli) -> Result<(), PriceError> {
    match cli.strategy {
        Strategy::SimpleOption {
            spot,
            strike,
            category,
        } => {
            let claim = BasicOption::build(spot, strike, category).expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The option:\n
                        - The spot: {},\n\
                        - The strike: {}, \n
                        - The type: {:?}",
                claim.spot_price, claim.strike_price, claim.category
            );

            println!("The payoff type: {:?}", payoff);

            Ok(())
        }
        Strategy::BullSpread {
            spot,
            strike_up,
            strike_down,
            category,
        } => {
            let claim = BullSpread::build(spot, strike_down, strike_up, category)
                .expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The BullSpread:\n\
                    - The spot: {}, \n\
                    - The strike-up: {},\n
                    - The strike-down: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_up, claim.strike_up, claim.category
            );

            println!("The payoff: {:?}", payoff);

            Ok(())
        }

        Strategy::BearSpread {
            spot,
            strike_up,
            strike_down,
            category,
        } => {
            let claim = BearSpread::build(spot, strike_down, strike_up, category)
                .expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The BearSpread:\n\
                    - The spot: {}, \n\
                    - The strike-up: {},\n
                    - The strike-down: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_up, claim.strike_up, claim.category
            );

            println!("The payoff: {:?}", payoff);

            Ok(())
        }

        Strategy::Straddle {
            spot,
            strike,
            category,
        } => {
            let claim = Straddle::build(spot, strike, category).expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The Straddle:\n
                    - The spot: {}, \n
                    - The strike: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_price, claim.category
            );

            println!("The payoff: {:?}", payoff);

            Ok(())
        }

        Strategy::Strangle {
            spot,
            strike_up,
            strike_down,
            category,
        } => {
            let claim = Strangle::build(spot, strike_down, strike_up, category)
                .expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The Strangle:\n\
                    - The spot: {}, \n\
                    - The strike-up: {},\n
                    - The strike-down: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_up, claim.strike_up, claim.category
            );

            println!("The payoff: {:?}", payoff);

            Ok(())
        }

        Strategy::IronButterfly {
            spot,
            strike_up,
            strike_main,
            strike_down,
        } => {
            let claim = IronButterfly::build(spot, strike_main, strike_down, strike_up)
                .expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The IronButterfly:\n\
                    - The spot: {}, \n\
                    - The strike up: {},\n
                    - The main: {},\n
                    - The strike down: {}",
                claim.spot_price, claim.strike_up, claim.main_strike, claim.strike_down
            );

            println!("The payoff: {:?}", payoff);

            Ok(())
        }

        Strategy::IronCondor {
            spot,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
        } => {
            let claim = IronCondor::build(spot, strike_down1, strike_up1, strike_down2, strike_up2)
                .expect("Something went wrong");
            let payoff = claim.payoff();

            println!(
                "The IronButterfly:\n\
                    - The spot: {}, \n\
                    - The strike-down1: {},\n
                    - The strike-up1: {},\n
                    - The strike-down2: {}\n\
                    - The strike-up2: {}",
                claim.spot_price,
                claim.strike_down1,
                claim.strike_up1,
                claim.strike_down2,
                claim.strike_up2
            );

            println!("The payoff: {:?}", payoff);

            Ok(())
        }
    }
}
