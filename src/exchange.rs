//!
//! The Binance exchange data.
//!

use serde_derive::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ExchangeInfo {
    pub timezone: String,
    pub serverTime: i64,
    pub symbols: Vec<Symbol>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub baseAsset: String,
    pub quoteAsset: String,
}
