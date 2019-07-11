//!
//! The kline.
//!

use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub open_time: i64,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub open: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub high: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub low: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub close: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub volume: Decimal,
    pub close_time: i64,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub quote_asset_volume: Decimal,
    pub number_of_trades: i64,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub taker_buy_base_asset_volume: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub taker_buy_quote_asset_volume: Decimal,
    #[serde(deserialize_with = "crate::data::serde::deserialize_decimal")]
    pub ignore: Decimal,
}

impl Kline {
    pub fn is_green(&self) -> bool {
        self.close >= self.open
    }

    pub fn is_red(&self) -> bool {
        self.close < self.open
    }

    pub fn middle(&self) -> Decimal {
        (self.low + self.high) / Decimal::new(2, 0)
    }

    pub fn middle_body(&self) -> Decimal {
        (self.open + self.close) / Decimal::new(2, 0)
    }

    pub fn average(&self) -> Decimal {
        (self.low + self.open + self.close + self.high) / Decimal::new(4, 0)
    }
}
