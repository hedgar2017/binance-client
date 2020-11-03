//!
//! The trade event data, received via WebSocket.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

///
/// The trade event data, received via WebSocket.
///
#[derive(Debug, Deserialize, Clone)]
pub struct Trade {
    /// The trade event type. Usually equal to `trade`.
    #[serde(rename = "e")]
    pub event_type: String,
    /// The trade event time in milliseconds since Unix epoch.
    #[serde(rename = "E")]
    pub event_time: i64,
    /// The trading symbol name.
    #[serde(rename = "s")]
    pub symbol: String,
    /// The unique trade event ID.
    #[serde(rename = "t")]
    pub trade_id: i64,
    /// The trade event price.
    #[serde(rename = "p")]
    pub price: Decimal,
    /// The trade event quantity.
    #[serde(rename = "q")]
    pub quantity: Decimal,
    /// The order ID if the buying side.
    #[serde(rename = "b")]
    pub buyer_order_id: i64,
    /// The order ID if the selling side.
    #[serde(rename = "a")]
    pub seller_order_id: i64,
    /// The trade time in milliseconds since Unix epoch.
    #[serde(rename = "T")]
    pub trade_time: i64,
    /// If the buyer is the marker maker.
    #[serde(rename = "m")]
    pub is_market_maker: bool,
    /// Unknown value.
    #[serde(rename = "M")]
    pub ignore: bool,
}
