//!
//! The Binance HTTP response.
//!

use serde::Deserialize;

use crate::data::error::Error as ResponseError;

///
/// The binance HTTP response wrapper.
///
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    /// The successful response.
    Ok(T),
    /// The error response.
    Error(ResponseError),
}
