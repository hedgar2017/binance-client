//!
//! The exchange info GET response.
//!

use serde::Deserialize;

use crate::http_api_v3::data::exchange_info::symbol::Symbol;

///
/// The `https://www.binance.com/api/v3/exchangeInfo` GET response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The server timezone name.
    pub timezone: String,
    /// The server time in milliseconds since Unix epoch.
    pub server_time: i64,
    /// The trading symbol data.
    pub symbols: Vec<Symbol>,
}
