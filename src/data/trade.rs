//!
//! The trade.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "p")]
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub price: Decimal,
    #[serde(rename = "q")]
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub quantity: Decimal,
    #[serde(rename = "b")]
    pub buyer_order_id: i64,
    #[serde(rename = "a")]
    pub seller_order_id: i64,
    #[serde(rename = "T")]
    pub trade_time: i64,
    #[serde(rename = "m")]
    pub is_market_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
}
