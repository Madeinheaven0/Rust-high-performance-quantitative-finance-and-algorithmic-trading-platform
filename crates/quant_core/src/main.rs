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
            prime,
            category,
        } => {
            let claim =
                BasicOption::build(spot, strike, prime, category).expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            println!(
                "The option:\n
                        - The spot: {},\n\
                        - The strike: {}, \n\
                        - The prime: {}, \n\
                        - The type: {:?}",
                claim.spot_price, claim.strike_price, prime, claim.category
            );

            println!("The payoff type: {}", payoff);
            println!("The pnl: {}", pnl);

            Ok(())
        }
        Strategy::BullSpread {
            spot,
            strike_up,
            strike_down,
            prime_up,
            prime_down,
            category,
        } => {
            let claim =
                BullSpread::build(spot, strike_down, strike_up, prime_up, prime_down, category)
                    .expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            println!(
                "The BullSpread:\n\
                    - The spot: {}, \n\
                    - The strike-up: {},\n
                    - The strike-down: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_up, claim.strike_up, claim.category
            );

            println!("The payoff: {:?}", payoff);
            println!("The pnl: {:?}", pnl);

            Ok(())
        }

        Strategy::BearSpread {
            spot,
            strike_up,
            strike_down,
            prime_up,
            prime_down,
            category,
        } => {
            let claim =
                BearSpread::build(spot, strike_down, strike_up, prime_up, prime_down, category)
                    .expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            println!(
                "The BearSpread:\n\
                    - The spot: {}, \n\
                    - The strike-up: {},\n
                    - The strike-down: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_up, claim.strike_up, claim.category
            );

            println!("The payoff: {}", payoff);
            println!("The pnl: {}", pnl);

            Ok(())
        }

        Strategy::Straddle {
            spot,
            strike,
            call_prime,
            put_prime,
            category,
        } => {
            let claim = Straddle::build(spot, strike, call_prime, put_prime, category)
                .expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            println!(
                "The Straddle:\n
                    - The spot: {}, \n
                    - The strike: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_price, claim.category
            );

            println!("The payoff: {}", payoff);
            println!("The pnl: {}", pnl);

            Ok(())
        }

        Strategy::Strangle {
            spot,
            strike_up,
            strike_down,
            call_prime,
            put_prime,
            category,
        } => {
            let claim = Strangle::build(
                spot,
                strike_down,
                strike_up,
                call_prime,
                put_prime,
                category,
            )
            .expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            println!(
                "The Strangle:\n\
                    - The spot: {}, \n\
                    - The strike-up: {},\n
                    - The strike-down: {},\n
                    - The type: {:?}",
                claim.spot_price, claim.strike_up, claim.strike_up, claim.category
            );

            println!("The payoff: {}", payoff);
            println!("The pnl: {}", pnl);

            Ok(())
        }

        Strategy::IronButterfly {
            spot,
            strike_up,
            strike_main,
            strike_down,
            main_call_prime,
            main_put_prime,
            down_prime,
            up_prime,
        } => {
            let claim = IronButterfly::build(
                spot,
                strike_main,
                strike_down,
                strike_up,
                main_call_prime,
                main_put_prime,
                down_prime,
                up_prime,
            )
            .expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            println!(
                "The IronButterfly:\n\
                    - The spot: {}, \n\
                    - The strike up: {},\n
                    - The main: {},\n
                    - The strike down: {}",
                claim.spot_price, claim.strike_up, claim.main_strike, claim.strike_down
            );

            println!("The payoff: {:?}", payoff);
            println!("The pnl: {}", pnl);

            Ok(())
        }

        Strategy::IronCondor {
            spot,
            strike_down1,
            strike_up1,
            strike_down2,
            strike_up2,
            call_prime1,
            call_prime2,
            put_prime1,
            put_prime2,
        } => {
            let claim = IronCondor::build(
                spot,
                strike_down1,
                strike_up1,
                strike_down2,
                strike_up2,
                call_prime1,
                call_prime2,
                put_prime1,
                put_prime2,
            )
            .expect("Something went wrong");
            let payoff = claim.payoff();
            let pnl = claim.pnl();

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
            println!("The pnl: {}", pnl);

            Ok(())
        }
    }
}
