use crate::payoffs::categorical_options::{CallPutCategory, LongShortCategory};
use clap::{Parser, Subcommand};

/// # The structure of the Command line
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "The structure of the command line of payoff"
)]
#[command(name = "payoff")]
pub struct Cli {
    #[clap(subcommand)]
    pub strategy: Strategy,
}

/// ## The Subcommand Strategy and their structure
#[derive(Subcommand, Debug)]
pub enum Strategy {
    /// ### The simplest strategy that exists
    SimpleOption {
        /// The spot price of the option
        #[clap(short = 's', long)]
        spot: f64,
        /// The strike of the option
        #[clap(short = 'k', long)]
        strike: f64,
        /// The price of the option
        #[clap(short = 'p', long)]
        prime: f64,
        /// The type of the option: Call or Put
        #[clap(short = 'c', long)]
        category: CallPutCategory,
    },

    /// ### Command line's structure of a Bull spread strategy
    BullSpread {
        /// The spot price
        #[clap(short = 's', long)]
        spot: f64,
        /// The strike of the highest option
        #[clap(short = 'u', long)]
        strike_up: f64,
        /// The strike of the lowest option
        #[clap(short = 'd', long)]
        strike_down: f64,
        /// The category of the BullSpread: BullCallSpread (Call) or BullPutSpread (Put)
        #[clap(short, long)]
        category: CallPutCategory,
        /// The prime of the option with the greatest strike
        #[clap(long)]
        prime_up: f64,
        /// The prime of the option with the lowest strike
        #[clap(long)]
        prime_down: f64,
    },

    /// ### Command line's structure of a Bear spread strategy
    BearSpread {
        /// The spot price
        #[clap(short = 's', long)]
        spot: f64,
        #[clap(short = 'u', long)]
        /// The strike of the highest option
        strike_up: f64,
        /// The strike of the lowest option
        #[clap(short = 'd', long)]
        strike_down: f64,
        /// The category of the BullSpread: BearCallSpread (Call) or BearPutSpread (Put)
        #[clap(short = 'c', long)]
        category: CallPutCategory,
        /// The prime of the option with the greatest strike
        #[clap(long)]
        prime_up: f64,
        /// The prime of the option with the lowest strike
        #[clap(long)]
        prime_down: f64,
    },

    /// ### Command line's structure of a straddle
    Straddle {
        /// The spot price
        #[clap(short = 's', long)]
        spot: f64,
        /// The strike of the option
        #[clap(short = 'k', long)]
        strike: f64,
        /// The prime of the call
        #[clap(long)]
        call_prime: f64,
        /// The prime of the put
        #[clap(long)]
        put_prime: f64,
        /// The category of the Straddle: Long straddle (Long) Short Straddle (Short)
        #[clap(short = 'c', long)]
        category: LongShortCategory,
    },

    /// ### Command line's structure of a strangle
    Strangle {
        /// The spot price
        #[clap(short = 's', long)]
        spot: f64,
        /// The highest strike
        #[clap(short = 'u', long)]
        strike_up: f64,
        /// The lowest strike
        #[clap(short = 'd', long)]
        strike_down: f64,
        /// The prime of the call
        #[clap(short, long)]
        call_prime: f64,
        /// The prime of the put
        #[clap(long)]
        put_prime: f64,
        /// The category of the Strangle: Long strangle (Long) Short Strangle (Short)
        #[clap(short = 'c', long)]
        category: LongShortCategory,
    },

    /// ### Command line's structure of an iron condor
    IronCondor {
        /// The spot price
        #[clap(short, long)]
        spot: f64,
        /// The strike of the lowest option of the bull spread
        #[clap(short, long)]
        strike_down1: f64,
        /// The strike of the highest option of the bull spread
        #[clap(short, long)]
        strike_up1: f64,
        /// The strike of the lowest option of the bear spread
        #[clap(short, long)]
        strike_down2: f64,
        /// The strike of the highest option of the bear spread
        #[clap(short, long)]
        strike_up2: f64,
        /// The prime of the call with the lowest strike
        #[clap(short, long)]
        call_prime1: f64,
        /// The prime of the call with the highest strike
        #[clap(short, long)]
        call_prime2: f64,
        /// The prime of the put the lowest strike
        #[clap(short, long)]
        put_prime1: f64,
        /// The prime of the put the highest strike
        #[clap(short, long)]
        put_prime2: f64,
    },

    /// The command line's structure of the Iron butterfly
    IronButterfly {
        /// The spot price
        #[clap(short = 's', long)]
        spot: f64,
        /// The main strike price
        #[clap(short = 'm', long)]
        strike_main: f64,
        /// The strike of the lowest option
        #[clap(short = 'd', long)]
        strike_down: f64,
        /// The strike of the highest option
        #[clap(short = 'u', long)]
        strike_up: f64,
        /// The prime of the call with the main strike
        #[clap(short, long)]
        main_call_prime: f64,
        /// The prime of the put with the main strike
        #[clap(short, long)]
        main_put_prime: f64,
        /// The prime of the put option (the lowest strike)
        #[clap(short, long)]
        down_prime: f64,
        /// The prime of the call option (the highest strike)
        #[clap(short, long)]
        up_prime: f64,
    },
}
