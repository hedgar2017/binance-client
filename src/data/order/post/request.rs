//!
//! The order POST request.
//!

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::data::order::post::response::r#type::Type as ResponseType;
use crate::data::order::r#type::Type;
use crate::data::order::side::Side;
use crate::data::order::time_in_force::TimeInForce;

///
/// The `https://www.binance.com/api/v3/order` POST request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
    /// The order side.
    pub side: Side,
    /// The order type.
    pub r#type: Type,
    /// The order time-in-force.
    pub time_in_force: Option<TimeInForce>,
    /// The order quantity.
    pub quantity: Decimal,
    /// The order price. Required for limit orders.
    pub price: Option<Decimal>,
    /// A unique id for the order. Automatically generated if not sent.
    pub new_client_order_id: Option<String>,
    /// Used with `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders.
    pub stop_price: Option<Decimal>,
    /// The iceberg order quantity.
    pub iceberg_qty: Option<Decimal>,
    /// Set the response JSON. ACK, RESULT, or FULL;
    /// MARKET and LIMIT order types default to FULL, all other orders default to ACK.
    pub new_order_resp_type: Option<ResponseType>,
    /// The allowed time window between the request and response in milliseconds.
    pub recv_window: Option<i64>,
    /// The request time in milliseconds.
    pub timestamp: i64,
}

impl Query {
    /// The URL GET query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 255;

    ///
    /// Creates a market order request.
    ///
    pub fn market(symbol: &str, side: Side, quantity: Decimal) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type: Type::Market,
            time_in_force: None,
            quantity,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            iceberg_qty: None,
            new_order_resp_type: Some(ResponseType::Full),
            recv_window: None,
            timestamp: Utc::now().timestamp_millis() - crate::r#const::REQUEST_TIMESTAMP_OFFSET,
        }
    }

    ///
    /// Creates a limit order request.
    ///
    pub fn limit(symbol: &str, side: Side, quantity: Decimal, price: Decimal) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type: Type::Limit,
            time_in_force: Some(TimeInForce::GoodTilCanceled),
            quantity,
            price: Some(price),
            new_client_order_id: None,
            stop_price: None,
            iceberg_qty: None,
            new_order_resp_type: Some(ResponseType::Ack),
            recv_window: None,
            timestamp: Utc::now().timestamp_millis() - crate::r#const::REQUEST_TIMESTAMP_OFFSET,
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("symbol={}", self.symbol.to_owned());
        params += &format!("&side={}", self.side.to_string());
        params += &format!("&type={}", self.r#type.to_string());
        if let Some(time_in_force) = self.time_in_force {
            params += &format!("&timeInForce={}", time_in_force.to_string());
        }
        params += &format!("&quantity={}", self.quantity.to_string());
        if let Some(price) = self.price {
            params += &format!("&price={}", price.to_string());
        }
        if let Some(ref new_client_order_id) = self.new_client_order_id {
            params += &format!("&newClientOrderId={}", new_client_order_id.to_owned());
        }
        if let Some(stop_price) = self.stop_price {
            params += &format!("&stopPrice={}", stop_price.to_string());
        }
        if let Some(iceberg_qty) = self.iceberg_qty {
            params += &format!("&icebergQty={}", iceberg_qty.to_string());
        }
        if let Some(new_order_resp_type) = self.new_order_resp_type {
            params += &format!("&newOrderRespType={}", new_order_resp_type.to_string());
        }
        if let Some(recv_window) = self.recv_window {
            params += &format!("&recvWindow={}", recv_window.to_string());
        }
        params += &format!("&timestamp={}", self.timestamp.to_string());
        params
    }
}
