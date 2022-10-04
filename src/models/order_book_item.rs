use super::{Price, Quantity};

#[derive(Debug, Clone)]
pub struct OrderBookItem {
    pub price: Price,
    pub quantity: Quantity,
}

impl OrderBookItem {
    pub fn new(price: Price, quantity: Quantity) -> Self {
        Self { price, quantity }
    }
}
