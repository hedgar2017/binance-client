//!
//! The order side.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
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
