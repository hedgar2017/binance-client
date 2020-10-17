//!
//! The Binance HTTP error.
//!

use failure::Fail;

use crate::data::error::Error as ResponseError;

///
/// The Binance HTTP client error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The request URL parsing error. Can happen on invalid user input.
    #[fail(display = "URL {} parsing: {}", _1, _0)]
    UrlParsing(reqwest::UrlError, String),
    /// The request building error. Can happen on invalid user input.
    #[fail(display = "request building: {}", _0)]
    RequestBuilding(reqwest::Error),
    /// The authorization keys data missing. The client was created without them.
    #[fail(display = "authorization keys missing. Please, add create a client with keys")]
    AuthorizationKeysMissing,
    /// The request execution error. Usually happens due to some network errors.
    #[fail(display = "request execution: {}", _0)]
    RequestExecution(reqwest::Error),
    /// The response parsing error. Binance returned invalid data or the data model must be updated.
    #[fail(display = "response parsing: {}", _0)]
    ResponseParsing(reqwest::Error),
    /// The response is valid, but Binance returned an application-level error.
    #[fail(display = "response error: {:?}", _0)]
    ResponseError(ResponseError),
}
