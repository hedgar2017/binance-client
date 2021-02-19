//!
//! The Binance API v3 HTTP response.
//!

use serde::Deserialize;

use crate::http_api_v3::data::error::Error as ResponseError;

///
/// The Binance API v3 HTTP response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Response<T> {
    /// The successful response.
    Ok(T),
    /// The error response.
    Error(ResponseError),
}
