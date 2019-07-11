//!
//! The order.
//!

mod delete;
mod get;
mod post;
mod side;
mod status;
mod time_in_force;
mod r#type;

pub use self::{
    delete::{Request as DeleteRequest, Response as DeleteResponse},
    get::{Request as GetRequest, Response as GetResponse},
    post::{
        Ack as PostResponseAck, Fill as PostResponseFill, Full as PostResponseFull,
        Request as PostRequest, Response as PostResponse, Result as PostResponseResult,
        Type as PostResponseType,
    },
    r#type::Type,
    side::Side,
    status::Status,
    time_in_force::TimeInForce,
};
