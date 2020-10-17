//!
//! The Binance WebSocket adapter.
//!

use failure::Fail;

///
/// The Binance WebSocket error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The connection error.
    #[fail(display = "connection: {}", _0)]
    Connection(websocket::WebSocketError),
}
