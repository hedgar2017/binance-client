//!
//! The order POST response fill.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

///
/// The order partial fill.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    /// The trade event price.
    pub price: Decimal,
    /// The trade event quantity.
    pub qty: Decimal,
    /// The commission of the trade event.
    pub commission: Decimal,
    /// The token of the commission of the trade event.
    pub commission_asset: String,
}
