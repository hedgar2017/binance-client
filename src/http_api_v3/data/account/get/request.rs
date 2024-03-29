//!
//! The account GET request.
//!

use chrono::prelude::*;

///
/// The `https://www.binance.com/api/v3/order` GET request query.
///
pub struct Query {
    /// The allowed time window between the request and response in milliseconds.
    pub recv_window: Option<i64>,
    /// The request time in milliseconds.
    pub timestamp: i64,
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

impl Query {
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self {
            recv_window: None,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        if let Some(recv_window) = self.recv_window {
            params += &format!("&recvWindow={}", recv_window);
        }
        params += &format!("&timestamp={}", self.timestamp);
        params
    }
}
