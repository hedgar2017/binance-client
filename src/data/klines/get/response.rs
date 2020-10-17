//!
//! The klines GET response.
//!

use crate::data::klines::kline::Kline;

///
/// The kline GET response, which is a simple array of klines.
///
pub type Response = Vec<Kline>;
