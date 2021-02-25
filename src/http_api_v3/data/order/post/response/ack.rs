//!
//! The order POST response ack.
//!

use serde::Deserialize;

///
/// The `https://www.binance.com/api/v3/order` POST ack-type response.
///
/// Ack response does not contain any order data, but only the acknowledgement of being accepted.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ack {
    /// The symbol name.
    pub symbol: String,
    /// The server-side order ID.
    pub order_id: i64,
    /// The client-side order ID.
    pub client_order_id: String,
    /// The time when the order was acknowledged.
    pub transact_time: i64,
}
