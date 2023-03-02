//!
//! The Binance error.
//!

use thiserror::Error;

use crate::http_api_v3::data::error::Error as ResponseError;

///
/// The Binance error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The request URL parsing error. Can happen on invalid user input.
    #[error("URL {0} parsing: {1}")]
    UrlParsing(reqwest::UrlError, String),
    /// The request building error. Can happen on invalid user input.
    #[error("request building: {0}")]
    RequestBuilding(reqwest::Error),
    /// The authorization keys data missing. The client was created without them.
    #[error("authorization keys missing. Please, add create a client with keys")]
    AuthorizationKeysMissing,
    /// The request execution error. Usually happens due to some network errors.
    #[error("request execution: {0}")]
    RequestExecution(reqwest::Error),
    /// The response reading error.
    #[error("response reading: {0}")]
    ResponseReading(reqwest::Error),
    /// The response parsing error. Binance returned invalid data or the data model must be updated.
    #[error("response parsing: {0} ({1})")]
    ResponseParsing(serde_json::Error, String),
    /// The response is valid, but Binance returned an application-level error.
    #[error("response error: {0:?}")]
    ResponseError(ResponseError),
    /// The WebSocket error.
    #[error("WebSocket: {0}")]
    WebSocket(websocket::WebSocketError),
}
