//!
//! The order POST response fill.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub price: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub qty: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub commission: Decimal,
    pub commission_asset: String,
}
