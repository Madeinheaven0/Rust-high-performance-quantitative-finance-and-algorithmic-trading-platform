use clap::{Parser, Subcommand};
use crate::payoffs::domain_types::*;

#[derive(Parser)]
#[command(
    name = "quant command option's strategy",
    version = "0.1.0",
    author = "Epee Epee Landry <epeeepeelandry887@gmail.com>",
    about = "Quantum Quantum Component"
)]
pub struct Args {
    #[clap(subcommand)]
    pub strategy: Strategy,
}

#[derive(Subcommand)]
pub enum Strategy {
    /// A simple option Call or Put
    Classic {
        #[arg(long, short='s')]
        spot: f64,
        #[arg(long, short='k', value_enum)]
        /// The type of the option (Call or Put)
        kind: OptionKind,
        #[arg(value_enum)]
        /// The position of the option (Long or Short)
        side: Side,
        /// The price of the spot
        strike: f64,
        #[arg(long, short='p')]
        premium: f64,
    },
    
    /// The BullCallSpread strategy
    BullCallSpread {
        #[arg(long, short='s')]
        spot: f64,
        /// Strike of the cheaper Call
        strike_up: f64,
        /// The strike of the expensive Call
        strike_down: f64,
        /// The premium of the expensive Call
        premium_up: f64,
        /// The premium of the cheaper Call
        premium_down: f64,
        /// The position of the customer
        #[arg(value_enum)]
        side: Side
    },
    
    /// The BullPutSpread Strategy
    BullPutSpread {
        #[arg(long, short='s')]
        spot: f64,
        strike_up: f64,
        strike_down: f64,
        premium_up: f64,
        premium_down: f64,
        #[arg(value_enum)]
        side: Side
    },
    
    /// The BearCAllSpread Strategy
    BearCallSpread {
        #[arg(long, short='s')]
        spot: f64,
        /// The strike of expensive Put
        strike_up: f64,
        /// The strike of the cheaper Put
        strike_down: f64,
        /// The premium of the expensive Put
        premium_up: f64,
        /// The premium of the cheaper Put
        premium_down: f64,
        #[arg(value_enum)]
        side: Side
    },
    
    /// The BearPutSpread Strategy
    BearPutSpread {
        #[arg(long, short='s')]
        spot: f64,
        strike_up: f64,
        strike_down: f64,
        premium_up: f64,
        premium_down: f64,
        #[arg(value_enum)]
        side: Side
    },
    
    /// The Straddle Strategy
    Straddle {
        #[arg(long, short='s')]
        spot: f64,
        strike: f64,
        #[arg(long, short='c')]
        call_premium: f64,
        #[arg(long, short='p')]
        put_premium: f64,
        #[arg(value_enum)]
        side: Side
    },
    
    /// The Strangle Strategy
    Strangle {
        #[arg(long, short='s')]
        spot: f64,
        call_strike: f64,
        call_premium: f64,
        put_strike: f64,
        put_premium: f64,
        #[arg(value_enum)]
        side: Side
    },
    
    /// IronCondor Strategy
    IronCondor {
        #[arg(long, short='s')]
        spot: f64,
        long_put_strike: f64,
        long_put_premium: f64,
        short_put_strike: f64,
        short_put_premium: f64,
        short_call_strike: f64,
        short_call_premium: f64,
        long_call_strike: f64,
        long_call_premium: f64,
    },
    
    /// IronButterfly Strategy
    IronButterfly {
        #[arg(long, short='s')]
        spot: f64,
        long_put_strike: f64,
        long_put_premium: f64,
        short_strike: f64,
        short_put_premium: f64,
        short_call_premium: f64,
        long_call_strike: f64,
        long_call_premium: f64,
    }
}
