//!
//! The order time in force.
//!

use serde::Deserialize;
use std::fmt::Formatter;

///
/// The order time-in-force. See the below descriptions.
///
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderTimeInForce {
    /// The default time-in-force. Effective until the trade is executed or cancelled.
    #[serde(rename = "GTC")]
    GoodTilCanceled,
    /// Must be filled immediately or is cancelled.
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    /// Is cancelled if the entire order does not execute as soon as it becomes available.
    #[serde(rename = "FOK")]
    FillOrKill,
}

impl std::fmt::Display for OrderTimeInForce {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GoodTilCanceled => write!(f, "GTC"),
            Self::ImmediateOrCancel => write!(f, "IOC"),
            Self::FillOrKill => write!(f, "FOK"),
        }
    }
}
