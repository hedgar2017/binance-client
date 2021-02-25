//!
//! The products symbol status.
//!

use serde::Deserialize;

///
/// The status of a trading symbol.
///
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// Not available for trading.
    PreTrading,
    /// The only variant that should be treated as valid for trading.
    Trading,
    /// Not available for trading.
    PostTrading,
    /// Not available for trading.
    EndOfDay,
    /// Not available for trading.
    Halt,
    /// Not available for trading.
    AuctionMatch,
    /// Not available for trading.
    Break,
    /// Fallback for all other variants.
    #[serde(other)]
    Other,
}
