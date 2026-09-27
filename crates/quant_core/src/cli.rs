use clap::{Parser, Subcommand};
use crate::payoffs::categorical_options::{CallPutCategory, LongShortCategory};

// -----------------------------------------------------------------------------
// Main CLI Structure
// -----------------------------------------------------------------------------
/// Command-line structure for options payoff evaluation
#[derive(Parser, Debug)]
#[command(
    name = "payoff",
    author,
    version,
    about = "CLI tool for simulating and calculating financial option payoffs",
    long_about = "Evaluates profit and loss (payoff) profiles for various option trading strategies at maturity."
)]
pub struct Cli {
    #[command(subcommand)]
    pub strategy: Strategy,
}

// -----------------------------------------------------------------------------
// Strategy Subcommands
// -----------------------------------------------------------------------------
/// Supported option trading strategies
#[derive(Subcommand, Debug)]
pub enum Strategy {
    /// Single option payoff (Call or Put)
    SimpleOption {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Strike price of the option
        #[arg(short = 'k', long)]
        strike: f64,
        /// Option premium (price paid or received)
        #[arg(short = 'p', long)]
        prime: f64,
        /// Option type: call or put
        #[arg(short = 'c', long, value_enum)]
        category: CallPutCategory,
    },
    /// Bull Spread strategy (Bull Call Spread or Bull Put Spread)
    BullSpread {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Upper strike price (K_up)
        #[arg(short = 'u', long = "strike-up")]
        strike_up: f64,
        /// Lower strike price (K_down)
        #[arg(short = 'd', long = "strike-down")]
        strike_down: f64,
        /// Spread type: call (Bull Call Spread) or put (Bull Put Spread)
        #[arg(short = 'c', long, value_enum)]
        category: CallPutCategory,
        /// Premium of the option with the higher strike (K_up)
        #[arg(long = "prime-up")]
        prime_up: f64,
        /// Premium of the option with the lower strike (K_down)
        #[arg(long = "prime-down")]
        prime_down: f64,
    },
    /// Bear Spread strategy (Bear Call Spread or Bear Put Spread)
    BearSpread {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Upper strike price (K_up)
        #[arg(short = 'u', long = "strike-up")]
        strike_up: f64,
        /// Lower strike price (K_down)
        #[arg(short = 'd', long = "strike-down")]
        strike_down: f64,
        /// Spread type: call (Bear Call Spread) or put (Bear Put Spread)
        #[arg(short = 'c', long, value_enum)]
        category: CallPutCategory,
        /// Premium of the option with the higher strike (K_up)
        #[arg(long = "prime-up")]
        prime_up: f64,
        /// Premium of the option with the lower strike (K_down)
        #[arg(long = "prime-down")]
        prime_down: f64,
    },
    /// Straddle strategy (Simultaneous Call and Put at the same strike)
    Straddle {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Shared strike price for both Call and Put
        #[arg(short = 'k', long)]
        strike: f64,
        /// Call option premium
        #[arg(long = "call-prime")]
        call_prime: f64,
        /// Put option premium
        #[arg(long = "put-prime")]
        put_prime: f64,
        /// Straddle position: long or short
        #[arg(short = 'c', long, value_enum)]
        category: LongShortCategory,
    },
    /// Strangle strategy (OTM Put and OTM Call at different strike prices)
    Strangle {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Higher strike price (Call)
        #[arg(short = 'u', long = "strike-up")]
        strike_up: f64,
        /// Lower strike price (Put)
        #[arg(short = 'd', long = "strike-down")]
        strike_down: f64,
        /// Call option premium (strike_up)
        #[arg(long = "call-prime")]
        call_prime: f64,
        /// Put option premium (strike_down)
        #[arg(long = "put-prime")]
        put_prime: f64,
        /// Strangle position: long or short
        #[arg(short = 'c', long, value_enum)]
        category: LongShortCategory,
    },
    /// Iron Condor strategy (Combination of Bull Put Spread and Bear Call Spread)
    IronCondor {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Strike K1 (Long Put - lowest strike)
        #[arg(long = "k1")]
        strike_k1: f64,
        /// Strike K2 (Short Put)
        #[arg(long = "k2")]
        strike_k2: f64,
        /// Strike K3 (Short Call)
        #[arg(long = "k3")]
        strike_k3: f64,
        /// Strike K4 (Long Call - highest strike)
        #[arg(long = "k4")]
        strike_k4: f64,
        /// Premium of the Put at strike K1
        #[arg(long = "p1")]
        put_prime_1: f64,
        /// Premium of the Put at strike K2
        #[arg(long = "p2")]
        put_prime_2: f64,
        /// Premium of the Call at strike K3
        #[arg(long = "c3")]
        call_prime_1: f64,
        /// Premium of the Call at strike K4
        #[arg(long = "c4")]
        call_prime_2: f64,
    },
    /// Iron Butterfly strategy (Short Straddle bounded by protective Long Put and Long Call)
    IronButterfly {
        /// Current spot price of the underlying asset
        #[arg(short = 's', long)]
        spot: f64,
        /// Central strike price (K_main / ATM)
        #[arg(short = 'm', long = "strike-main")]
        strike_main: f64,
        /// Lower strike price (K_down - Long Put)
        #[arg(short = 'd', long = "strike-down")]
        strike_down: f64,
        /// Upper strike price (K_up - Long Call)
        #[arg(short = 'u', long = "strike-up")]
        strike_up: f64,
        /// Call premium at the central strike
        #[arg(long = "main-call-prime")]
        main_call_prime: f64,
        /// Put premium at the central strike
        #[arg(long = "main-put-prime")]
        main_put_prime: f64,
        /// Put premium at the lower strike (K_down)
        #[arg(long = "down-prime")]
        down_prime: f64,
        /// Call premium at the upper strike (K_up)
        #[arg(long = "up-prime")]
        up_prime: f64,
    },
}