//!
//! The order time in force.
//!

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GoodTilCanceled,
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match self {
            TimeInForce::GoodTilCanceled => "GTC",
            TimeInForce::ImmediateOrCancel => "IOC",
            TimeInForce::FillOrKill => "FOK",
        }
        .to_owned()
    }
}
