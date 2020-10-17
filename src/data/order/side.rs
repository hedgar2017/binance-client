//!
//! The order side.
//!

use serde_derive::Deserialize;

///
/// The order side.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    /// The buy order.
    Buy,
    /// The sell order.
    Sell,
}

impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
        .to_owned()
    }
}
