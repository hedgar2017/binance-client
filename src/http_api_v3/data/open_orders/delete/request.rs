//!
//! The open orders DELETE request.
//!

use chrono::prelude::*;

///
/// The `https://www.binance.com/api/v3/openOrders` GET request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
    /// The allowed time window between the request and response in milliseconds.
    pub recv_window: Option<i64>,
    /// The request time in milliseconds.
    pub timestamp: i64,
}

impl Query {
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            recv_window: None,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("symbol={}", self.symbol);
        if let Some(recv_window) = self.recv_window {
            params += &format!("&recvWindow={}", recv_window);
        }
        params += &format!("&timestamp={}", self.timestamp);
        params
    }
}
