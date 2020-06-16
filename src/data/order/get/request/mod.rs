//!
//! The order GET request.
//!

use chrono::prelude::*;

pub struct Request {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

impl Request {
    const TIMESTAMP_OFFSET: i64 = 1000;

    pub fn new(symbol: &str, orig_client_order_id: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: Some(orig_client_order_id.to_owned()),
            recv_window: None,
            timestamp: Utc::now().timestamp_millis() - Self::TIMESTAMP_OFFSET,
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(256);
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
