//!
//! The order POST request.
//!

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::data::order::{post::Type as ResponseType, Side, TimeInForce, Type};

pub struct Request {
    symbol: String,
    side: Side,
    r#type: Type,
    time_in_force: Option<TimeInForce>,
    quantity: Decimal,
    price: Option<Decimal>,
    new_client_order_id: Option<String>,
    stop_price: Option<Decimal>,
    iceberg_qty: Option<Decimal>,
    new_order_resp_type: Option<ResponseType>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl Request {
    const TIMESTAMP_OFFSET: i64 = 1000;

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
            timestamp: Utc::now().timestamp_millis() - Self::TIMESTAMP_OFFSET,
        }
    }

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
            timestamp: Utc::now().timestamp_millis() - Self::TIMESTAMP_OFFSET,
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(256);
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
