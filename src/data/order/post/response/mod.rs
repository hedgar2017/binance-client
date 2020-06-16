//!
//! The order POST response.
//!

mod ack;
mod fill;
mod full;
mod result;
mod r#type;

pub use self::ack::Ack;
pub use self::fill::Fill;
pub use self::full::Full;
pub use self::r#type::Type;
pub use self::result::Result;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Full(Full),
    Result(Result),
    Ack(Ack),
}

impl Response {
    pub fn client_order_id(&self) -> String {
        match self {
            Response::Full(response) => response.client_order_id.to_owned(),
            Response::Result(response) => response.client_order_id.to_owned(),
            Response::Ack(response) => response.client_order_id.to_owned(),
        }
    }
}
