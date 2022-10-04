use super::{OrderBookItem, OrderSide};
use std::time::SystemTime;

#[derive(Debug, Clone, Default)]
pub struct OrderBookDelta {
    pub timestamp: Option<SystemTime>,
    pub bids: Vec<OrderBookItem>,
    pub asks: Vec<OrderBookItem>,
}

impl OrderBookDelta {
    pub fn is_empty(&self) -> bool {
        self.bids.len() == 0 && self.asks.len() == 0
    }

    pub fn add_item(&mut self, side: OrderSide, item: OrderBookItem, timestamp: SystemTime) {
        match side {
            OrderSide::Bid => self.bids.push(item),
            OrderSide::Ask => self.asks.push(item),
        }
        self.timestamp = Some(timestamp)
    }
}
