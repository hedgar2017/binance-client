//!
//! The depth data, received via WebSocket.
//!

use serde::Deserialize;

use crate::data::depth_element::DepthElement;

///
/// The depth data, received via WebSocket.
///
#[derive(Debug, Deserialize, Clone)]
pub struct Depth {
    /// The trade event type. Usually equal to `depthUpdate`.
    #[serde(rename = "e")]
    pub event_type: String,
    /// The trade event time in milliseconds since Unix epoch.
    #[serde(rename = "E")]
    pub event_time: i64,
    /// The trading symbol name.
    #[serde(rename = "s")]
    pub symbol: String,
    /// The first update ID.
    #[serde(rename = "U")]
    pub first_update_id: i64,
    /// The last update ID.
    #[serde(rename = "u")]
    pub last_update_id: i64,
    /// The bids below the current price.
    #[serde(rename = "b")]
    pub bids: Vec<DepthElement>,
    /// The asks above the current price.
    #[serde(rename = "a")]
    pub asks: Vec<DepthElement>,
}
