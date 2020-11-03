//!
//! A single depth element.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

///
/// A single depth element.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthElement {
    /// The depth level price.
    pub price: Decimal,
    /// The depth level quantity in secondary token.
    pub quantity: Decimal,
}
