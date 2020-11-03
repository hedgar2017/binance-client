//!
//! The klines GET response.
//!

use crate::data::klines::kline::Kline;

///
/// The `https://www.binance.com/api/v3/klines` GET response.
///
pub type Response = Vec<Kline>;
