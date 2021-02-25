//!
//! The time GET response.
//!

use serde::Deserialize;

///
/// The `https://www.binance.com/api/v3/time` GET response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The server time in milliseconds since Unix epoch.
    pub server_time: i64,
}
