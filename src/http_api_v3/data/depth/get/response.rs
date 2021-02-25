//!
//! The depth GET response.
//!

use serde::Deserialize;

use crate::http_api_v3::data::depth_element::DepthElement;

///
/// The `https://www.binance.com/api/v3/depth` GET response.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The last update ID.
    pub last_update_id: i64,
    /// The bids below the current price.
    pub bids: Vec<DepthElement>,
    /// The asks above the current price.
    pub asks: Vec<DepthElement>,
}
