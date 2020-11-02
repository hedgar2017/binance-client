//!
//! The order type.
//!

use serde::Deserialize;

///
/// The order time.
///
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
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

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Limit => "LIMIT",
            Type::Market => "MARKET",
            Type::StopLoss => "STOP_LOSS",
            Type::StopLossLimit => "STOP_LOSS_LIMIT",
            Type::TakeProfit => "TAKE_PROFIT",
            Type::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
            Type::LimitMaker => "LIMIT_MAKER",
            Type::Other => "OTHER",
        }
        .to_owned()
    }
}
