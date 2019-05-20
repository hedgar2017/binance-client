//!
//! The Binance errors.
//!

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "URL parsing: {}", _0)]
    UrlParsing(reqwest::UrlError),
    #[fail(display = "Request building: {}", _0)]
    RequestBuilding(reqwest::Error),
    #[fail(display = "Request execution: {}", _0)]
    RequestExecution(reqwest::Error),
    #[fail(display = "Request parsing: {}", _0)]
    RequestParsing(reqwest::Error),
}
