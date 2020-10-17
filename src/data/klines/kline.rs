//!
//! The kline.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

///
/// The kline data.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    /// The kline open time in milliseconds since Unix epoch.
    pub open_time: i64,
    /// The kline open price.
    pub open: Decimal,
    /// The kline high price.
    pub high: Decimal,
    /// The kline low price.
    pub low: Decimal,
    /// The kline close price.
    pub close: Decimal,
    /// The kline volume in secondary token.
    pub volume: Decimal,
    /// The kline open time in milliseconds since Unix epoch.
    pub close_time: i64,
    /// The kline volume in primary token.
    pub quote_asset_volume: Decimal,
    /// The number of trades executed within the kline.
    pub number_of_trades: i64,
    /// The taker buy volume in secondary token.
    pub taker_buy_base_asset_volume: Decimal,
    /// The taker buy volume in primary token.
    pub taker_buy_quote_asset_volume: Decimal,
    /// The unknown value.
    pub ignore: Decimal,
}

impl Kline {
    ///
    /// If the kline is green, that is, `close >= open`.
    ///
    pub fn is_green(&self) -> bool {
        self.close >= self.open
    }

    ///
    /// If the kline is red, that is, `close < open`.
    ///
    pub fn is_red(&self) -> bool {
        self.close < self.open
    }

    ///
    /// The average of low and high.
    ///
    pub fn middle(&self) -> Decimal {
        (self.low + self.high) / Decimal::new(2, 0)
    }

    ///
    /// The average of open and close.
    ///
    pub fn middle_body(&self) -> Decimal {
        (self.open + self.close) / Decimal::new(2, 0)
    }

    ///
    /// The average of open, high, low, and close.
    ///
    pub fn average(&self) -> Decimal {
        (self.low + self.open + self.close + self.high) / Decimal::new(4, 0)
    }
}
