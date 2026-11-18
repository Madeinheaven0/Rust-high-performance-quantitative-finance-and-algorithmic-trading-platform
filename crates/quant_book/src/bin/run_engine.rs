use quant_book::domain::*;
use quant_book::errors::BookError;
use quant_book::functions::*;
use quant_core::payoffs::domain_types::{OptionKind, Premium, Side as OptionSide, Spot, Strike};
use quant_core::payoffs::{
    leg::OptionLeg, range_bound::IronCondor, spread::BullCallSpread,
    volatility_strategies::Straddle,
};
use quant_core::telemetry::init_telemetry;
use quant_exec::engine::*;
use tracing::{info, instrument};

#[instrument]
fn main() -> Result<(), BookError> {
    // Initialize tracing/telemetry subscriber
    init_telemetry().unwrap();
    info!("--- Starting Enriched LOB + Parallel Engines Simulation ---");

    // 1. Initialize the pre-populated limit order book
    let mut book = create_order_book();
    info!("Limit order book successfully initialized with initial bids and asks.");

    // 2. Define an incoming sequence of market orders (Order Flow Simulation)
    let incoming_orders = vec![
        // Step 1: An aggressive buy order sweeping the lower ask levels (10050 and 10100)
        (
            "Aggressive Buy Sweep",
            Order {
                id: Id(101),
                price: Price(10100),
                quantity: Quantity(35),
                side: Side::Buy,
            },
        ),
        // Step 2: A passive limit sell order adding depth at 10150
        (
            "Passive Limit Sell Addition",
            Order {
                id: Id(102),
                price: Price(10150),
                quantity: Quantity(50),
                side: Side::Sell,
            },
        ),
        // Step 3: An aggressive sell order dropping down to hit resting bids
        (
            "Aggressive Sell Sweep",
            Order {
                id: Id(103),
                price: Price(9900),
                quantity: Quantity(60),
                side: Side::Sell,
            },
        ),
    ];

    // 3. Define heterogeneous strategy portfolios (independent of spot dimensions)
    let option_legs = vec![
        OptionLeg::build(OptionKind::Call, OptionSide::Long, Strike(100.0), Premium(3.0)).unwrap(),
        OptionLeg::build(OptionKind::Put, OptionSide::Short, Strike(95.0), Premium(2.5)).unwrap(),
    ];

    let spreads = vec![
        BullCallSpread::build(Strike(102.0), Strike(99.0), Premium(0.5), Premium(2.0), OptionSide::Long).unwrap(),
    ];

    let straddles = vec![
        Straddle::build(Strike(100.0), Premium(3.0), Premium(3.0), OptionSide::Long).unwrap(),
    ];

    let condors = vec![
        IronCondor::build(
            Strike(70.0), Premium(2.0),
            Strike(80.0), Premium(3.0),
            Strike(90.0), Premium(4.0),
            Strike(100.0), Premium(5.0),
        ).unwrap(),
    ];

    // 4. Process the order flow sequentially and evaluate engines at each step
    println!("\n--- Processing Sequential Order Flow & Market Ticks ---");

    for (step_name, order) in incoming_orders {
        info!(step = step_name, ?order, "Processing incoming order");
        println!("\n[Event] Action: {} (Order ID: {:?}, Price: {}, Qty: {})",
                 step_name, order.id, order.price.0, order.quantity.0);

        // Execute matching on the shared book state
        let trades = match book.add_order(order) {
            Ok(t) => t,
            Err(e) => {
                info!(error = ?e, "Order execution failed");
                println!("  -> Order rejected or failed: {:?}", e);
                continue;
            }
        };

        if trades.is_empty() {
            println!("  -> Order rested in the book without immediate matching (passive order).");
            continue;
        }

        println!("  -> Trades executed: {} trade(s)", trades.len());
        for trade in &trades {
            println!("     * Executed Price: ${:.2} | Qty: {}", trade.price.0 as f64 / 100.0, trade.quantity.0);
        }

        // Convert generated trades into Spot prices for quant engines
        let spots = trades_to_spots(&trades);

        // Build engines dynamically for this batch of trades
        let option_engine = OptionEngine::build(&option_legs, &spots).unwrap();
        let spread_engine = SpreadEngine::build(&spreads, &spots).unwrap();
        let vol_engine = VolatilityEngine::build(&straddles, &spots).unwrap();
        let range_engine = RangeEngine::build(&condors, &spots).unwrap();

        // Compute parallel PnLs via Rayon for these specific spot ticks
        let opt_pnls = option_engine.pnls().unwrap();
        let spread_pnls = spread_engine.pnls().unwrap();
        let vol_pnls = vol_engine.pnls().unwrap();
        let range_pnls = range_engine.pnls().unwrap();

        println!("  -> Portfolio PnLs for this tick batch:");
        println!("     - Option Engine PnLs     : {:?}", opt_pnls);
        println!("     - Spread Engine PnLs     : {:?}", spread_pnls);
        println!("     - Volatility Engine PnLs : {:?}", vol_pnls);
        println!("     - Range Engine PnLs      : {:?}", range_pnls);
    }

    info!("Enriched simulation completed successfully!");
    println!("\n--- Enriched Simulation Finished Successfully ---");
    Ok(())
}