//!
//! The order type.
//!

use serde::Deserialize;

///
/// The order time.
///
#[derive(Debug, Deserialize, Clone, Copy)]
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

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            Self::Limit => "LIMIT",
            Self::Market => "MARKET",
            Self::StopLoss => "STOP_LOSS",
            Self::StopLossLimit => "STOP_LOSS_LIMIT",
            Self::TakeProfit => "TAKE_PROFIT",
            Self::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
            Self::LimitMaker => "LIMIT_MAKER",
            Self::Other => "OTHER",
        }
        .to_owned()
    }
}
