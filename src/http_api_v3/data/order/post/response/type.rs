//!
//! The order POST response type.
//!

use std::fmt::Formatter;

///
/// The `https://www.binance.com/api/v3/order` POST response type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    /// The ack-type. See the `ack` module.
    Ack,
    /// The result-type. See the `result` module.
    Result,
    /// The full-type. See the `full` module.
    Full,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ack => write!(f, "ACK"),
            Self::Result => write!(f, "RESULT"),
            Self::Full => write!(f, "FULL"),
        }
    }
}
