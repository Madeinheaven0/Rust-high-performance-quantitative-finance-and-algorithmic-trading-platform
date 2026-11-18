use std::cmp::Reverse;
use quant_book::domain::*;
use quant_book::functions::*;

#[test]
fn test_sequential_order_flow_state_and_fifo() {
    // 1. Initialize pre-populated order book
    // Initial Bids: 9950 (Qty 10 + 25), 9900 (Qty 50), 9850 (Qty 100)
    // Initial Asks: 10050 (Qty 15), 10100 (Qty 30 + 20), 10200 (Qty 80)
    let mut book = create_order_book();

    // -------------------------------------------------------------------------
    // STEP 1: Aggressive Buy Sweep across two Ask levels
    // -------------------------------------------------------------------------
    let aggressive_buy = Order {
        id: Id(101),
        price: Price(10100),
        quantity: Quantity(35),
        side: Side::Buy,
    };

    let trades_step1 = book.add_order(aggressive_buy).expect("Buy order should succeed");

    // Verify generated trades: 15 @ 10050 (Id 5) + 20 @ 10100 (Id 6)
    assert_eq!(trades_step1.len(), 2);
    assert_eq!(trades_step1[0].price, Price(10050));
    assert_eq!(trades_step1[0].quantity, Quantity(15));
    assert_eq!(trades_step1[1].price, Price(10100));
    assert_eq!(trades_step1[1].quantity, Quantity(20));

    // Verify Book & Index state after Step 1:
    // - Price level 10050 must be completely cleared
    assert!(!book.asks.contains_key(&10050));
    assert!(!book.index.contains_key(&5));

    // - Price level 10100 must have Id(6) partially filled (10 remaining) and Id(7) untouched (20)
    let ask_queue_10100 = book.asks.get(&10100).expect("Level 10100 should still exist");
    assert_eq!(ask_queue_10100.len(), 2);
    assert_eq!(ask_queue_10100[0].id, Id(6));
    assert_eq!(ask_queue_10100[0].quantity, Quantity(10));
    assert_eq!(ask_queue_10100[1].id, Id(7));
    assert_eq!(ask_queue_10100[1].quantity, Quantity(20));

    // -------------------------------------------------------------------------
    // STEP 2: Passive Limit Sell Addition (no crossing)
    // -------------------------------------------------------------------------
    let passive_sell = Order {
        id: Id(102),
        price: Price(10150),
        quantity: Quantity(50),
        side: Side::Sell,
    };

    let trades_step2 = book.add_order(passive_sell).expect("Passive sell should succeed");

    // Verify no trades occurred and order is indexed
    assert!(trades_step2.is_empty());
    assert_eq!(book.index.get(&102), Some(&(Side::Sell, 10150)));
    assert_eq!(book.asks.get(&10150).unwrap().front().unwrap().quantity, Quantity(50));

    // -------------------------------------------------------------------------
    // STEP 3: Aggressive Sell Sweep verifying Price-Time Priority (FIFO)
    // -------------------------------------------------------------------------
    let aggressive_sell = Order {
        id: Id(103),
        price: Price(9900),
        quantity: Quantity(60),
        side: Side::Sell,
    };

    let trades_step3 = book.add_order(aggressive_sell).expect("Aggressive sell should succeed");

    // Verify FIFO execution order:
    // 1. Id(1) @ 9950 for 10
    // 2. Id(2) @ 9950 for 25
    // 3. Id(3) @ 9900 for 25 (partial fill out of 50)
    assert_eq!(trades_step3.len(), 3);
    assert_eq!(trades_step3[0].quantity, Quantity(10));
    assert_eq!(trades_step3[1].quantity, Quantity(25));
    assert_eq!(trades_step3[2].price, Price(9900));
    assert_eq!(trades_step3[2].quantity, Quantity(25));

    // Verify final Bid state & Index consistency:
    // - Level 9950 must be empty and removed from BTreeMap
    assert!(!book.bids.contains_key(&Reverse(9950)));
    assert!(!book.index.contains_key(&1));
    assert!(!book.index.contains_key(&2));

    // - Best Bid is now 9900 with 25 remaining on Id(3)
    let best_bid_queue = book.bids.get(&Reverse(9900)).expect("Level 9900 should remain");
    assert_eq!(best_bid_queue.len(), 1);
    assert_eq!(best_bid_queue[0].id, Id(3));
    assert_eq!(best_bid_queue[0].quantity, Quantity(25));
    assert_eq!(book.index.get(&3), Some(&(Side::Buy, 9900)));
}