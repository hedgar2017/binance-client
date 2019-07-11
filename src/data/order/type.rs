//!
//! The order type.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
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
        }
        .to_owned()
    }
}
