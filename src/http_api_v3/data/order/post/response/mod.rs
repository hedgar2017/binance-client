//!
//! The order POST response.
//!

pub mod ack;
pub mod fill;
pub mod full;
pub mod result;
pub mod r#type;

use self::ack::Ack;
use self::full::Full;
use self::result::Result;

use serde::Deserialize;

///
/// The `https://www.binance.com/api/v3/order` POST response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Response {
    /// The full-type response. See the inner element description.
    Full(Full),
    /// The result-type response. See the inner element description.
    Result(Result),
    /// The ack-type response. See the inner element description.
    Ack(Ack),
}

impl Response {
    ///
    /// Returns the client order ID from the inner types.
    ///
    pub fn client_order_id(&self) -> String {
        match self {
            Response::Full(inner) => inner.client_order_id.to_owned(),
            Response::Result(inner) => inner.client_order_id.to_owned(),
            Response::Ack(inner) => inner.client_order_id.to_owned(),
        }
    }
}
