use crate::payoffs::categorical_options::{CallPutCategory, LongShortCategory};
use clap::{Parser, Subcommand};

/// The structure of the Command line
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

#[derive(Subcommand, Debug)]
pub enum Strategy {
    /// The simplest strategy that exists
    SimpleOption {
        /// The spot price of the option
        #[clap(short = 's', long)]
        spot: f64,
        /// The strike of the option
        #[clap(short = 'k', long)]
        strike: f64,
        /// The type of the option: Call or Put
        #[clap(short = 'c', long)]
        category: CallPutCategory,
    },

    /// Command line's structure of a Bull spread strategy
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
    },

    /// Command line's structure of a Bear spread strategy
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
    },

    /// Command line's structure of a straddle
    Straddle {
        /// The spot price
        #[clap(short = 's', long)]
        spot: f64,
        /// The strike of the option
        #[clap(short = 'k', long)]
        strike: f64,
        /// The category of the Straddle: Long straddle (Long) Short Straddle (Short)
        #[clap(short = 'c', long)]
        category: LongShortCategory,
    },

    /// Command line's structure of a strangle
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
        /// The category of the Strangle: Long strangle (Long) Short Strangle (Short)
        #[clap(short = 'c', long)]
        category: LongShortCategory,
    },

    /// Command line's structure of an iron condor
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
    },

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
        #[clap(short = 'u', long)]
        /// The strike of the highest option
        strike_up: f64,
    },
}
