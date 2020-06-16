//!
//! The order POST response ack.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ack {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
}
