use std::cmp::Reverse;
use std::collections::BTreeMap;
use rayon::prelude::*;

use crate::domain::{Bids, Asks, Index, Order, Side, Trade, TradeBuilder};
use crate::errors::BookError;

use tracing::{info, debug, error};

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

                let orders = self.asks
                    .get_mut(&order.price.0)
                    .ok_or(BookError::AsksEmpty)?;

                if orders.is_empty() {
                    return Err(BookError::AsksEmpty);
                }


                while let Some(ask) = orders.front_mut() {
                    if order.quantity.0 == 0 {
                        break;
                    }

                    if ask.quantity.0 == order.quantity.0 {
                        let q = ask.quantity.0;
                        let ask_id = ask.id.0;

                        orders.pop_front();

                        let trade = TradeBuilder::new()
                            .seller_id(ask_id)
                            .buyer_id(order.id.0)
                            .price(order.price.0)
                            .quantity(q)
                            .build()?;

                        trades.push(trade);
                        order.quantity.0 = 0;
                        break;
                    }
                    else if ask.quantity.0 > order.quantity.0 {
                        let matched_qty = order.quantity.0;
                        ask.quantity.0 -= matched_qty;

                        let trade = TradeBuilder::new()
                            .seller_id(ask.id.0)
                            .buyer_id(order.id.0)
                            .price(order.price.0)
                            .quantity(matched_qty)
                            .build()?;

                        trades.push(trade);

                        order.quantity.0 = 0;
                        break;
                    }
                    else {
                        let q = ask.quantity.0;
                        let ask_id = ask.id.0;

                        orders.pop_front();
                        let trade = TradeBuilder::new()
                            .seller_id(ask_id)
                            .buyer_id(order.id.0)
                            .price(order.price.0)
                            .quantity(q)
                            .build()?;

                        trades.push(trade);
                        order.quantity.0 -= q;
                    }
                }

                if orders.is_empty() {
                    self.asks.remove(&order.price.0);
                }

                if order.quantity.0 > 0 {
                    let bids_queue = self.bids
                        .entry(Reverse(order.price.0))
                        .or_default();

                    bids_queue.push_back(order.clone());
                    self.index
                        .insert(order.id.0, (Side::Buy, order.price.0));
                }

                Ok(trades)
            }
            Side::Sell => {
                let mut trades: Vec<Trade> = vec![];

                let orders = self.asks
                    .get_mut(&order.price.0)
                    .ok_or(BookError::BidsEmpty)?;

                if orders.is_empty() {
                    return Err(BookError::BidsEmpty);
                }

                while let Some(bid) = orders.front_mut() {
                    if order.quantity.0 == 0 {
                        break;
                    }

                    if bid.quantity.0 == order.quantity.0 {
                        let q = bid.quantity.0;
                        let bid_id = bid.id.0;

                        orders.pop_front();

                        let trade = TradeBuilder::new()
                        .seller_id(order.id.0)
                        .buyer_id(bid_id)
                        .price(order.price.0)
                        .quantity(q)
                        .build()?;

                        trades.push(trade);

                        order.quantity.0 = 0;
                        break;
                    }
                    else if bid.quantity.0 > order.quantity.0 {
                        let matched_qty = order.quantity.0;
                        bid.quantity.0 -= matched_qty;

                        let trade = TradeBuilder::new()
                        .seller_id(order.id.0)
                        .buyer_id(bid.id.0)
                        .price(order.price.0)
                        .quantity(matched_qty)
                        .build()?;

                        trades.push(trade);

                        order.quantity.0 = 0;
                        break;
                    }
                    else {
                        let q = bid.quantity.0;
                        let bid_id = bid.id.0;

                        orders.pop_front();

                        let trade = TradeBuilder::new()
                        .seller_id(order.id.0)
                        .buyer_id(bid_id)
                        .price(q)
                        .quantity(q)
                        .build()?;

                        trades.push(trade);
                        order.quantity.0 -= q;
                    }
                }

                if orders.is_empty() {
                    self.bids.remove(&Reverse(order.price.0));
                }

                if order.quantity.0 > 0 {
                    let asks_queue = self.asks
                        .entry(order.price.0)
                        .or_default();
                    asks_queue.push_back(order.clone());
                    self.index.insert(order.id.0, (Side::Sell, order.price.0));
                }

                Ok(trades)
            },
        }
    }

    pub fn cancel_order(&mut self, order: Order) -> Option<Order> {
        unimplemented!()
    }

    pub fn best_did(&self) -> Option<Order> {
        unimplemented!()
    }

    pub fn best_ask(&self) -> Option<Order> {
        unimplemented!()
    }

    pub fn modify_order(&mut self, order_id: u64, new_quantity: u64) -> Result<(), &'static str> {
        unimplemented!()
    }
}

impl Default for LimitOrderBook {
    fn default() -> Self {
        Self::new()
    }
}