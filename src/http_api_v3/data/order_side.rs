//!
//! The order side.
//!

use serde::Deserialize;

///
/// The order side.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    /// The buy order.
    Buy,
    /// The sell order.
    Sell,
}

impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
        .to_owned()
    }
}
