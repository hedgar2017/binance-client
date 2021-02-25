//!
//! The products symbol.
//!

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::http_exchange_api_v2::data::status::Status;

///
/// The trading symbol data.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// The symbol name.
    #[serde(rename = "s")]
    pub symbol: String,
    /// The symbol status on the exchange.
    #[serde(rename = "st")]
    pub status: Status,
    /// The secondary token in the trading pair.
    #[serde(rename = "b")]
    pub base_asset: String,
    /// The primary token in the trading pair.
    #[serde(rename = "q")]
    pub quote_asset: String,
    /// The price tick size.
    #[serde(rename = "ts")]
    pub price_tick: Decimal,
    /// The quantity tick size.
    #[serde(rename = "i")]
    pub quantity_tick: Decimal,
    /// The market open timestamp, if the symbol is not being traded yet.
    #[serde(rename = "planToOpenMarketTime")]
    pub plan_to_open_market_time: Option<u64>,
}

impl Symbol {
    ///
    /// If the symbol is active and can be normally traded.
    ///
    pub fn is_trading(&self) -> bool {
        matches!(self.status, Status::Trading)
    }

    ///
    /// The number of fractional digits in the symbol price.
    ///
    /// E.g. `0.000256` has `6` fractional digits.
    /// E.g. `0.42` has `2` fractional digits.
    ///
    pub fn price_precision(&self) -> u32 {
        let mut scale = crate::r#const::PRECISION_DEFAULT;
        let mut tick_size_check = Decimal::new(1, scale);
        while self.price_tick > tick_size_check && scale > 0 {
            tick_size_check *= Decimal::new(10, 0);
            scale -= 1;
        }
        scale
    }

    ///
    /// The number of fractional digits in the symbol quantity.
    ///
    pub fn quantity_precision(&self) -> u32 {
        let mut scale = crate::r#const::PRECISION_DEFAULT;
        let mut step_size_check = Decimal::new(1, scale);
        while self.quantity_tick > step_size_check && scale > 0 {
            step_size_check *= Decimal::new(10, 0);
            scale -= 1;
        }
        scale
    }
}
