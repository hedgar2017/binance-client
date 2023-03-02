//!
//! The order POST request.
//!

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::http_api_v3::data::order::post::response::r#type::Type as ResponseType;
use crate::http_api_v3::data::order_side::OrderSide;
use crate::http_api_v3::data::order_time_in_force::OrderTimeInForce;
use crate::http_api_v3::data::order_type::OrderType;

///
/// The `https://www.binance.com/api/v3/order` POST request query.
///
pub struct Query {
    /// The symbol name.
    pub symbol: String,
    /// The order side.
    pub side: OrderSide,
    /// The order type.
    pub r#type: OrderType,
    /// The order time-in-force.
    pub time_in_force: Option<OrderTimeInForce>,
    /// The order quantity in the secondary asset.
    pub quantity: Option<Decimal>,
    /// The order quantity in the primary asset.
    pub quote_order_qty: Option<Decimal>,
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
    /// The query params default capacity.
    const QUERY_INITIAL_CAPACITY: usize = 255;

    ///
    /// Creates a market order request.
    ///
    pub fn market(
        symbol: &str,
        side: OrderSide,
        quantity: Decimal,
        use_base_quantity: bool,
    ) -> Self {
        let (quantity, quote_order_qty) = if use_base_quantity {
            (None, Some(quantity))
        } else {
            (Some(quantity), None)
        };

        Self {
            symbol: symbol.to_owned(),
            side,
            r#type: OrderType::Market,
            time_in_force: None,
            quantity,
            quote_order_qty,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            iceberg_qty: None,
            new_order_resp_type: Some(ResponseType::Full),
            recv_window: None,
            timestamp: Utc::now().timestamp_millis(),
        }
    }

    ///
    /// Creates a limit order request.
    ///
    pub fn limit(symbol: &str, side: OrderSide, quantity: Decimal, price: Decimal) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type: OrderType::Limit,
            time_in_force: Some(OrderTimeInForce::GoodTilCanceled),
            quantity: Some(quantity),
            quote_order_qty: None,
            price: Some(price),
            new_client_order_id: None,
            stop_price: None,
            iceberg_qty: None,
            new_order_resp_type: Some(ResponseType::Ack),
            recv_window: None,
            timestamp: Utc::now().timestamp_millis(),
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut params = String::with_capacity(Self::QUERY_INITIAL_CAPACITY);
        params += &format!("symbol={}", self.symbol.to_owned());
        params += &format!("&side={}", self.side);
        params += &format!("&type={}", self.r#type);
        if let Some(time_in_force) = self.time_in_force {
            params += &format!("&timeInForce={}", time_in_force);
        }
        if let Some(quantity) = self.quantity {
            params += &format!("&quantity={}", quantity);
        }
        if let Some(quote_order_qty) = self.quote_order_qty {
            params += &format!("&quoteOrderQty={}", quote_order_qty);
        }
        if let Some(price) = self.price {
            params += &format!("&price={}", price);
        }
        if let Some(ref new_client_order_id) = self.new_client_order_id {
            params += &format!("&newClientOrderId={}", new_client_order_id.to_owned());
        }
        if let Some(stop_price) = self.stop_price {
            params += &format!("&stopPrice={}", stop_price);
        }
        if let Some(iceberg_qty) = self.iceberg_qty {
            params += &format!("&icebergQty={}", iceberg_qty);
        }
        if let Some(new_order_resp_type) = self.new_order_resp_type {
            params += &format!("&newOrderRespType={}", new_order_resp_type);
        }
        if let Some(recv_window) = self.recv_window {
            params += &format!("&recvWindow={}", recv_window);
        }
        params += &format!("&timestamp={}", self.timestamp);
        params
    }
}
