//!
//! The WebSocket adapter.
//!

use std::{sync::mpsc, thread};

use failure::Fail;
use log::*;
use websocket::{client::ClientBuilder, ws::dataframe::DataFrame, OwnedMessage};

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
                        debug!("Received ping");
                        match client.send_message(&OwnedMessage::Pong(b"pong frame".to_vec())) {
                            Ok(()) => debug!("Sent pong"),
                            Err(error) => warn!("Pong sending error: {}", error),
                        }
                        continue;
                    }
                    message.take_payload()
                }
                Err(error) => {
                    error!("Websocket error: {}", error);
                    return;
                }
            };

            if message.is_empty() {
                continue;
            }

            match serde_json::from_slice::<Trade>(&message) {
                Ok(trade) => {
                    let _ = tx
                        .send(trade)
                        .map_err(|error| warn!("Message sending error: {}", error));
                }
                Err(error) => warn!("Parsing error: {} ({:?})", error, message),
            }
        });
        Ok(rx)
    }
}
