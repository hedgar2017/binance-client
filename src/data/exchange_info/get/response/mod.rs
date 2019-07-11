//!
//! The exchange info GET response.
//!

use serde_derive::Deserialize;

use crate::data::exchange_info::Symbol;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub timezone: String,
    pub server_time: i64,
    pub symbols: Vec<Symbol>,
}
