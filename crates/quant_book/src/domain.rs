use std::cmp::Reverse;
use std::collections::{BTreeMap, VecDeque};
use crate::errors::BookError;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Price(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Quantity(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Id(pub u64);

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Id,
    pub price: Price,
    pub quantity: Quantity,
    pub side: Side,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            id: Id(0),
            price: Price(0),
            quantity: Quantity(0),
            side: Side::Buy,
        }
    }
}

pub struct OrderBuilder {
    pub id: Option<u64>,
    pub price: Option<u64>,
    pub quantity: Option<u64>,
    pub side: Option<Side>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            price: None,
            quantity: None,
            side: None,
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn price(mut self, price: u64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn quantity(mut self, quantity: u64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn build(self) -> Result<Order, BookError> {
        let mut order: Order = Order::default();
        
        if let Some(id) = self.id {
            order.id = Id(id);
        }
        else {
            return Err(BookError::IDMissing);
        }
        if let Some(price) = self.price {
            order.price = Price(price);
        } 
        else {
            return Err(BookError::PriceMissing);
        }
        if let Some(quantity) = self.quantity {
            order.quantity = Quantity(quantity);
        } 
        else {
            return Err(BookError::QuantityMissing);
        }
        if let Some(side) = self.side {
            order.side = side
        } 
        else {
            return Err(BookError::SideMissing);
        }

        Ok(order)
    }
}

pub struct Trade {
    pub buyer_id: Id,
    pub seller_id: Id,
    pub price: Price,
    pub quantity: Quantity,
}

pub struct TradeBuilder {
    pub buyer_id: Option<u64>,
    pub seller_id: Option<u64>,
    pub price: Option<u64>,
    pub quantity: Option<u64>,
}

impl TradeBuilder {
    pub fn new() -> Self {
        Self {
            buyer_id: None,
            seller_id: None,
            price: None,
            quantity: None,
        }
    }

    pub fn buyer_id(mut self, buyer_id: u64) -> Self {
        self.buyer_id = Some(buyer_id);
        self
    }

    pub fn seller_id(mut self, seller_id: u64) -> Self {
        self.seller_id = Some(seller_id);
        self
    }

    pub fn price(mut self, price: u64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn quantity(mut self, quantity: u64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn build(self) -> Result<Trade, BookError> {
        if self.buyer_id.is_none() {
            return Err(BookError::IDMissing);
        }
        if self.seller_id.is_none() {
            return Err(BookError::SellerIDMissing);
        }
        if self.buyer_id.is_none() {
            return Err(BookError::BuyerIDMissing);
        }
        if self.price.is_none() {
            return Err(BookError::PriceMissing);
        }
        if self.quantity.is_none() {
            return Err(BookError::QuantityMissing);
        }

        let buyer_id = Id(self.buyer_id.unwrap());
        let seller_id = Id(self.seller_id.unwrap());
        let price = Price(self.price.unwrap());
        let quantity = Quantity(self.quantity.unwrap());

        Ok(Trade {
            buyer_id,
            seller_id,
            price,
            quantity,
        })
    }
}

pub type Bids = BTreeMap<Reverse<u64>, VecDeque<Order>>;
pub type Asks = BTreeMap<u64, VecDeque<Order>>;
pub type Index = BTreeMap<u64, (Side, u64)>;
