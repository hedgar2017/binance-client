//!
//! The order POST response ack.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ack {
    symbol: String,
    order_id: i64,
    client_order_id: String,
    transact_time: i64,
}

impl Ack {
    pub fn client_order_id(&self) -> &str {
        self.client_order_id.as_str()
    }
}
