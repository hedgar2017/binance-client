//!
//! The trade.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: i64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "t")]
    trade_id: i64,
    #[serde(rename = "p")]
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    price: Decimal,
    #[serde(rename = "q")]
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    quantity: Decimal,
    #[serde(rename = "b")]
    buyer_order_id: i64,
    #[serde(rename = "a")]
    seller_order_id: i64,
    #[serde(rename = "T")]
    trade_time: i64,
    #[serde(rename = "m")]
    is_market_maker: bool,
    #[serde(rename = "M")]
    ignore: bool,
}

impl Trade {
    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}
