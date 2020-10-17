//!
//! The Binance client library constants.
//!

/// The request timestamp offset, which is substituted from the request time to prevent
/// the Binance `request window missed` error.
pub const REQUEST_TIMESTAMP_OFFSET: i64 = 1000;
