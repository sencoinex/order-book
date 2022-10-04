mod input;
pub use input::OrderBookGeneratorInput;

use crate::models::{
    Asset, AssetPair, OrderBookDelta, OrderBookItem, OrderEvent, OrderSide, Price, Quantity,
};
use crate::store::OrderBookStore;
use std::sync::mpsc;
use std::time::SystemTime;

pub struct OrderBookGenerator<A: Asset, S: OrderBookStore> {
    asset_pair: AssetPair<A>,
    /// max number of elements inside bids/asks of order-book
    max_depth: usize,
    store: S,
    input_receiver: mpsc::Receiver<OrderBookGeneratorInput<A>>,
}

impl<A: Asset, S: OrderBookStore> OrderBookGenerator<A, S> {
    pub fn new(
        asset_pair: AssetPair<A>,
        max_depth: usize,
        store: S,
        input_receiver: mpsc::Receiver<OrderBookGeneratorInput<A>>,
    ) -> Self {
        Self {
            asset_pair,
            max_depth,
            store,
            input_receiver,
        }
    }

    pub fn start(mut self) -> Result<(), S::Error> {
        let order_book = self.store.get()?;
        let mut store = self.store;
        for input in self.input_receiver {
            match input {
                OrderBookGeneratorInput::OrderEvents(events) => {
                    for event in events {
                        let mut delta = OrderBookDelta::default();
                        match event {
                            OrderEvent::Created {
                                asset_pair,
                                side,
                                price,
                                quantity,
                                timestamp,
                            } => {
                                assert_eq!(asset_pair, self.asset_pair);
                                Self::make_add_delta(
                                    &mut store, &mut delta, side, price, quantity, timestamp,
                                )?;
                            }
                            OrderEvent::Filled {
                                asset_pair,
                                side,
                                price,
                                quantity,
                                timestamp,
                            } => {
                                assert_eq!(asset_pair, self.asset_pair);
                                Self::make_sub_delta(
                                    &mut store, &mut delta, side, price, quantity, timestamp,
                                )?;
                            }
                            OrderEvent::Amended {
                                asset_pair,
                                side,
                                from_price,
                                from_quantity,
                                to_price,
                                to_quantity,
                                timestamp,
                            } => {
                                assert_eq!(asset_pair, self.asset_pair);
                                if from_price == to_price {
                                    if from_quantity == to_quantity {
                                        // skip
                                        continue;
                                    } else if from_quantity < to_quantity {
                                        let added_quantity = to_quantity - from_quantity;
                                        Self::make_add_delta(
                                            &mut store,
                                            &mut delta,
                                            side,
                                            from_price,
                                            added_quantity,
                                            timestamp,
                                        )?;
                                    } else {
                                        let subtracted_quantity = from_quantity - to_quantity;
                                        Self::make_sub_delta(
                                            &mut store,
                                            &mut delta,
                                            side,
                                            from_price,
                                            subtracted_quantity,
                                            timestamp,
                                        )?;
                                    }
                                } else {
                                    Self::make_sub_delta(
                                        &mut store,
                                        &mut delta,
                                        side,
                                        from_price,
                                        from_quantity,
                                        timestamp,
                                    )?;
                                    Self::make_add_delta(
                                        &mut store,
                                        &mut delta,
                                        side,
                                        to_price,
                                        to_quantity,
                                        timestamp,
                                    )?;
                                }
                            }
                            OrderEvent::Cancelled {
                                asset_pair,
                                side,
                                price,
                                quantity,
                                timestamp,
                            } => {
                                assert_eq!(asset_pair, self.asset_pair);
                                Self::make_sub_delta(
                                    &mut store, &mut delta, side, price, quantity, timestamp,
                                )?;
                            }
                        }
                        // TODO if there is deleted items / load more and create new deltas
                        if !delta.is_empty() {
                            store.apply_delta(delta)?;
                        }
                    }
                }
                OrderBookGeneratorInput::Terminate => {
                    if let Some(order_book) = order_book {
                        store.update(order_book)?;
                        store.flush()?;
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    fn make_add_delta(
        store: &mut S,
        delta: &mut OrderBookDelta,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        timestamp: SystemTime,
    ) -> Result<(), S::Error> {
        let item = store.get_item(side, &price)?;
        let update_item = if let Some(old) = item {
            OrderBookItem::new(price, old.quantity + quantity)
        } else {
            OrderBookItem::new(price, quantity)
        };
        delta.add_item(side, update_item, timestamp);
        Ok(())
    }

    fn make_sub_delta(
        store: &mut S,
        delta: &mut OrderBookDelta,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        timestamp: SystemTime,
    ) -> Result<(), S::Error> {
        let item = store.get_item(side, &price)?;
        let update_item = if let Some(old) = item {
            if old.quantity <= quantity {
                OrderBookItem::new(price, quantity.zero())
            } else {
                OrderBookItem::new(price, old.quantity - quantity)
            }
        } else {
            OrderBookItem::new(price, quantity.zero())
        };
        delta.add_item(side, update_item, timestamp);
        Ok(())
    }
}
