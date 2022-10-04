use super::OrderBookItem;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub timestamp: SystemTime,
    pub bids: Vec<OrderBookItem>,
    pub asks: Vec<OrderBookItem>,
}
