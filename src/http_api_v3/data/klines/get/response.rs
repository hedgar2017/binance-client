//!
//! The klines GET response.
//!

use crate::http_api_v3::data::klines::kline::Kline;

///
/// The `https://www.binance.com/api/v3/klines` GET response.
///
pub type Response = Vec<Kline>;
