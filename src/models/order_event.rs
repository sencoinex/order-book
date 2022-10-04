use super::{Asset, AssetPair, OrderSide, Price, Quantity};
use std::time::SystemTime;

#[derive(Debug)]
pub enum OrderEvent<A: Asset> {
    Created {
        asset_pair: AssetPair<A>,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        timestamp: SystemTime,
    },
    Filled {
        asset_pair: AssetPair<A>,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        timestamp: SystemTime,
    },
    Amended {
        asset_pair: AssetPair<A>,
        side: OrderSide,
        from_price: Price,
        from_quantity: Quantity,
        to_price: Price,
        to_quantity: Quantity,
        timestamp: SystemTime,
    },
    Cancelled {
        asset_pair: AssetPair<A>,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        timestamp: SystemTime,
    },
}
