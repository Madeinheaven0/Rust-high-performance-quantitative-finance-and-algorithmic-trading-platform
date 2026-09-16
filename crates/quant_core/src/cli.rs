use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
struct PayoffCommand {
    #[clap(subcommand)]
    claim: Claim,
}

#[derive(Subcommand)]
enum Claim {
    Call {
        spot_price: f64,
        strike_price: f64,
    },
    Put {
        spot_price: f64,
        strike_price: f64,
    },
    BullCallSpread {
        spot_price1: f64,
        spot_price2: f64,
        strike_price1: f64,
        strike_price2: f64,
    },
    BullPutSpread {
        spot_price1: f64,
        spot_price2: f64,
        strike_price1: f64,
        strike_price2: f64,
    },
}
