//!
//! The Binance kline.
//!

use serde_derive::Deserialize;
use ta::{Close, High, Low, Open, Volume};

#[derive(Debug)]
pub struct Kline {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
    pub ignore: f64,
}

impl Kline {
    pub fn is_green(&self) -> bool {
        self.close > self.open
    }

    pub fn is_red(&self) -> bool {
        self.close < self.open
    }

    pub fn middle(&self) -> f64 {
        (self.low + self.high) / 2.0
    }

    pub fn average(&self) -> f64 {
        (self.low + self.open + self.close + self.high) / 4.0
    }
}

impl Open for Kline {
    fn open(&self) -> f64 {
        self.open
    }
}

impl High for Kline {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for Kline {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Close for Kline {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Volume for Kline {
    fn volume(&self) -> f64 {
        self.volume
    }
}

impl From<KlineInner> for Kline {
    fn from(inner: KlineInner) -> Self {
        Self {
            open_time: inner.open_time,
            open: inner.open.parse().expect("Parsing bug"),
            high: inner.high.parse().expect("Parsing bug"),
            low: inner.low.parse().expect("Parsing bug"),
            close: inner.close.parse().expect("Parsing bug"),
            volume: inner.volume.parse().expect("Parsing bug"),
            close_time: inner.close_time,
            quote_asset_volume: inner.quote_asset_volume.parse().expect("Parsing bug"),
            number_of_trades: inner.number_of_trades,
            taker_buy_base_asset_volume: inner
                .taker_buy_base_asset_volume
                .parse()
                .expect("Parsing bug"),
            taker_buy_quote_asset_volume: inner
                .taker_buy_quote_asset_volume
                .parse()
                .expect("Parsing bug"),
            ignore: inner.ignore.parse().expect("Parsing bug"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct KlineInner {
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub ignore: String,
}
