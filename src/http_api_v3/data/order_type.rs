//!
//! The order type.
//!

use serde::Deserialize;
use std::fmt::Formatter;

///
/// The order time.
///
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    /// The limit order.
    Limit,
    /// The market order.
    Market,
    /// The stop-loss order. Unused for now.
    StopLoss,
    /// The stop-loss-limit order. Unused for now.
    StopLossLimit,
    /// The take-profit order. Unused for now.
    TakeProfit,
    /// The take-profit-limit order. Unused for now.
    TakeProfitLimit,
    /// The limit-maker order. Unused for now.
    LimitMaker,
    /// Fallback for all other variants.
    #[serde(other)]
    Other,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::StopLoss => write!(f, "STOP_LOSS"),
            Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
            Self::TakeProfit => write!(f, "TAKE_PROFIT"),
            Self::TakeProfitLimit => write!(f, "TAKE_PROFIT_LIMIT"),
            Self::LimitMaker => write!(f, "LIMIT_MAKER"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}
