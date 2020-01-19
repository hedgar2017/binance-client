//!
//! The order GET response.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

use crate::data::order::Side;
use crate::data::order::Status;
use crate::data::order::TimeInForce;
use crate::data::order::Type;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    symbol: String,
    order_id: i64,
    client_order_id: String,
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
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    stop_price: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    iceberg_qty: Decimal,
    time: i64,
    update_time: i64,
    is_working: bool,
}

impl Response {
    pub fn original_quantity(&self) -> Decimal {
        self.orig_qty
    }

    pub fn executed_quantity(&self) -> Decimal {
        self.executed_qty
    }

    pub fn status(&self) -> Status {
        self.status
    }
}
