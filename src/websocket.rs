//!
//! The WebSocket adapter.
//!

use std::sync::mpsc;
use std::thread;

use failure::Fail;
use websocket::client::ClientBuilder;
use websocket::ws::dataframe::DataFrame;
use websocket::OwnedMessage;

use crate::data::Trade;

pub struct Client {}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Connection: {}", _0)]
    Connection(websocket::WebSocketError),
}

impl Client {
    pub fn trade(symbol: String) -> Result<mpsc::Receiver<Trade>, Error> {
        let address = format!(
            "wss://stream.binance.com:9443/ws/{}@trade",
            symbol.to_ascii_lowercase()
        );
        let mut client = ClientBuilder::new(&address)
            .expect("Websocket address parsing bug")
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
