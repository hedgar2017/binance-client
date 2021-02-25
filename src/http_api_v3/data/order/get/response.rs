//!
//! The order GET response.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::http_api_v3::data::order_side::OrderSide;
use crate::http_api_v3::data::order_status::OrderStatus;
use crate::http_api_v3::data::order_time_in_force::OrderTimeInForce;
use crate::http_api_v3::data::order_type::OrderType;

///
/// The `https://www.binance.com/api/v3/order` GET response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The symbol name.
    pub symbol: String,
    /// The server-side order ID.
    pub order_id: i64,
    /// The client-side order ID.
    pub client_order_id: String,
    /// The order price.
    pub price: Decimal,
    /// The initial order quantity.
    pub orig_qty: Decimal,
    /// The order quantity executed so far.
    pub executed_qty: Decimal,
    /// Usually the same as `executed_qty`.
    pub cummulative_quote_qty: Decimal,
    /// The order status.
    pub status: OrderStatus,
    /// The order time-in-force.
    pub time_in_force: OrderTimeInForce,
    /// The order type.
    pub r#type: OrderType,
    /// The order side.
    pub side: OrderSide,
    /// Used with `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders.
    pub stop_price: Decimal,
    /// The iceberg order quantity.
    pub iceberg_qty: Decimal,
    /// The order time in milliseconds.
    pub time: i64,
    /// Usually the same as `time`.
    pub update_time: i64,
    /// Unknown value.
    pub is_working: bool,
}
