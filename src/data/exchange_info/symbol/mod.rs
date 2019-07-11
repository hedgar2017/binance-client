//!
//! The exchange info symbol.
//!

mod status;

pub use self::status::Status;

use serde_derive::Deserialize;

use crate::data::order::Type as OrderType;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: Status,
    pub base_asset: String,
    pub base_asset_precision: usize,
    pub quote_asset: String,
    pub quote_precision: usize,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
}

impl Symbol {
    pub fn is_trading(&self) -> bool {
        match self.status {
            Status::Trading => true,
            _ => false,
        }
    }
}
