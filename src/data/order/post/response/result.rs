//!
//! The order POST response result.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

use crate::data::order::Side;
use crate::data::order::Status;
use crate::data::order::TimeInForce;
use crate::data::order::Type;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    symbol: String,
    order_id: i64,
    client_order_id: String,
    transact_time: i64,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    price: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    orig_qty: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    executed_qty: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    cummulative_quote_qty: Decimal,
    status: Status,
    time_in_force: TimeInForce,
    r#type: Type,
    side: Side,
}

impl Result {
    pub fn executed_quantity(&self) -> Decimal {
        self.executed_qty
    }

    pub fn client_order_id(&self) -> &str {
        self.client_order_id.as_str()
    }
}
