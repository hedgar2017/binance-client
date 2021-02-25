//!
//! The account balance.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

///
/// The account balance.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// The token name.
    pub asset: String,
    /// The free balance amount, which can be used in trades.
    pub free: Decimal,
    /// The locked balance amount, which is unavailable at the moment.
    pub locked: Decimal,
}
