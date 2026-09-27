use std::collections::HashMap;
use rand::{Rng, RngExt};
use std::time::Instant;
use tracing_subscriber::prelude::*;
use tracing;
use crate::payoffs::basics::BasicOption;
use crate::{payoffs::spread, payoffs::volatility, payoffs::range_bound, };
use crate::payoffs::categorical_options;

const N: usize = 10000;
fn main() {
    tracing_subscriber::fmt::init();
}

fn create_spot_array() -> [u16; N] {
    let mut rng = rand::rng();
    let array: [u16; N] = rng.random();
    array
}

fn create_spot_vec() -> Vec<u16> {
    let mut rng = rand::rng();
    let array: [u16; N] = rng.random();
    array.to_vec()
}

fn run_bench() {
    tracing::info!("Running benchmarks");

}

