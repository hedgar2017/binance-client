//!
//! The order GET request.
//!

use chrono::prelude::*;

///
/// The `https://www.binance.com/api/v3/order` GET request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
    /// The order ID to get.
    pub order_id: Option<i64>,
    /// Either `orderId` or `origClientOrderId` must be sent.
    pub orig_client_order_id: Option<String>,
    /// The allowed time window between the request and response in milliseconds.
    pub recv_window: Option<i64>,
    /// The request time in milliseconds.
    pub timestamp: i64,
}

impl Query {
    /// The URL GET query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 256;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(symbol: &str, orig_client_order_id: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: Some(orig_client_order_id.to_owned()),
            recv_window: None,
            timestamp: Utc::now().timestamp_millis() - crate::r#const::REQUEST_TIMESTAMP_OFFSET,
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("symbol={}", self.symbol.to_owned());
        if let Some(order_id) = self.order_id {
            params += &format!("&orderId={}", order_id.to_string());
        }
        if let Some(ref orig_client_order_id) = self.orig_client_order_id {
            params += &format!("&origClientOrderId={}", orig_client_order_id.to_owned());
        }
        if let Some(recv_window) = self.recv_window {
            params += &format!("&recvWindow={}", recv_window.to_string());
        }
        params += &format!("&timestamp={}", self.timestamp.to_string());
        params
    }
}
