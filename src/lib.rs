//!
//! The Binance client.
//!

mod client;
mod error;
mod exchange;
mod interval;
mod kline;

pub use self::{
    client::Client,
    error::Error,
    exchange::{ExchangeInfo, Symbol},
    interval::Interval,
    kline::Kline,
};
