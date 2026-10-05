#[path = "payoffs/mod.rs"]
pub mod payoffs;
pub mod errors;

use std::time::Instant;
use crate::payoffs::domain_types::{OptionKind, Position, Premium, Spot, Strike};
use errors::StrategyError;
use crate::payoffs::leg::OptionLeg;
use crate::payoffs::{domain_types, leg, range_bound, spread, volatility_strategies};
use rand::{rng, Rng, RngExt};
use tracing;
use tracing_subscriber;

const N: usize = 30_000;

fn main() {
    tracing_subscriber::fmt::init();

    bench_all().unwrap()
}

fn created_table_basic_options() -> Result<[OptionLeg; N], StrategyError> {
    let mut options: Vec<OptionLeg> = Vec::with_capacity(N);
    for _ in 0..N {
        let basic = OptionLeg::build(OptionKind::Call, Position::Long, Strike(100.), Premium(7.))?;
        options.push(basic);
    }

    let options_array = options.try_into().map_err(|_| panic!())?;

    Ok(options_array)
}

fn created_vector_basic_options() -> Result<Vec<OptionLeg>, StrategyError> {
    let mut options: Vec<leg::OptionLeg> = Vec::with_capacity(N);
    for _ in 0..N {
        let basic = OptionLeg::build(OptionKind::Call, Position::Long, Strike(100.), Premium(7.))?;
        options.push(basic);
    }

    Ok(options)
}

fn bench_array_option_payoff(options: [OptionLeg; N]) -> Result<(), StrategyError> {
    tracing::info!("Start the array option bench");
    let rng = rand::rng();
    let spots: [f64; N] = rng.random_iter().take(N).collect::<Vec<f64>>().try_into().unwrap();

    let begin = Instant::now();

    let payoffs = options
        .iter()
        .zip(spots.iter())
        .map(|(option, s)| {
            option.payoff(Spot(*s)).unwrap();
        })
        .collect::<Vec<_>>();

    let _ = begin.elapsed();

    tracing::info!("Duration: {:?}", begin.elapsed().as_secs_f64());

    Ok(())
}

fn bench_vector_option_payoff(options: Vec<OptionLeg>) -> Result<(), StrategyError> {
    tracing::info!("Start the vector option bench");
    let mut rng = rand::rng();
    let spots: Vec<f64> = rng.random_iter().take(N).collect::<Vec<f64>>();

    let begin = Instant::now();

    let payoffs = options
        .iter()
        .zip(spots.iter())
        .map(|(option, s)| {
            option.payoff(Spot(*s)).unwrap();
        })
        .collect::<Vec<_>>();

    let _ = begin.elapsed();

    tracing::info!("Duration: {:?}", begin.elapsed().as_secs_f64());

    Ok(())
}

fn bench_all() -> Result<(), StrategyError> {
    tracing::info!("Start the bench");

    let array_options = created_table_basic_options()?;
    let vector_options = created_vector_basic_options()?;

    bench_array_option_payoff(array_options)?;
    bench_vector_option_payoff(vector_options)?;

    Ok(())
}
