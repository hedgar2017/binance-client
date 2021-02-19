//!
//! The Binance Exchange API v2 HTTP response.
//!

use serde::Deserialize;

///
/// The Binance Exchange API v2 HTTP response.
///
#[derive(Debug, Deserialize, Clone)]
pub struct Response<T> {
    /// The response code.
    pub code: String,
    /// The response message. Usually `null`.
    pub message: Option<String>,
    /// The response message detail. Usually `null`.
    pub message_detail: Option<String>,
    /// The response data.
    pub data: T,
    /// The response status.
    pub success: bool,
}
