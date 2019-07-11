//!
//! The time GET response.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub server_time: i64,
}
