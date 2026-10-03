use clap::{Parser, Command, Subcommand, ValueEnum};
use crate::payoffs::domain_types::{OptionKind, Position, Premium, Spot, Strike};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub strategy: Strategy,
}


#[derive(Subcommand, Debug)]
pub enum Strategy {
    /// The simple option strategies (Call or Put)
    Basic {
        /// The type of the option (Call or Put)
        #[clap(value_enum)]
        #[arg(long, short='k')]
        kind: OptionKind,
        /// The side (Long or Short)
        #[clap(value_enum)]
        #[arg(long, short='p')]
        position: Position,
        /// The strike
        #[arg(long)]
        strike: f64,
        /// The premium of the option
        #[arg(long)]
        premium: f64,
        /// The price of the asset
        #[arg(long, short='s')]
        spot: f64,
    },

    BullCallSpread {
        #[arg(long, short='u')]
        strike_up: f64,
        /// The premium of the option with the highest strike
        #[arg(long)]
        premium_up: f64,
        /// The lowest option strategy strike
        #[arg(long, short='d')]
        strike_down: f64,
        /// The premium of the option with the lowest strike
        #[arg(long)]
        premium_down: f64,
        #[arg(long, short='s')]
        spot: f64,
    },

    BullPutSpread {
        /// The highest option strategy strike
        #[arg(long, short='u')]
        strike_up: f64,
        /// The premium of the option with the highest strike
        #[arg(long)]
        premium_up: f64,
        /// The lowest option strategy strike
        #[arg(long, short='d')]
        strike_down: f64,
        /// The premium of the option with the lowest strike
        #[arg(long)]
        premium_down: f64,
        #[arg(long, short='s')]
        spot: f64,
    },

    BearCallSpread {
        /// The highest option strategy strike
        #[arg(long, short='u')]
        strike_up: f64,
        /// The premium of the option with the highest strike
        #[arg(long)]
        premium_up: f64,
        /// The lowest option strategy strike
        #[arg(long, short='d')]
        strike_down: f64,
        /// The premium of the option with the lowest strike
        #[arg(long)]
        premium_down: f64,
        #[arg(long, short='s')]
        spot: f64,
    },

    BearPutSpread {
        /// The highest option strategy strike
        #[arg(long, short='u')]
        strike_up: f64,
        /// The premium of the option with the highest strike
        #[arg(long)]
        premium_up: f64,
        /// The lowest option strategy strike
        #[arg(long, short='d')]
        strike_down: f64,
        /// The premium of the option with the lowest strike
        #[arg(long)]
        premium_down: f64,
        /// The price of the asset
        #[arg(long, short='s')]
        spot: f64,
    },

    Straddle {
        /// The unique strike
        #[arg(long, short='s')]
        strike: f64,
        /// The premium of the call
        #[arg(long)]
        call_premium: f64,
        /// The premium of the put
        #[arg(long)]
        put_premium: f64,
        /// The position (Long or Short)
        #[clap(value_enum)]
        #[arg(long, short='p')]
        position: Position,
        /// The price of the asset
        #[arg(long, short='s')]
        spot: f64
    },

    Strangle {
        /// The strike of the call
        #[arg(long)]
        call_strike: f64,
        /// The premium of the call
        #[arg(long)]
        call_premium: f64,
        /// The strike of the put
        #[arg(long)]
        put_strike: f64,
        /// The premium of the put
        #[arg(long)]
        put_premium: f64,
        /// The position (Long or Short)
        #[clap(value_enum)]
        #[arg(long)]
        position: Position,
        /// The price of the asset
        #[arg(long, short='s')]
        spot: f64
    },

    IronCondor {
        /// The strike of the long put
        #[arg(long)]
        long_put_strike: f64,
        /// The premium of the long put
        #[arg(long)]
        long_put_premium: f64,
        /// The strike of the short put
        #[arg(long)]
        short_put_strike: f64,
        /// The premium of the short put
        #[arg(long)]
        short_put_premium: f64,
        /// The strike of the short call
        #[arg(long)]
        short_call_strike: f64,
        /// The premium of the short call
        #[arg(long)]
        short_call_premium: f64,
        /// The strike of the long call
        #[arg(long)]
        long_call_strike: f64,
        /// The premium of the long call
        #[arg(long)]
        long_call_premium: f64,
        /// The price of the asset
        #[arg(long, short='s')]
        spot: f64
    },

    IronButterfly {
        /// The strike of the long put
        #[arg(long)]
        long_put_strike: f64,
        /// The premium of the long put
        #[arg(long)]
        long_put_premium: f64,
        /// The strike of the call and the put to short
        #[arg(long)]
        short_strike: f64,
        /// The premium of the short put
        #[arg(long)]
        short_put_premium: f64,
        /// The premium of the short call
        #[arg(long)]
        short_call_premium: f64,
        /// The strike of the long call
        #[arg(long)]
        long_call_strike: f64,
        /// The premium of the long call
        #[arg(long)]
        long_call_premium: f64,
        /// The price of the asset
        #[arg(long)]
        spot: f64
    }
}