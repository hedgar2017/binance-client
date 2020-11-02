//!
//! The exchange info symbol permission.
//!

use serde::Deserialize;

///
/// The trading method allowed for the symbol.
///
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    /// The spot trading.
    Spot,
    /// The margin trading.
    Margin,
    /// Fallback for all other variants.
    #[serde(other)]
    Other,
}
