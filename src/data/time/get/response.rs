//!
//! The time GET response.
//!

use serde_derive::Deserialize;

///
/// The `https://www.binance.com/api/v3/time` GET response.
///
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The server time in milliseconds since Unix epoch.
    pub server_time: i64,
}
