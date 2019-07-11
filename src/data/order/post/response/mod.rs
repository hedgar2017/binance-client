//!
//! The order POST response.
//!

mod ack;
mod fill;
mod full;
mod result;
mod r#type;

pub use self::{ack::Ack, fill::Fill, full::Full, r#type::Type, result::Result};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Full(Full),
    Result(Result),
    Ack(Ack),
}

impl Response {
    pub fn client_order_id(&self) -> &str {
        match self {
            Response::Full(response) => response.client_order_id(),
            Response::Result(response) => response.client_order_id(),
            Response::Ack(response) => response.client_order_id(),
        }
    }
}
