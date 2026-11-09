use clap::Parser;
use tracing::{info, debug, error};
use anyhow::{Result, Context};
use quant_core::cli;
use quant_core::cli::Strategy;
use quant_core::telemetry;
use quant_core::payoffs::domain_types::*;
use quant_core::payoffs::{leg::OptionLeg, spread, volatility_strategies, range_bound};
use quant_core::payoffs::range_bound::RangeStrategy;
use quant_core::payoffs::spread::SpreadStrategy;
use quant_core::payoffs::volatility_strategies::VolatilityStrategy;

fn main() -> Result<(), anyhow::Error> {
    telemetry::init_telemetry().unwrap();
    let args = cli::Args::parse();
    matching(args).context("Something went wrong")?;
    
    Ok(())
}

fn matching(args: cli::Args) ->  Result<()>{
    match args.strategy {
        Strategy::Classic {
            spot,
            kind,
            side,
            strike,
            premium
        } => {
            info!("matching spot {:?}, strike {:?}, kind {:?}, side {:?} for a simple claim", spot, strike, kind, side);
            let spot = Spot(spot);
            let claim = OptionLeg::build(
                kind,
                side,
                Strike(strike),
                Premium(premium),
            ).context("failed to build leg option")?;

            let payoff = claim.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = claim.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::BullCallSpread {
            spot,
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            side
        } => {
            info!("\
            matching spot {:?}, \
            strike_up {:?}, \
            strike_down {:?}, \
            side {:?} \
            for a bull call spread strategy",
                spot, strike_up, strike_down,  side
            );
            let spot = Spot(spot);
            let bull_call_spread = spread::BullCallSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                side
            ).context("failed to build bull call spread")?;

            let payoff = bull_call_spread.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = bull_call_spread.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::BullPutSpread {
            spot,
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            side
        } => {
            info!("\
            matching spot {:?}, \
            strike_up {:?}, \
            strike_down {:?}, \
            side {:?} \
            for a bull put spread strategy",
            spot, strike_up, strike_down,  side
            );

            let spot = Spot(spot);

            let bull_put_spread = spread::BullPutSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                side
            ).context("failed to build bull put spread")?;

            let payoff = bull_put_spread.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = bull_put_spread.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::BearCallSpread {
            spot,
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            side
        } => {
            info!("\
            matching spot {:?}, \
            strike_up {:?}, \
            strike_down {:?}, \
            side {:?} \
            for a bear call spread strategy",
            spot, strike_up, strike_down,  side
            );

            let spot = Spot(spot);

            let bear_call_spread = spread::BearCallSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                side
            ).context("failed to build bull put spread")?;

            let payoff = bear_call_spread.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = bear_call_spread.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::BearPutSpread {
            spot,
            strike_up,
            strike_down,
            premium_up,
            premium_down,
            side
        } =>  {
            info!("\
            matching spot {:?}, \
            strike_up {:?}, \
            strike_down {:?}, \
            side {:?} \
            for a bear put spread strategy",
            spot, strike_up, strike_down,  side
            );

            let spot = Spot(spot);

            let bear_put_spread = spread::BullPutSpread::build(
                Strike(strike_up),
                Strike(strike_down),
                Premium(premium_up),
                Premium(premium_down),
                side
            ).context("failed to build bull put spread")?;

            let payoff = bear_put_spread.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = bear_put_spread.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::Straddle {
            spot,
            strike,
            call_premium,
            put_premium,
            side
        } => {
            info!("\
            matching spot {:?}, \
            strike 0{:?}, \
            call premium {:?}, \
            put premium {:?}, \
            side {:?} \
            for a straddle",
            spot, strike, call_premium,  put_premium, side
            );

            let spot = Spot(spot);

            let straddle = volatility_strategies::Straddle::build(
                Strike(strike),
                Premium(call_premium),
                Premium(put_premium),
                side
            ).context("failed to build straddle")?;

            let payoff = straddle.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);
            
            let pnl = straddle.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);
            
            Ok(())
        },
        Strategy::Strangle {
            spot,
            call_strike,
            call_premium,
            put_strike,
            put_premium,
            side
        } => {
            info!("\
            matching spot {:?}, \
            call strike {:?}, \
            call premium {:?}, \
            put strike {:?}, \
            put premium {:?}, \
            side {:?} \
            for a strangle",
                spot, 
                call_strike, 
                call_premium, 
                put_strike, 
                put_premium, 
                side
            );
            
            let spot = Spot(spot);
            
            let strangle = volatility_strategies::Strangle::build(
                Strike(call_strike),
                Premium(call_premium),
                Strike(put_strike),
                Premium(put_premium),
                side
            ).context("failed to build strangle")?;

            let payoff = strangle.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = strangle.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::IronCondor {
            spot,
            long_put_strike,
            long_put_premium,
            short_put_strike,
            short_put_premium,
            short_call_strike,
            short_call_premium,
            long_call_strike,
            long_call_premium
        } => {
            info!("\
            matching spot {:?}, \
            long put strike {:?}, \
            long put premium {:?}, \
            short put strike {:?}, \
            short put premium {:?}, \
            short call strike {:?}, \
            short call premium {:?}, \
            long call strike {:?}, \
            long call premium {:?}, \
            for an iron condor",
                spot, 
                long_put_strike,
                long_put_premium, 
                short_put_strike, 
                short_put_premium, 
                short_call_strike, 
                short_call_premium, 
                long_call_strike, 
                long_call_premium
            );
            
            let spot = Spot(spot);
            
            let condor = range_bound::IronCondor::build(
                Strike(long_put_strike),
                Premium(long_put_premium),
                Strike(short_put_strike),
                Premium(short_put_premium),
                Strike(short_call_strike),
                Premium(short_call_premium),
                Strike(long_call_strike),
                Premium(long_call_premium)
            ).context("failed to build Iron condor")?;

            let payoff = condor.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = condor.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        },
        Strategy::IronButterfly {
            spot,
            long_put_strike,
            long_put_premium,
            short_strike,
            short_put_premium,
            short_call_premium,
            long_call_strike,
            long_call_premium
        } => {
            info!("\
            matching spot {:?}, \
            long put strike {:?}, \
            long put premium {:?}, \
            short strike {:?}, \
            short put premium {:?}, \
            short call premium {:?}, \
            long call strike {:?}, \
            long call premium {:?}, \
            for an iron butterfly",
                spot, 
                long_put_strike,
                long_put_premium,
                short_strike,
                short_put_premium,
                short_call_premium, 
                long_call_strike, 
                long_call_premium
            );
            
            let spot = Spot(spot);
            
            let butterfly = range_bound::IronButterfly::build(
                Strike(long_put_strike),
                Premium(long_put_premium),
                Strike(short_strike),
                Premium(short_put_premium),
                Premium(short_call_premium),
                Strike(long_call_strike),
                Premium(long_call_premium)
            ).context("Fail to build Tron Butterfly")?;

            let payoff = butterfly.payoff(spot).context("failed to compute the payoff")?;
            debug!("matching payoff {:?}$", payoff);

            let pnl = butterfly.pnl(spot).context("failed to compute the pnl")?;
            debug!("matching pnl {:?}$", pnl);

            Ok(())
        }
    }
}