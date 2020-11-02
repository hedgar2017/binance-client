//!
//! The order POST response full.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::data::order::post::response::fill::Fill;
use crate::data::order::r#type::Type;
use crate::data::order::side::Side;
use crate::data::order::status::Status;
use crate::data::order::time_in_force::TimeInForce;

///
/// The `https://www.binance.com/api/v3/order` POST full-type response.
///
/// Full response contains all the order data, which is usually returned for market orders.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Full {
    /// The symbol name.
    pub symbol: String,
    /// The server-side order ID.
    pub order_id: i64,
    /// The client-side order ID.
    pub client_order_id: String,
    /// The time when the order was acknowledged.
    pub transact_time: i64,
    /// The order price.
    pub price: Decimal,
    /// The initial order quantity.
    pub orig_qty: Decimal,
    /// The quantity executed so far. Usually equal to `orig_qty` for market orders.
    pub executed_qty: Decimal,
    /// Usually the same as `executed_qty`.
    pub cummulative_quote_qty: Decimal,
    /// The order status.
    pub status: Status,
    /// The order time-in-force.
    pub time_in_force: TimeInForce,
    /// The order type.
    pub r#type: Type,
    /// The order side.
    pub side: Side,
    /// The order partial fills.
    pub fills: Vec<Fill>,
}
