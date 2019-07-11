//!
//! The data structures.
//!

mod error;
mod exchange_info;
mod interval;
mod klines;
mod order;
mod precision;
mod serde;
mod time;
mod trade;

pub use self::{
    error::Error as ResponseError,
    exchange_info::{
        GetResponse as ExchangeInfoGetResponse, Symbol as ExchangeInfoSymbol,
        SymbolStatus as ExchangeInfoSymbolStatus,
    },
    interval::Interval,
    klines::{GetRequest as KlinesGetRequest, GetResponse as KlinesGetResponse, Kline},
    order::{
        DeleteRequest as OrderDeleteRequest, DeleteResponse as OrderDeleteResponse,
        GetRequest as OrderGetRequest, GetResponse as OrderGetResponse,
        PostRequest as OrderPostRequest, PostResponse as OrderPostResponse, Side as OrderSide,
        Status as OrderStatus, Type as OrderType,
    },
    precision::Precision,
    time::GetResponse as TimeGetResponse,
    trade::Trade,
};
