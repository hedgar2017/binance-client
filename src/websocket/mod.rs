//!
//! The Binance WebSocket adapter.
//!

pub mod error;

use std::sync::mpsc;
use std::thread;

use websocket::client::ClientBuilder;
use websocket::ws::dataframe::DataFrame;
use websocket::OwnedMessage;

use crate::data::trade::Trade;

use self::error::Error;

///
/// The Binance WebSocket client.
///
pub struct Client {}

impl Client {
    ///
    /// Subscribes to a `symbol`-dedicated trade stream.
    ///
    /// Returns the receiving channel end.
    ///
    pub fn trade(symbol: String) -> Result<mpsc::Receiver<Trade>, Error> {
        let address = format!(
            "wss://stream.binance.com:9443/ws/{}@trade",
            symbol.to_ascii_lowercase()
        );
        let mut client = ClientBuilder::new(&address)
            .expect(crate::panic::WEBSOCKET_ADDRESS_VALID)
            .connect_secure(None)
            .map_err(Error::Connection)?;

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            let message = match client.recv_message() {
                Ok(message) => {
                    if message.is_ping() {
                        log::debug!("Received ping");
                        match client.send_message(&OwnedMessage::Pong(b"pong frame".to_vec())) {
                            Ok(()) => log::debug!("Sent pong"),
                            Err(error) => log::warn!("Pong sending error: {}", error),
                        }
                        continue;
                    }

                    message.take_payload()
                }
                Err(error) => {
                    log::error!("Websocket error: {}", error);
                    return;
                }
            };

            if message.is_empty() {
                continue;
            }

            match serde_json::from_slice::<Trade>(&message) {
                Ok(trade) => match tx.send(trade) {
                    Ok(()) => {}
                    Err(_) => break,
                },
                Err(error) => log::warn!("Parsing error: {} ({:?})", error, message),
            }
        });
        Ok(rx)
    }
}
