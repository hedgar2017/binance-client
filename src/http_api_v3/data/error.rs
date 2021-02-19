//!
//! The Binance error response.
//!

use serde::Deserialize;

///
/// The Binance error response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// The Binance error code.
    pub code: i64,
    /// The Binance error message.
    pub msg: String,
}
