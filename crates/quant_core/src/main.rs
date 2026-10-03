mod cli;
mod payoffs;

use clap::Parser;
use tracing_subscriber;
use tracing;
use anyhow::{Context, Result};
use crate::cli::{Cli, Strategy};
use crate::payoffs::domain_types::{Position, Premium, Spot, Strike};
use crate::payoffs::leg::OptionLeg;
use crate::payoffs::spread;
use crate::payoffs::volatility_strategies;
use crate::payoffs::range_bound;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    
    running(cli).with_context(|| "Failed to run")?;
    
    Ok(())
}

fn running(cli: Cli)->Result<()> {
    tracing::info!("Running the cli");

    match cli.strategy {
        Strategy::Basic {
            kind,
            position,
            strike,
            premium,
            spot
        } => {
             tracing::info!("Running the basic");
            let basic = OptionLeg::build(
                kind,
                position,
                Strike(strike),
                Premium(premium),
            ).with_context(|| "failed to build basic strategy".to_string())?;

            let payoff = basic.payoff(Spot(spot)).with_context(|| "failed to payoff spot".to_string())?;
            let pnl = basic.pnl(Spot(spot)).with_context(|| "failed to pnl spot".to_string())?;

            tracing::info!("The payoff is: {}", payoff);
            tracing::info!("The pnl is: {}", pnl);

            Ok(())
        },
        Strategy::BullCallSpread {
            strike_up,
            premium_up,
            strike_down,
            premium_down,
            spot, 
        } => {
            tracing::info!("Running the Bull Call Spread");
            
            
            let bull_call_spread = spread::BullCallSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                Position::Long,
            ).with_context(|| "failed to build bull call spread".to_string())?;
            
            let payoff = bull_call_spread.payoff(Spot(spot)).with_context(|| "failed to payoff spot of the bull call spread".to_string())?;
            let pnl = bull_call_spread.pnl(Spot(spot)).with_context(|| "failed to pnl spot of the bull call spread".to_string())?;
            
            tracing::info!("The payoff of the Bull Call Spread is: {}", payoff);
            tracing::info!("The pnl of the Bull Call Spread is: {}", pnl);
            
            Ok(())
        },
        
        Strategy::BullPutSpread {
            strike_up,
            premium_up,
            strike_down,
            premium_down,
            spot
        } => {
            tracing::info!("Running the Bull Put Spread");
            
            let bull_put_spread = spread::BullPutSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                Position::Long,
            ).with_context(|| "failed to build bull put spread".to_string())?;
            
            let payoff = bull_put_spread.payoff(Spot(spot)).with_context(|| "failed to payoff spot of the bull put spread".to_string())?;
            let pnl = bull_put_spread.pnl(Spot(spot)).with_context(|| "failed to pnl spot of the bull put spread".to_string())?;
            
            tracing::info!("The payoff of the Bull Put Spread is: {}", payoff);
            tracing::info!("The pnl of the Bull Put Spread is: {}", pnl);
            
            Ok(())
        },
        
        Strategy::BearCallSpread {
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            spot,
        } => {
            tracing::info!("Running the Bear Call Spread");
            
            let bear_call_spread = spread::BearCallSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                Position::Long
            ).with_context(|| "failed to build bear call spread".to_string())?;
            
            let payoff = bear_call_spread.payoff(Spot(spot)).with_context(|| "failed to payoff spot of the bear call spread".to_string())?;
            let pnl = bear_call_spread.pnl(Spot(spot)).with_context(|| "failed to pnl spot of the bear call spread".to_string())?;
            
            tracing::info!("The payoff of the Bear Call Spread is: {}", payoff);
            tracing::info!("The pnl of the Bear Call Spread is: {}", pnl);
            
            Ok(())
        },
        
        Strategy::BearPutSpread {
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            spot
        } => {
            tracing::info!("Running the Bear Put Spread");
            
            let bear_put_spread = spread::BearPutSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                Position::Long,
            ).with_context(|| "failed to build bear put spread".to_string())?;
            
            let payoff = bear_put_spread.payoff(Spot(spot)).with_context(|| "failed to payoff spot of the bear put spread".to_string())?;
            let pnl = bear_put_spread.pnl(Spot(spot)).with_context(|| "failed to pnl spot of the bear put spread".to_string())?;
            
            tracing::info!("The payoff of the Bear Put Spread is: {}", payoff);
            tracing::info!("The pnl of the Bear Call Spread is: {}", pnl);
            
            Ok(())
        },

        Strategy::Straddle {
            strike,
            call_premium,
            put_premium,
            position,
            spot
        } => {
            let straddle = volatility_strategies::Straddle::build(
                Strike(strike),
                Premium(call_premium),
                Premium(put_premium),
                position
            ).with_context(|| "failed to build straddle".to_string())?;

            let payoff = straddle.payoff(Spot(spot)).with_context(|| "failed to payoff straddle".to_string())?;
            let pnl = straddle.pnl(Spot(spot)).with_context(|| "failed to pnl straddle".to_string())?;

            tracing::info!("The payoff of the Straddle is: {}", payoff);
            tracing::info!("The pnl of the Straddle is: {}", pnl);

            Ok(())
        },

        Strategy::Strangle {
            call_strike,
            call_premium,
            put_strike,
            put_premium,
            position,
            spot
        } =>  {
            let strangle = volatility_strategies::Strangle::build(
                Strike(call_strike),
                Premium(call_premium),
                Strike(put_strike),
                Premium(put_premium),
                position
            ).with_context(|| "failed to build strangle".to_string())?;

            let payoff = strangle.payoff(Spot(spot)).with_context(|| "failed to payoff strangle".to_string())?;
            let pnl = strangle.pnl(Spot(spot)).with_context(|| "failed to pnl strangle".to_string())?;

            tracing::info!("The payoff of Strangle is: {}", payoff);
            tracing::info!("The pnl of the Strangle is: {}", pnl);

            Ok(())
        },

        Strategy::IronCondor {
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium,
            spot
        } => {
            let iron_condor = range_bound::IronCondor::build(
                Strike(long_put_strike),
                Premium(long_put_premium),
                Strike(short_put_strike),
                Premium(short_put_premium),
                Strike(short_call_strike),
                Premium(short_call_premium),
                Strike(long_call_strike),
                Premium(long_call_premium)
            ).with_context(|| "failed to build iron_condor".to_string())?;

            let payoff = iron_condor.payoff(Spot(spot)).with_context(|| "failed to payoff iron_condor".to_string())?;
            let pnl = iron_condor.pnl(Spot(spot)).with_context(|| "failed to pnl iron_condor".to_string())?;


            tracing::info!("The payoff of the Iron condor is: {}", payoff);
            tracing::info!("The pnl of the Iron condor is: {}", pnl);

            Ok(())
        },

        Strategy::IronButterfly {
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium,
            spot
        } => {
            let iron_butterfly = range_bound::IronButterfly::build(
                Strike(long_put_strike),
                Premium(long_put_premium),
                Strike(short_strike),
                Premium(short_put_premium),
                Premium(short_call_premium),
                Strike(long_call_strike),
                Premium(long_call_premium)
            ).with_context(|| "failed to build iron_butterfly".to_string())?;
            
            let payoff = iron_butterfly.payoff(Spot(spot)).with_context(|| "failed to payoff iron_butterfly".to_string())?;
            let pnl = iron_butterfly.pnl(Spot(spot)).with_context(|| "failed to pnl iron_butterfly".to_string())?;

            tracing::info!("The payoff of the Iron Butterfly is: {}", payoff);
            tracing::info!("The pnl of the Iron Butterfly is: {}", pnl);

            Ok(())
        }
    }
}