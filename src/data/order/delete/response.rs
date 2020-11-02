//!
//! The order DELETE response.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::data::order::r#type::Type;
use crate::data::order::side::Side;
use crate::data::order::status::Status;
use crate::data::order::time_in_force::TimeInForce;

///
/// The `https://www.binance.com/api/v3/order` POST response.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The symbol name.
    pub symbol: String,
    /// Either `orderId` or `origClientOrderId` must be sent.
    pub orig_client_order_id: String,
    /// The cancelled order ID.
    pub order_id: i64,
    /// The client-side order ID.
    pub client_order_id: String,
    /// The order price.
    pub price: Decimal,
    /// The initial order quantity.
    pub orig_qty: Decimal,
    /// The order quantity executed before cancellation.
    pub executed_qty: Decimal,
    /// Usually the same as `executed_qty`.
    pub cummulative_quote_qty: Decimal,
    /// The order status, which must be `CANCELED`.
    pub status: Status,
    /// The order time-in-force.
    pub time_in_force: TimeInForce,
    /// The order type.
    pub r#type: Type,
    /// The order side.
    pub side: Side,
}
