//!
//! The order POST response fill.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    price: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    qty: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    commission: Decimal,
    commission_asset: String,
}
