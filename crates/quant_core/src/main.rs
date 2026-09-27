mod cli;
mod bench;
mod payoffs;

use anyhow::{Context, Result};
use clap::Parser;
use crate::cli::{Cli, Strategy};
use crate::payoffs::basics::BasicOption;
use crate::payoffs::range_bound::{IronButterfly, IronCondor};
use crate::payoffs::spread::{BearSpread, BullSpread};
use crate::payoffs::volatility::{Straddle, Strangle};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    tracing::info!("Executing options payoff evaluation");

    match cli.strategy {
        Strategy::SimpleOption {
            spot,
            strike,
            prime,
            category,
        } => {
            let claim = BasicOption::build(spot, strike, prime, category)
                .with_context(|| "Failed to build SimpleOption")?;
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            tracing::info!(
                spot = claim.spot_price,
                strike = claim.strike_price,
                prime = prime,
                category = ?claim.category,
                "Built SimpleOption"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
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
            // Note: build expects (strike_down, strike_up, prime_down, prime_up, category)
            let claim = BullSpread::build(strike_down, strike_up, prime_down, prime_up, category)
                .with_context(|| "Failed to build BullSpread")?;
            let payoff = claim
                .payoff(spot)
                .with_context(|| "Failed to compute BullSpread payoff")?;
            let pnl = claim
                .pnl(spot)
                .with_context(|| "Failed to compute BullSpread PnL")?;

            tracing::info!(
                spot = spot,
                strike_down = claim.strike_down,
                strike_up = claim.strike_up,
                category = ?claim.category,
                "Built BullSpread"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
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
            // Note: build expects (strike_down, strike_up, prime_down, prime_up, category)
            let claim = BearSpread::build(strike_down, strike_up, prime_down, prime_up, category)
                .with_context(|| "Failed to build BearSpread")?;
            let payoff = claim
                .payoff(spot)
                .with_context(|| "Failed to compute BearSpread payoff")?;
            let pnl = claim
                .pnl(spot)
                .with_context(|| "Failed to compute BearSpread PnL")?;

            tracing::info!(
                spot = spot,
                strike_down = claim.strike_down,
                strike_up = claim.strike_up,
                category = ?claim.category,
                "Built BearSpread"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
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
                .with_context(|| "Failed to build Straddle")?;
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            tracing::info!(
                spot = claim.spot_price,
                strike = claim.strike_price,
                category = ?claim.category,
                "Built Straddle"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
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
            let claim =
                Strangle::build(spot, strike_down, strike_up, call_prime, put_prime, category)
                    .with_context(|| "Failed to build Strangle")?;
            let payoff = claim.payoff();
            let pnl = claim.pnl();

            tracing::info!(
                spot = claim.spot_price,
                strike_down = claim.strike_down,
                strike_up = claim.strike_up,
                category = ?claim.category,
                "Built Strangle"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
            Ok(())
        }

        Strategy::IronCondor {
            spot,
            strike_k1,
            strike_k2,
            strike_k3,
            strike_k4,
            put_prime_1,
            put_prime_2,
            call_prime_1,
            call_prime_2,
        } => {
            // Matches IronCondor::new(K1, p1, K2, p2, K3, c3, K4, c4)
            let claim = IronCondor::new(
                strike_k1,
                put_prime_1,
                strike_k2,
                put_prime_2,
                strike_k3,
                call_prime_1,
                strike_k4,
                call_prime_2,
            )
                .with_context(|| "Failed to build IronCondor")?;

            let payoff = claim.payoff_at(spot);
            let pnl = claim.pnl_at(spot);

            tracing::info!(
                spot = spot,
                k1 = claim.put_buy.strike,
                k2 = claim.put_sell.strike,
                k3 = claim.call_sell.strike,
                k4 = claim.call_buy.strike,
                net_premium = claim.net_premium(),
                "Built IronCondor"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
            Ok(())
        }

        Strategy::IronButterfly {
            spot,
            strike_main,
            strike_down,
            strike_up,
            main_call_prime,
            main_put_prime,
            down_prime,
            up_prime,
        } => {
            // Matches IronButterfly::new(
            //   strike_put_buy, premium_put_buy,
            //   strike_atm, premium_put_sell, premium_call_sell,
            //   strike_call_buy, premium_call_buy
            // )
            let claim = IronButterfly::new(
                strike_down,   // K1 - Long Put
                down_prime,    // premium Long Put
                strike_main,   // K2 - ATM
                main_put_prime,// premium Short Put
                main_call_prime,// premium Short Call
                strike_up,     // K3 - Long Call
                up_prime,      // premium Long Call
            )
                .with_context(|| "Failed to build IronButterfly")?;

            let payoff = claim.payoff_at(spot);
            let pnl = claim.pnl_at(spot);

            tracing::info!(
                spot = spot,
                k_down = claim.put_buy.strike,
                k_atm = claim.atm_put_sell.strike,
                k_up = claim.call_buy.strike,
                net_premium = claim.net_premium(),
                "Built IronButterfly"
            );

            println!("Payoff: {}", payoff);
            println!("PnL: {}", pnl);
            Ok(())
        }
    }
}