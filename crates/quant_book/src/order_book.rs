//! # The Limit Order Book module

use std::cmp::Reverse;

use crate::domain::{Asks, Bids, Index, Order, Side, Trade, TradeBuilder};
use crate::errors::BookError;

use tracing::{debug, error, info};

/// ## The structure of the Limit Order Book
pub struct LimitOrderBook {
    pub bids: Bids,
    pub asks: Asks,
    pub index: Index,
}

impl LimitOrderBook {
    pub fn new() -> Self {
        Self {
            bids: Bids::new(),
            asks: Asks::new(),
            index: Index::new(),
        }
    }

    pub fn add_order(&mut self, mut order: Order) -> Result<Vec<Trade>, BookError> {
        match order.side {
            Side::Buy => {
                let mut trades: Vec<Trade> = vec![];

                // As long as there are Asks in the book
                while let Some(&best_ask_price) = self.asks.keys().next() {
                    // If the best ask is higher than our buy price, or if our order is fully filled, stop
                    if best_ask_price > order.price.0 || order.quantity.0 == 0 {
                        break;
                    }

                    let asks_queue = self.asks.get_mut(&best_ask_price).unwrap();

                    while let Some(ask) = asks_queue.front_mut() {
                        if order.quantity.0 == 0 {
                            break;
                        }

                        if ask.quantity.0 == order.quantity.0 {
                            let q = ask.quantity.0;
                            let ask_id = ask.id.0;
                            asks_queue.pop_front();
                            self.index.remove(&ask_id); // Clean up from index when fully consumed

                            let trade = TradeBuilder::new()
                                .seller_id(ask_id)
                                .buyer_id(order.id.0)
                                .price(best_ask_price) // Trade executes at the passive order price (the ask)
                                .quantity(q)
                                .build()?;

                            trades.push(trade);
                            order.quantity.0 = 0;
                            break;
                        } else if ask.quantity.0 > order.quantity.0 {
                            let matched_qty = order.quantity.0;
                            ask.quantity.0 -= matched_qty;

                            let trade = TradeBuilder::new()
                                .seller_id(ask.id.0)
                                .buyer_id(order.id.0)
                                .price(best_ask_price)
                                .quantity(matched_qty)
                                .build()?;

                            trades.push(trade);
                            order.quantity.0 = 0;
                            break;
                        } else {
                            let q = ask.quantity.0;
                            let ask_id = ask.id.0;
                            asks_queue.pop_front();
                            self.index.remove(&ask_id); // Clean up from index when fully consumed

                            let trade = TradeBuilder::new()
                                .seller_id(ask_id)
                                .buyer_id(order.id.0)
                                .price(best_ask_price)
                                .quantity(q)
                                .build()?;

                            trades.push(trade);
                            order.quantity.0 -= q;
                        }
                    }

                    if asks_queue.is_empty() {
                        self.asks.remove(&best_ask_price);
                    }
                }

                // If quantity remains, the order becomes passive and goes into the Bids
                if order.quantity.0 > 0 {
                    let bids_queue = self.bids.entry(Reverse(order.price.0)).or_default();

                    bids_queue.push_back(order.clone());
                    self.index.insert(order.id.0, (Side::Buy, order.price.0));
                }

                Ok(trades)
            }
            Side::Sell => {
                let mut trades: Vec<Trade> = vec![];

                // As long as there are Bids in the book
                while let Some(top_bid_key) = self.bids.keys().next().cloned() {
                    let best_bid_price = top_bid_key.0;

                    // If the best bid is lower than our sell price, or if our order is fully filled, stop
                    if best_bid_price < order.price.0 || order.quantity.0 == 0 {
                        break;
                    }

                    let bids_queue = self.bids.get_mut(&top_bid_key).unwrap();

                    while let Some(bid) = bids_queue.front_mut() {
                        if order.quantity.0 == 0 {
                            break;
                        }

                        if bid.quantity.0 == order.quantity.0 {
                            let q = bid.quantity.0;
                            let bid_id = bid.id.0;
                            bids_queue.pop_front();
                            self.index.remove(&bid_id); // Clean up from index when fully consumed

                            let trade = TradeBuilder::new()
                                .seller_id(order.id.0)
                                .buyer_id(bid_id)
                                .price(best_bid_price) // Trade executes at the bid price
                                .quantity(q)
                                .build()?;

                            trades.push(trade);
                            order.quantity.0 = 0;
                            break;
                        } else if bid.quantity.0 > order.quantity.0 {
                            let matched_qty = order.quantity.0;
                            let bid_id = bid.id.0;
                            bid.quantity.0 -= matched_qty;

                            let trade = TradeBuilder::new()
                                .seller_id(order.id.0)
                                .buyer_id(bid_id)
                                .price(best_bid_price)
                                .quantity(matched_qty)
                                .build()?;

                            trades.push(trade);
                            order.quantity.0 = 0;
                            break;
                        } else {
                            let q = bid.quantity.0;
                            let bid_id = bid.id.0;
                            bids_queue.pop_front();
                            self.index.remove(&bid_id); // Clean up from index when fully consumed

                            let trade = TradeBuilder::new()
                                .seller_id(order.id.0)
                                .buyer_id(bid_id)
                                .price(best_bid_price)
                                .quantity(q)
                                .build()?;

                            trades.push(trade);
                            order.quantity.0 -= q;
                        }
                    }

                    if bids_queue.is_empty() {
                        self.bids.remove(&top_bid_key);
                    }
                }

                // If quantity remains, the sell order becomes passive and goes into the Asks
                if order.quantity.0 > 0 {
                    let asks_queue = self.asks.entry(order.price.0).or_default();
                    asks_queue.push_back(order.clone());
                    self.index.insert(order.id.0, (Side::Sell, order.price.0));
                }

                Ok(trades)
            }
        }
    }

    pub fn cancel_order(&mut self, order: Order) -> Option<Order> {
        let index = self.index.remove(&order.id.0);

        if index.is_none() {
            return None;
        }

        match order.side {
            Side::Buy => {
                let bids = self.bids.get_mut(&Reverse(order.price.0)).unwrap();
                bids.retain(|ord| ord.id.0 != order.id.0);

                if bids.is_empty() {
                    self.bids.remove(&Reverse(order.price.0));
                }

                Some(order)
            }
            Side::Sell => {
                let asks = self.asks.get_mut(&order.price.0).unwrap();
                asks.retain(|ord| ord.id.0 != order.id.0);

                if asks.is_empty() {
                    self.asks.remove(&order.price.0);
                }

                Some(order)
            }
        }
    }

    #[inline]
    pub fn best_bid(&self) -> Option<u64> {
        if self.bids.is_empty() {
            return None;
        }

        Some(self.bids.keys().max()?.0)
    }

    #[inline]
    pub fn best_ask(&self) -> Option<u64> {
        if self.asks.is_empty() {
            return None;
        }

        Some(*self.asks.keys().max()?)
    }

    pub fn spread(&self) -> Option<u64> {
        Some(self.best_ask()? - self.best_bid()?)
    }

    pub fn modify_order(
        &mut self,
        order_id: u64,
        new_quantity: Option<u64>,
        new_price: Option<u64>,
    ) -> Result<Vec<Trade>, BookError> {
        let (side, price) = self
            .index
            .get_mut(&order_id)
            .ok_or(BookError::OrderNotFound)?;
        let current_price = *price;
        let mut needs_reinsertion = false;

        match side {
            Side::Buy => {
                // 1. Retrieving the bid queue
                let queue = self
                    .bids
                    .get_mut(&Reverse(current_price))
                    .ok_or(BookError::OrderNotFound)?;

                // 2. Finding order in the queue
                let bid = queue
                    .iter_mut()
                    .find(|o| o.id.0 == order_id)
                    .ok_or(BookError::OrderNotFound)?;

                // 3. Handling based on the type of change requested
                // Price management (changing the price ALWAYS results in a loss of priority)
                if let Some(p) = new_price {
                    if p != bid.price.0 {
                        needs_reinsertion = true;
                        bid.price.0 = p;
                    }
                }

                // Quantity management
                if let Some(q) = new_quantity {
                    if q > bid.quantity.0 {
                        // Increase in quantity -> Loss of priority (re-injection occurs)
                        needs_reinsertion = true;
                        bid.quantity.0 = q;
                    } else if q < bid.quantity.0 {
                        // Decrease in quantity -> Stays in the same spot! We simply make the adjustment right where it is.
                        bid.quantity.0 = q;
                    }
                }

                // 4. If the price has changed or the quantity has increased, the order must be moved.
                if needs_reinsertion {
                    // The order is removed from its current position and from the index.
                    let pos = queue.iter().position(|o| o.id.0 == order_id).unwrap();
                    let order = queue.remove(pos).unwrap();
                    self.index.remove(&order_id);

                    if queue.is_empty() {
                        self.bids.remove(&Reverse(current_price));
                    }

                    // It is re-injected via add_order (which handles matching or repositioning at the back).
                    return self.add_order(order);
                }

                // Even if there was only a reduction in size, the order remained unchanged.
                // We simply update the index if necessary and return an empty vector of trades.
                Ok(vec![])
            }
            Side::Sell => {
                let queue = self
                    .asks
                    .get_mut(&current_price)
                    .ok_or(BookError::OrderNotFound)?;

                let ask = queue
                    .iter_mut()
                    .find(|o| o.id.0 == order_id)
                    .ok_or(BookError::OrderNotFound)?;

                if let Some(p) = new_price {
                    if p != ask.price.0 {
                        needs_reinsertion = true;
                        ask.price.0 = p;
                    }
                }

                if let Some(q) = new_quantity {
                    if q > ask.quantity.0 {
                        needs_reinsertion = true;
                        ask.quantity.0 = q;
                    } else if q < ask.quantity.0 {
                        ask.quantity.0 = q;
                    }
                }

                if needs_reinsertion {
                    let pos = queue.iter().position(|o| o.id.0 == order_id).unwrap();
                    let order = queue.remove(pos).unwrap();
                    self.index.remove(&order_id);

                    if queue.is_empty() {
                        self.asks.remove(&current_price);
                    }

                    return self.add_order(order);
                }

                Ok(vec![])
            }
        }
    }
}

impl Default for LimitOrderBook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{OrderBuilder, Side};
    use proptest::prelude::*;
    use rstest::rstest;

    // --- 1. Classic unittest ---

    #[test]
    fn test_new_book_is_empty() {
        let book = LimitOrderBook::new();
        assert!(book.bids.is_empty());
        assert!(book.asks.is_empty());
        assert!(book.index.is_empty());
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.spread(), None);
    }

    #[test]
    fn test_add_buy_order_matching_existing_ask() {
        let mut book = LimitOrderBook::new();

        // 1. We put a passif sell order (Ask) at 100 for a quantity of 10
        let sell_order = OrderBuilder::new()
            .id(1)
            .side(Side::Sell)
            .price(100)
            .quantity(10)
            .build()
            .unwrap();

        let trades = book.add_order(sell_order).unwrap();
        assert!(trades.is_empty()); // No match cuz not buyers in front
        assert_eq!(book.best_ask(), Some(100));

        // 2. We put a buy order (Buy) at 100 for a quantity of ten (Full matching)
        let buy_order = OrderBuilder::new()
            .id(2)
            .side(Side::Buy)
            .price(100)
            .quantity(10)
            .build()
            .unwrap();

        let trades = book.add_order(buy_order).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity.0, 10);
        assert_eq!(trades[0].price.0, 100);
        assert_eq!(trades[0].buyer_id.0, 2);
        assert_eq!(trades[0].seller_id.0, 1);

        // The order book must be empty again
        assert!(book.asks.is_empty());
        assert!(book.bids.is_empty());
        assert!(book.index.is_empty());
    }

    #[test]
    fn test_spread_calculation() {
        let mut book = LimitOrderBook::new();

        // Add a BId at 90
        let bid = OrderBuilder::new()
            .id(1)
            .side(Side::Buy)
            .price(90)
            .quantity(5)
            .build()
            .unwrap();
        book.add_order(bid).unwrap();

        // Add an Ask at 105
        let ask = OrderBuilder::new()
            .id(2)
            .side(Side::Sell)
            .price(105)
            .quantity(5)
            .build()
            .unwrap();
        book.add_order(ask).unwrap();

        assert_eq!(book.best_bid(), Some(90));
        assert_eq!(book.best_ask(), Some(105));
        // Spread = Beast Ask (105) - Best Bid (90) = 15
        assert_eq!(book.spread(), Some(15));
    }

    #[test]
    fn test_cancel_order() {
        let mut book = LimitOrderBook::new();

        let order = OrderBuilder::new()
            .id(42)
            .side(Side::Buy)
            .price(50)
            .quantity(20)
            .build()
            .unwrap();

        book.add_order(order.clone()).unwrap();

        assert_eq!(book.best_bid(), Some(50));
        assert!(!book.index.is_empty());

        // Cancel the order
        let canceled = book.cancel_order(order);
        assert!(canceled.is_some());
        assert!(book.bids.is_empty());
        assert!(book.index.is_empty());
        assert_eq!(book.best_bid(), None);
    }

    // --- 2. PARAMETRIC TESTS TESTS WITH RSTEST ---

    #[rstest]
    #[case(10, 10, 10)] // Qqual quantities
    #[case(15, 10, 10)] // Ask larger than the Buy (partial consumption of the ask)
    #[case(5, 10, 5)] // Buy order larger than the Ask (full consumption of the Ask, with the remainder placed in the Bid)
    fn test_buy_matching_scenarios(
        #[case] ask_qty: u64,
        #[case] buy_qty: u64,
        #[case] expected_traded_qty: u64,
    ) {
        let mut book = LimitOrderBook::new();

        let ask = OrderBuilder::new()
            .id(1)
            .side(Side::Sell)
            .price(100)
            .quantity(ask_qty)
            .build()
            .unwrap();
        book.add_order(ask).unwrap();

        let buy = OrderBuilder::new()
            .id(2)
            .side(Side::Buy)
            .price(100)
            .quantity(buy_qty)
            .build()
            .unwrap();
        let trades = book.add_order(buy).unwrap();

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity.0, expected_traded_qty);
    }

    // --- 3. PROPERTY TESTING WITH PROPTEST ---

    proptest! {
        // Property: An order book must never crash, regardless of the sequence of buys and sells.
        #[test]
        fn prop_order_book_never_panics(
            operations in prop::collection::vec(
                (1u64..1000, 50u64..150, 1u64..100, any::<bool>()),
                1..50
            )
        ) {
            let mut book = LimitOrderBook::new();

            for (id, price, quantity, is_buy) in operations {
                let side = if is_buy { Side::Buy } else { Side::Sell };
                let order = OrderBuilder::new()
                    .id(id)
                    .side(side)
                    .price(price)
                    .quantity(quantity)
                    .build();

                if let Ok(ord) = order {
                    let _ = book.add_order(ord);
                }
            }
        }

        // Property: The spread (when it exists) must always be positive or zero (Best Ask >= Best Bid)
        #[test]
        fn prop_spread_is_always_non_negative(
            bid_prices in prop::collection::vec(10u64..90, 1..20),
            ask_prices in prop::collection::vec(100u64..200, 1..20)
        ) {
            let mut book = LimitOrderBook::new();

            for (i, &p) in bid_prices.iter().enumerate() {
                let _ = book.add_order(OrderBuilder::new().id(i as u64).side(Side::Buy).price(p).quantity(10).build().unwrap());
            }

            for (i, &p) in ask_prices.iter().enumerate() {
                let _ = book.add_order(OrderBuilder::new().id((1000 + i) as u64).side(Side::Sell).price(p).quantity(10).build().unwrap());
            }

            if let Some(spread) = book.spread() {
                prop_assert!(spread >= 0);
            }
        }
    }
}
