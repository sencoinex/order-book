use crate::models::{Asset, OrderEvent};

#[derive(Debug)]
pub enum OrderBookGeneratorInput<A: Asset> {
    /// order event from matching engine
    OrderEvents(Vec<OrderEvent<A>>),
    Terminate,
}
