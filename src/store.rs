use crate::models::{OrderBook, OrderBookDelta, OrderBookItem, OrderSide, Price};

pub trait OrderBookStore {
    type Error: std::error::Error;
    fn get(&mut self) -> Result<Option<OrderBook>, Self::Error>;
    fn update(&mut self, order_book: OrderBook) -> Result<(), Self::Error>;
    fn delete(&mut self) -> Result<(), Self::Error>;
    fn get_item(
        &mut self,
        side: OrderSide,
        price: &Price,
    ) -> Result<Option<OrderBookItem>, Self::Error>;
    fn load_items(
        &mut self,
        side: OrderSide,
        price: &Price,
        limit: usize,
    ) -> Result<Vec<OrderBookItem>, Self::Error>;
    fn apply_delta(&mut self, delta: OrderBookDelta) -> Result<(), Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}
