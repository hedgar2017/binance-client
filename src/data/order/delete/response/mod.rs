//!
//! The order DELETE response.
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
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: i64,
    pub client_order_id: String,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub price: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub orig_qty: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub executed_qty: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub cummulative_quote_qty: Decimal,
    pub status: Status,
    pub time_in_force: TimeInForce,
    pub r#type: Type,
    pub side: Side,
}
